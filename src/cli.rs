use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PdbFormat {
    Pdb,
    Cif,
}

/// Command=line arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The input file containing a newline-separated list of PDB ids
    #[clap(short, long, value_parser, display_order = 1)]
    pub file: PathBuf,

    /// The output dir, default: current dir
    #[clap(short, long, value_parser, display_order = 2)]
    pub output: Option<PathBuf>,

    /// Download file format each PDB id
    #[clap(long, arg_enum, value_parser, default_value_t = PdbFormat::Pdb, display_order = 3)]
    pub format: PdbFormat,

    /// Number of concurrency for batch downloading
    #[clap(long, value_parser, default_value_t = 50, display_order = 4)]
    pub concurrency: u16,

    /// The file for error PDB ids, default: "error.pdb-id.log"
    #[clap(short, long, value_parser, display_order = 5)]
    pub err_file: Option<PathBuf>,
}