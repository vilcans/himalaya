use anyhow::Result;
use clap::Parser;
#[cfg(feature = "imap")]
use email::message::add::imap::AddImapMessage;
#[cfg(feature = "maildir")]
use email::message::add::maildir::AddMaildirMessage;
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
    folder::arg::name::FolderNameOptionalFlag,
    printer::Printer,
};

/// Save a template to a folder.
///
/// This command allows you to save a template to the given
/// folder. The template is compiled into a MIME message before being
/// saved to the folder. If you want to save a raw message, use the
/// message save command instead.
#[derive(Debug, Parser)]
pub struct TemplateSaveCommand {
    #[command(flatten)]
    pub folder: FolderNameOptionalFlag,

    #[command(flatten)]
    pub template: TemplateRawArg,

    #[cfg(feature = "account-sync")]
    #[command(flatten)]
    pub cache: CacheDisableFlag,

    #[command(flatten)]
    pub account: AccountNameFlag,
}

impl TemplateSaveCommand {
    pub async fn execute(self, printer: &mut impl Printer, config: &TomlConfig) -> Result<()> {
        info!("executing save template command");

        let folder = &self.folder.name;

        let (toml_account_config, account_config) = config.clone().into_account_configs(
            self.account.name.as_deref(),
            #[cfg(feature = "account-sync")]
            self.cache.disable,
        )?;

        let add_message_kind = toml_account_config.add_message_kind();

        let backend = Backend::new(
            &toml_account_config,
            &account_config,
            add_message_kind,
            |#[allow(unused)] builder| match add_message_kind {
                #[cfg(feature = "imap")]
                Some(BackendKind::Imap) => {
                    builder.set_add_message(|ctx| ctx.imap.as_ref().and_then(AddImapMessage::new));
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
            },
        )
        .await?;

        let is_tty = io::stdin().is_terminal();
        let is_json = printer.is_json();
        let tpl = if is_tty || is_json {
            self.template.raw()
        } else {
            io::stdin()
                .lock()
                .lines()
                .map_while(Result::ok)
                .collect::<Vec<String>>()
                .join("\n")
        };

        #[allow(unused_mut)]
        let mut compiler = MmlCompilerBuilder::new();

        #[cfg(feature = "pgp")]
        compiler.set_some_pgp(account_config.pgp.clone());

        let msg = compiler.build(tpl.as_str())?.compile().await?.into_vec()?;

        backend.add_message(folder, &msg).await?;

        printer.print(format!("Template successfully saved to {folder}!"))
    }
}
