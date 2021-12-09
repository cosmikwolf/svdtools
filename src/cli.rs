use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

use svdtools::{
    interrupts::interrupts_cli, makedeps::makedeps_cli, mmap::mmap_cli, patch::patch_cli,
};

#[derive(StructOpt, Debug)]
enum Command {
    /// Patches an SVD file as specified by a YAML file
    Patch {
        #[structopt(parse(from_os_str))]
        svd_file: PathBuf,
    },
    /// Generate Make dependency file listing dependencies for a YAML file.
    Makedeps {
        /// Input yaml file
        #[structopt(parse(from_os_str))]
        yaml_file: PathBuf,

        /// Dependencies output file
        #[structopt(parse(from_os_str))]
        deps_file: PathBuf,
    },
    /// Print list of all interrupts described by an SVD file
    Interrupts {
        #[structopt(parse(from_os_str))]
        svd_file: PathBuf,

        /// Whether to print gaps in interrupt number sequence
        #[structopt(long)]
        no_gaps: bool,
    },
    /// Generate text-based memory map of an SVD file.
    Mmap {
        #[structopt(parse(from_os_str))]
        svd_file: PathBuf,
    },
}

impl Command {
    pub fn run(&self) -> Result<()> {
        match self {
            Self::Interrupts { svd_file, no_gaps } => {
                interrupts_cli::parse_device(svd_file, !no_gaps)?;
            }
            Self::Mmap { svd_file } => mmap_cli::parse_device(svd_file)?,
            Self::Patch { svd_file } => patch_cli::patch(svd_file)?,
            Self::Makedeps {
                yaml_file,
                deps_file,
            } => makedeps_cli::makedeps(yaml_file, deps_file)?,
        }
        Ok(())
    }
}

#[derive(StructOpt, Debug)]
struct CliArgs {
    #[structopt(subcommand)]
    command: Command,
}

pub fn run() {
    env_logger::init();

    let args = CliArgs::from_args();
    if let Err(e) = args.command.run() {
        log::error!("{:?}", e);

        std::process::exit(1);
    }
}