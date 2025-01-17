#[cfg(feature = "attachment-download")]
pub mod download;

use anyhow::Result;
use clap::Subcommand;

use crate::{config::TomlConfig, printer::Printer};

#[cfg(feature = "attachment-download")]
use self::download::AttachmentDownloadCommand;

/// Manage attachments.
///
/// A message body can be composed of multiple MIME parts. An
/// attachment is the representation of a binary part of a message
/// body.
#[derive(Debug, Subcommand)]
pub enum AttachmentSubcommand {
    #[cfg(feature = "attachment-download")]
    #[command(arg_required_else_help = true)]
    Download(AttachmentDownloadCommand),
}

impl AttachmentSubcommand {
    #[allow(unused)]
    pub async fn execute(self, printer: &mut impl Printer, config: &TomlConfig) -> Result<()> {
        match self {
            #[cfg(feature = "attachment-download")]
            Self::Download(cmd) => cmd.execute(printer, config).await,
        }
    }
}
