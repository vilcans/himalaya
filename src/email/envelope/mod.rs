pub mod arg;
pub mod command;
pub mod config;
pub mod flag;

#[cfg(feature = "envelope-list")]
use anyhow::Result;
#[cfg(feature = "envelope-list")]
use email::account::config::AccountConfig;
use serde::Serialize;
#[cfg(feature = "envelope-list")]
use std::ops;

use crate::flag::Flags;
#[cfg(feature = "envelope-list")]
use crate::{
    cache::IdMapper,
    flag::Flag,
    printer::{PrintTable, PrintTableOpts, WriteColor},
    ui::{Cell, Row, Table},
};

#[derive(Clone, Debug, Default, Serialize)]
pub struct Mailbox {
    pub name: Option<String>,
    pub addr: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Envelope {
    pub id: String,
    pub flags: Flags,
    pub subject: String,
    pub from: Mailbox,
    pub date: String,
}

#[cfg(feature = "envelope-list")]
impl Table for Envelope {
    fn head() -> Row {
        Row::new()
            .cell(Cell::new("ID").bold().underline().white())
            .cell(Cell::new("FLAGS").bold().underline().white())
            .cell(Cell::new("SUBJECT").shrinkable().bold().underline().white())
            .cell(Cell::new("FROM").bold().underline().white())
            .cell(Cell::new("DATE").bold().underline().white())
    }

    fn row(&self) -> Row {
        let id = self.id.to_string();
        let unseen = !self.flags.contains(&Flag::Seen);
        let flags = {
            let mut flags = String::new();
            flags.push_str(if !unseen { " " } else { "✷" });
            flags.push_str(if self.flags.contains(&Flag::Answered) {
                "↵"
            } else {
                " "
            });
            flags.push_str(if self.flags.contains(&Flag::Flagged) {
                "⚑"
            } else {
                " "
            });
            flags
        };
        let subject = &self.subject;
        let sender = if let Some(name) = &self.from.name {
            name
        } else {
            &self.from.addr
        };
        let date = &self.date;

        Row::new()
            .cell(Cell::new(id).bold_if(unseen).red())
            .cell(Cell::new(flags).bold_if(unseen).white())
            .cell(Cell::new(subject).shrinkable().bold_if(unseen).green())
            .cell(Cell::new(sender).bold_if(unseen).blue())
            .cell(Cell::new(date).bold_if(unseen).yellow())
    }
}

#[cfg(feature = "envelope-list")]
/// Represents the list of envelopes.
#[derive(Clone, Debug, Default, Serialize)]
pub struct Envelopes(Vec<Envelope>);

#[cfg(feature = "envelope-list")]
impl Envelopes {
    pub fn from_backend(
        config: &AccountConfig,
        id_mapper: &IdMapper,
        envelopes: email::envelope::Envelopes,
    ) -> Result<Envelopes> {
        let envelopes = envelopes
            .iter()
            .map(|envelope| {
                Ok(Envelope {
                    id: id_mapper.get_or_create_alias(&envelope.id)?,
                    flags: envelope.flags.clone().into(),
                    subject: envelope.subject.clone(),
                    from: Mailbox {
                        name: envelope.from.name.clone(),
                        addr: envelope.from.addr.clone(),
                    },
                    date: envelope.format_date(config),
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Envelopes(envelopes))
    }
}

#[cfg(feature = "envelope-list")]
impl ops::Deref for Envelopes {
    type Target = Vec<Envelope>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "envelope-list")]
impl PrintTable for Envelopes {
    fn print_table(&self, writer: &mut dyn WriteColor, opts: PrintTableOpts) -> Result<()> {
        writeln!(writer)?;
        Table::print(writer, self, opts)?;
        writeln!(writer)?;
        Ok(())
    }
}
