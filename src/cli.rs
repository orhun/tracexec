use std::{num::ParseFloatError, path::PathBuf};

use clap::{Parser, Subcommand};

use crate::tui::app::AppLayout;

use self::{
  args::{ModifierArgs, TracerEventArgs, LogModeArgs},
  options::{ActivePane, Color},
};

pub mod args;
pub mod options;
#[cfg(test)]
mod test;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
  #[arg(long, default_value_t = Color::Auto, help = "Control whether colored output is enabled. This flag has no effect on TUI mode.")]
  pub color: Color,
  #[arg(
    short = 'C',
    long,
    help = "Change current directory to this path before doing anything"
  )]
  pub cwd: Option<PathBuf>,
  #[arg(
    short,
    long,
    help = "Run as user. This option is only available when running tracexec as root"
  )]
  pub user: Option<String>,
  #[clap(subcommand)]
  pub cmd: CliCommand,
}

#[derive(Subcommand, Debug)]
pub enum CliCommand {
  #[clap(about = "Run tracexec in logging mode")]
  Log {
    #[arg(last = true, required = true, help = "command to be executed")]
    cmd: Vec<String>,
    #[clap(flatten)]
    tracing_args: LogModeArgs,
    #[clap(flatten)]
    modifier_args: ModifierArgs,
    #[clap(flatten)]
    tracer_event_args: TracerEventArgs,
    #[clap(
      short,
      long,
      help = "Output, stderr by default. A single hyphen '-' represents stdout."
    )]
    output: Option<PathBuf>,
  },
  #[clap(about = "Run tracexec in TUI mode, stdin/out/err are redirected to /dev/null by default")]
  Tui {
    #[arg(last = true, required = true, help = "command to be executed")]
    cmd: Vec<String>,
    #[clap(flatten)]
    modifier_args: ModifierArgs,
    #[clap(flatten)]
    tracer_event_args: TracerEventArgs,
    #[clap(
      long,
      short,
      help = "Allocate a pseudo terminal and show it alongside the TUI"
    )]
    tty: bool,
    #[clap(long, short, help = "Keep the event list scrolled to the bottom")]
    follow: bool,
    #[clap(
      long,
      help = "Instead of waiting for the root child to exit, terminate when the TUI exits",
      conflicts_with = "kill_on_exit"
    )]
    terminate_on_exit: bool,
    #[clap(
      long,
      help = "Instead of waiting for the root child to exit, kill when the TUI exits"
    )]
    kill_on_exit: bool,
    #[clap(
      long,
      short = 'A',
      help = "Set the default active pane to use when TUI launches",
      requires = "tty",
      default_value_t
    )]
    active_pane: ActivePane,
    #[clap(
      long,
      short = 'L',
      help = "Set the layout of the TUI when it launches",
      requires = "tty",
      default_value_t
    )]
    layout: AppLayout,
    #[clap(
      long,
      short = 'F',
      help = "Set the frame rate of the TUI",
      default_value = "60.0",
      value_parser = frame_rate_parser
    )]
    frame_rate: f64,
  },
}

#[derive(thiserror::Error, Debug)]
enum ParseFrameRateError {
  #[error("Failed to parse frame rate {0} as a floating point number")]
  ParseFloatError(ParseFloatError),
  #[error("Invalid frame rate")]
  InvalidFrameRate,
  #[error("Frame rate too low, must be at least 5.0")]
  FrameRateTooLow,
}

impl From<ParseFloatError> for ParseFrameRateError {
  fn from(e: ParseFloatError) -> Self {
    Self::ParseFloatError(e)
  }
}

fn frame_rate_parser(s: &str) -> Result<f64, ParseFrameRateError> {
  let v = s.parse::<f64>()?;
  if v < 0.0 || v.is_nan() || v.is_infinite() {
    Err(ParseFrameRateError::InvalidFrameRate)
  } else if v < 5.0 {
    Err(ParseFrameRateError::FrameRateTooLow)
  } else {
    Ok(v)
  }
}
