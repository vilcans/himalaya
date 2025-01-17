use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[cfg(feature = "account-subcmd")]
use crate::account::command::AccountSubcommand;
#[cfg(feature = "envelope-subcmd")]
use crate::envelope::command::EnvelopeSubcommand;
#[cfg(feature = "flag-subcmd")]
use crate::flag::command::FlagSubcommand;
#[cfg(feature = "folder-subcmd")]
use crate::folder::command::FolderSubcommand;
#[cfg(feature = "attachment-subcmd")]
use crate::message::attachment::command::AttachmentSubcommand;
#[cfg(feature = "message-subcmd")]
use crate::message::command::MessageSubcommand;
#[cfg(feature = "template-subcmd")]
use crate::message::template::command::TemplateSubcommand;
#[allow(unused)]
use crate::{
    completion::command::CompletionGenerateCommand,
    config::{self, TomlConfig},
    manual::command::ManualGenerateCommand,
    output::{ColorFmt, OutputFmt},
    printer::Printer,
};

#[derive(Parser, Debug)]
#[command(name = "himalaya", author, version, about)]
#[command(propagate_version = true, infer_subcommands = true)]
pub struct Cli {
    #[cfg(feature = "envelope-list")]
    #[command(subcommand)]
    pub command: Option<HimalayaCommand>,
    #[cfg(not(feature = "envelope-list"))]
    #[command(subcommand)]
    pub command: HimalayaCommand,

    /// Override the default configuration file path
    ///
    /// The given path is shell-expanded then canonicalized (if
    /// applicable). If the path does not point to a valid file, the
    /// wizard will propose to assist you in the creation of the
    /// configuration file.
    #[arg(short, long = "config", global = true)]
    #[arg(value_name = "PATH", value_parser = config::path_parser)]
    pub config_path: Option<PathBuf>,

    /// Customize the output format
    ///
    /// The output format determine how to display commands output to
    /// the terminal.
    ///
    /// The possible values are:
    ///
    ///  - json: output will be in a form of a JSON-compatible object
    ///
    ///  - plain: output will be in a form of either a plain text or
    ///    table, depending on the command
    #[arg(long, short, global = true)]
    #[arg(value_name = "FORMAT", value_enum, default_value_t = Default::default())]
    pub output: OutputFmt,

    /// Control when to use colors
    ///
    /// The default setting is 'auto', which means himalaya will try
    /// to guess when to use colors. For example, if himalaya is
    /// printing to a terminal, then it will use colors, but if it is
    /// redirected to a file or a pipe, then it will suppress color
    /// output. himalaya will suppress color output in some other
    /// circumstances as well. For example, if the TERM environment
    /// variable is not set or set to 'dumb', then himalaya will not
    /// use colors.
    ///
    /// The possible values are:
    ///
    ///  - never: colors will never be used
    ///
    ///  - always: colors will always be used regardless of where output is sent
    ///
    ///  - ansi: like 'always', but emits ANSI escapes (even in a Windows console)
    ///
    ///  - auto: himalaya tries to be smart
    #[arg(long, short = 'C', global = true)]
    #[arg(value_name = "MODE", value_enum, default_value_t = Default::default())]
    pub color: ColorFmt,
}

#[derive(Subcommand, Debug)]
pub enum HimalayaCommand {
    #[cfg(feature = "account-subcmd")]
    #[command(subcommand)]
    #[command(alias = "accounts")]
    Account(AccountSubcommand),

    #[cfg(feature = "folder-subcmd")]
    #[command(subcommand)]
    #[command(visible_alias = "mailbox", aliases = ["mailboxes", "mboxes", "mbox"])]
    #[command(alias = "folders")]
    Folder(FolderSubcommand),

    #[cfg(feature = "envelope-subcmd")]
    #[command(subcommand)]
    #[command(alias = "envelopes")]
    Envelope(EnvelopeSubcommand),

    #[cfg(feature = "flag-subcmd")]
    #[command(subcommand)]
    #[command(alias = "flags")]
    Flag(FlagSubcommand),

    #[cfg(feature = "message-subcmd")]
    #[command(subcommand)]
    #[command(alias = "messages", alias = "msgs", alias = "msg")]
    Message(MessageSubcommand),

    #[cfg(feature = "attachment-subcmd")]
    #[command(subcommand)]
    #[command(alias = "attachments")]
    Attachment(AttachmentSubcommand),

    #[cfg(feature = "template-subcmd")]
    #[command(subcommand)]
    #[command(alias = "templates", alias = "tpls", alias = "tpl")]
    Template(TemplateSubcommand),

    #[command(arg_required_else_help = true)]
    #[command(alias = "manuals", alias = "mans")]
    Manual(ManualGenerateCommand),

    #[command(arg_required_else_help = true)]
    #[command(alias = "completions")]
    Completion(CompletionGenerateCommand),
}

impl HimalayaCommand {
    #[allow(unused)]
    pub async fn execute(
        self,
        printer: &mut impl Printer,
        config_path: Option<&PathBuf>,
    ) -> Result<()> {
        match self {
            #[cfg(feature = "account-subcmd")]
            Self::Account(cmd) => {
                let config = TomlConfig::from_some_path_or_default(config_path).await?;
                cmd.execute(printer, &config).await
            }
            #[cfg(feature = "folder-subcmd")]
            Self::Folder(cmd) => {
                let config = TomlConfig::from_some_path_or_default(config_path).await?;
                cmd.execute(printer, &config).await
            }
            #[cfg(feature = "envelope-subcmd")]
            Self::Envelope(cmd) => {
                let config = TomlConfig::from_some_path_or_default(config_path).await?;
                cmd.execute(printer, &config).await
            }
            #[cfg(feature = "flag-subcmd")]
            Self::Flag(cmd) => {
                let config = TomlConfig::from_some_path_or_default(config_path).await?;
                cmd.execute(printer, &config).await
            }
            #[cfg(feature = "message-subcmd")]
            Self::Message(cmd) => {
                let config = TomlConfig::from_some_path_or_default(config_path).await?;
                cmd.execute(printer, &config).await
            }
            #[cfg(feature = "attachment-subcmd")]
            Self::Attachment(cmd) => {
                let config = TomlConfig::from_some_path_or_default(config_path).await?;
                cmd.execute(printer, &config).await
            }
            #[cfg(feature = "template-subcmd")]
            Self::Template(cmd) => {
                let config = TomlConfig::from_some_path_or_default(config_path).await?;
                cmd.execute(printer, &config).await
            }
            Self::Manual(cmd) => cmd.execute(printer).await,
            Self::Completion(cmd) => cmd.execute().await,
        }
    }
}
