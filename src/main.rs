use std::env;
use std::path::{PathBuf};
use std::vec::Vec;

use async_channel;
use async_channel::{Sender, Receiver};

use tokio;

use anyhow;
use anyhow::{Result};

use clap::Parser;
use tokio::task::JoinHandle;

mod cli;
mod download;
mod task;


#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    let pdb_id_file = args.file;

    // output
    let output_dir = match args.output {
        None => env::current_dir()?,
        Some(dir) => dir,
    };

    // concurrency
    let concurrency = args.concurrency;


    // format
    let fmt = match args.format {
        cli::PdbFormat::Pdb => download::PdbFormat::Pdb,
        cli::PdbFormat::Cif => download::PdbFormat::Cif,
    };

    // err pdb ids
    let err_file = match args.err_file {
        None => PathBuf::from("error.pdb-id.log"),
        Some(f) => f,
    };


    let (id_send, id_recv): (Sender<String>, Receiver<String>) = async_channel::bounded(1000);
    let (id_err_send, id_err_recv): (Sender<String>, Receiver<String>) = async_channel::bounded(1000);


    let task_gen_pdb_ids = task::spawn_task_read_pdb_ids(
        id_send,
        pdb_id_file,
    );

    let mut download_tasks: Vec<JoinHandle<Result<()>>> = Vec::new();

    for _ in 0..concurrency {
        let t = task::spawn_task_download_pdb(
            id_recv.clone(),
            &output_dir,
            &fmt,
            id_err_send.clone(),
        );
        download_tasks.push(t);
    }

    drop(id_recv);
    drop(id_err_send);


    let task_write_failed_pdb_ids = task::spawn_task_write_failed_pdb_ids(
        id_err_recv,
        &err_file,
    );

    let _ = task_gen_pdb_ids.await?;

    for t in download_tasks {
        let _ = t.await?;
    }

    let _ = task_write_failed_pdb_ids.await?;

    Ok(())
}
