use anyhow::Result;
use clap::Parser;
#[cfg(feature = "imap")]
use email::message::add::imap::AddImapMessage;
#[cfg(feature = "maildir")]
use email::message::add::maildir::AddMaildirMessage;
#[cfg(feature = "sendmail")]
use email::message::send::sendmail::SendMessageSendmail;
#[cfg(feature = "smtp")]
use email::message::send::smtp::SendMessageSmtp;
use log::info;
use mml::MmlCompilerBuilder;
use std::io::{self, BufRead, IsTerminal};

#[cfg(feature = "account-sync")]
use crate::cache::arg::disable::CacheDisableFlag;
#[allow(unused)]
use crate::{
    account::arg::name::AccountNameFlag,
    backend::{Backend, BackendKind},
    config::TomlConfig,
    email::template::arg::TemplateRawArg,
    printer::Printer,
};

/// Send a template.
///
/// This command allows you to send a template and save a copy to the
/// sent folder. The template is compiled into a MIME message before
/// being sent. If you want to send a raw message, use the message
/// send command instead.
#[derive(Debug, Parser)]
pub struct TemplateSendCommand {
    #[command(flatten)]
    pub template: TemplateRawArg,

    #[cfg(feature = "account-sync")]
    #[command(flatten)]
    pub cache: CacheDisableFlag,

    #[command(flatten)]
    pub account: AccountNameFlag,
}

impl TemplateSendCommand {
    pub async fn execute(self, printer: &mut impl Printer, config: &TomlConfig) -> Result<()> {
        info!("executing send template command");

        let (toml_account_config, account_config) = config.clone().into_account_configs(
            self.account.name.as_deref(),
            #[cfg(feature = "account-sync")]
            self.cache.disable,
        )?;

        let send_message_kind = toml_account_config.send_message_kind();

        #[cfg(feature = "message-add")]
        let add_message_kind = toml_account_config
            .add_message_kind()
            .filter(|_| account_config.should_save_copy_sent_message());
        #[cfg(not(feature = "message-add"))]
        let add_message_kind = None;

        let backend = Backend::new(
            &toml_account_config,
            &account_config,
            send_message_kind.into_iter().chain(add_message_kind),
            |#[allow(unused)] builder| {
                match add_message_kind {
                    #[cfg(feature = "imap")]
                    Some(BackendKind::Imap) => {
                        builder
                            .set_add_message(|ctx| ctx.imap.as_ref().and_then(AddImapMessage::new));
                    }
                    #[cfg(feature = "maildir")]
                    Some(BackendKind::Maildir) => {
                        builder.set_add_message(|ctx| {
                            ctx.maildir.as_ref().and_then(AddMaildirMessage::new)
                        });
                    }
                    #[cfg(feature = "account-sync")]
                    Some(BackendKind::MaildirForSync) => {
                        builder.set_add_message(|ctx| {
                            ctx.maildir_for_sync
                                .as_ref()
                                .and_then(AddMaildirMessage::new)
                        });
                    }
                    _ => (),
                };
                match send_message_kind {
                    #[cfg(feature = "smtp")]
                    Some(BackendKind::Smtp) => {
                        builder.set_send_message(|ctx| {
                            ctx.smtp.as_ref().and_then(SendMessageSmtp::new)
                        });
                    }
                    #[cfg(feature = "sendmail")]
                    Some(BackendKind::Sendmail) => {
                        builder.set_send_message(|ctx| {
                            ctx.sendmail.as_ref().and_then(SendMessageSendmail::new)
                        });
                    }
                    _ => (),
                };
            },
        )
        .await?;

        let tpl = if io::stdin().is_terminal() {
            self.template.raw()
        } else {
            io::stdin()
                .lock()
                .lines()
                .map_while(Result::ok)
                .collect::<Vec<_>>()
                .join("\n")
        };

        #[allow(unused_mut)]
        let mut compiler = MmlCompilerBuilder::new();

        #[cfg(feature = "pgp")]
        compiler.set_some_pgp(account_config.pgp.clone());

        let msg = compiler.build(tpl.as_str())?.compile().await?.into_vec()?;

        backend.send_message(&msg).await?;

        printer.print("Message successfully sent!")
    }
}
