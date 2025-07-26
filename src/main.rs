use clap::{Parser, Subcommand};
use unfolder::cli::{ArgsDispatcher, ParserDispatcher, SubcommandDispatcher};
use unfolder::{unfold_file, fold_file, Error, Exit, Result};
use iocore::Path;

#[derive(Parser, Debug, Clone)]
#[command(
    author,
    version,
    about,
    long_about = "unfold a file into a folder and fold a previously unfolded folder into a file"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}
impl Cli {
    pub fn command(&self) -> Command {
        self.command.clone()
    }
}

impl ParserDispatcher<Error> for Cli {
    fn dispatch(&self) -> Result<()> {
        self.command.dispatch()?;

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Fold(FoldOpt),
    Unfold(UnfoldOpt),
}
impl SubcommandDispatcher<Error> for Command {
    fn dispatch(&self) -> Result<()> {
        match self {
            Command::Fold(op) => op.dispatch()?,
            Command::Unfold(op) => op.dispatch()?,
        }
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
#[command(about = "unfolds the input file into multiple files in the output folder")]
pub struct UnfoldOpt {
    #[arg(required = true, help = "input file")]
    input_path: Path,

    #[arg(required = true, help = "output folder")]
    output_path: Path,
}
impl UnfoldOpt {
    pub fn input_path(&self) -> Path {
        self.input_path.clone()
    }

    pub fn output_path(&self) -> Path {
        self.output_path.clone()
    }
}

impl ArgsDispatcher<Error> for UnfoldOpt {
    fn dispatch(&self) -> Result<()> {
        let input_path = self.input_path.to_string();
        unfold_file(&self.input_path, &self.output_path, |progress| {
            println!("{input_path} => {progress}");
        })?;
        Ok(())
    }
}

#[derive(Parser, Debug, Clone)]
#[command(about = "folds multiple files from the input folder into a single output file")]
pub struct FoldOpt {
    #[arg(required = true, help = "input folder")]
    input_path: Path,

    #[arg(required = true, help = "output file")]
    output_path: Path,
}
impl FoldOpt {
    pub fn input_path(&self) -> Path {
        self.input_path.clone()
    }

    pub fn output_path(&self) -> Path {
        self.output_path.clone()
    }
}

impl ArgsDispatcher<Error> for FoldOpt {
    fn dispatch(&self) -> Result<()> {
        let output_path = self.output_path.to_string();
        fold_file(&self.input_path, &self.output_path, |progress| {
            println!("{progress} => {output_path}");
        })?;

        Ok(())
    }
}


fn main() -> Exit {
    Cli::main()
}
