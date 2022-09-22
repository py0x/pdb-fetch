use std::path::Path;

use reqwest;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use anyhow::Result;

#[derive(Clone)]
pub enum PdbFormat {
    Pdb,
    Cif,
}

/// Download a pdb file by pdb id/code
pub async fn download_pdb<P: AsRef<Path>>(pdb_id: &str, output_dir: P, format: &PdbFormat) -> Result<()> {
    let fmt = match format {
        PdbFormat::Pdb => "pdb",
        PdbFormat::Cif => "cif"
    };

    let pdb_file = format!("{pdb_id}.{fmt}.gz");
    let pdb_url = format!("https://files.rcsb.org/download/{pdb_file}");
    let out_file = output_dir.as_ref().join(pdb_file);

    let bytes = reqwest::get(pdb_url)
        .await?
        .bytes()
        .await?;


    let mut f = File::create(out_file).await?;
    f.write_all(&bytes).await?;
    f.sync_all().await?;

    Ok(())
}
