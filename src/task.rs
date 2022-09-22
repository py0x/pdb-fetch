use std::path::{Path};

use async_channel;
use async_channel::{Sender, Receiver};

use tokio;
use tokio::fs::File;
use std::fs::create_dir_all;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};

use anyhow;
use anyhow::{Context, Error, Result};

use tokio::task::JoinHandle;

use crate::download;


pub fn spawn_task_read_pdb_ids<P: AsRef<Path>>(
    pdb_id_chan: Sender<String>, pdb_id_file: P) -> JoinHandle<Result<()>> {
    let pdb_file = pdb_id_file.as_ref().to_owned();

    tokio::spawn(async move {
        let file = File::open(&pdb_file).await.context(format!(
            "Failed to open pdb id file: {}", pdb_file.display()
        ))?;

        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        while let Some(line) = lines.next_line().await? {
            let line = line.trim();
            if !line.is_empty() {
                pdb_id_chan.send(line.to_owned()).await?;
            }
        }

        Ok::<(), Error>(())
    })
}

pub fn spawn_task_download_pdb<P: AsRef<Path>>(
    pdb_id_chan: Receiver<String>,
    output_dir: P,
    format: &download::PdbFormat,
    pdb_id_err_chan: Sender<String>,
) -> JoinHandle<Result<()>> {
    let output_dir = output_dir.as_ref().to_owned();
    let fmt = (*format).to_owned();

    create_dir_all(&output_dir).unwrap();

    tokio::spawn(async move {
        while let Ok(pdb_id) = pdb_id_chan.recv().await {
            let r = download::download_pdb(
                &pdb_id,
                &output_dir,
                &fmt,
            ).await;

            match r {
                Ok(()) => {
                    println!("Successfully download: {pdb_id}");
                }
                Err(e) => {
                    println!("Failed to download: {pdb_id}, err: {e}");
                    pdb_id_err_chan.send(pdb_id).await?;
                }
            }
        }

        Ok::<(), Error>(())
    })
}

pub fn spawn_task_write_failed_pdb_ids<P: AsRef<Path>>(
    pdb_id_err_chan: Receiver<String>,
    pdb_id_err_file: P,
) -> JoinHandle<Result<()>> {
    let pdb_id_err_file = pdb_id_err_file.as_ref().to_owned();

    tokio::spawn(async move {
        let mut file = File::create(pdb_id_err_file).await?;

        while let Ok(pdb_id) = pdb_id_err_chan.recv().await {
            if let Err(e) = file.write(format!("{pdb_id}\n").as_bytes()).await {
                println!("Failed to log err pdb_id: {pdb_id}, Err: {e}");
            }
        }

        Ok::<(), Error>(())
    })
}