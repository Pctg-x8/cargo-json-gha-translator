use std::convert::TryFrom;
use std::path::PathBuf;

use crate::cargo;

pub enum WorkflowCommand {
    Warning {
        file: PathBuf,
        line: usize,
        col: usize,
        message: String,
    },
    Error {
        file: PathBuf,
        line: usize,
        col: usize,
        message: String,
    },
}
impl std::fmt::Display for WorkflowCommand {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Warning {
                file,
                line,
                col,
                message,
            } => write!(
                fmt,
                "::warning file={},line={},col={}::{}",
                file.display(),
                line,
                col,
                message
            ),
            Self::Error {
                file,
                line,
                col,
                message,
            } => write!(
                fmt,
                "::error file={},line={},col={}::{}",
                file.display(),
                line,
                col,
                message
            ),
        }
    }
}
impl WorkflowCommand {
    pub fn is_error(&self) -> bool {
        matches!(self, Self::Error { .. })
    }
}

impl TryFrom<cargo::jsonfmt::BuildEngineMessage> for WorkflowCommand {
    type Error = ();

    fn try_from(base: cargo::jsonfmt::BuildEngineMessage) -> Result<Self, ()> {
        match base {
            cargo::jsonfmt::BuildEngineMessage::CompilerMessage {
                message:
                    cargo::jsonfmt::CompilerMessage {
                        level: cargo::jsonfmt::MessageLevel::Warning,
                        message,
                        mut spans,
                    },
            } if !spans.is_empty() => {
                let span = spans.pop().unwrap();

                Ok(Self::Warning {
                    file: std::env::current_dir()
                        .expect("Failed to get current dir")
                        .join(span.file_name),
                    line: span.line_start,
                    col: span.column_start,
                    message,
                })
            }
            cargo::jsonfmt::BuildEngineMessage::CompilerMessage {
                message:
                    cargo::jsonfmt::CompilerMessage {
                        level: cargo::jsonfmt::MessageLevel::Error,
                        message,
                        mut spans,
                    },
            } if !spans.is_empty() => {
                let span = spans.pop().unwrap();

                Ok(Self::Error {
                    file: std::env::current_dir()
                        .expect("Failed to get current dir")
                        .join(span.file_name),
                    line: span.line_start,
                    col: span.column_start,
                    message,
                })
            }
            _ => Err(()),
        }
    }
}
