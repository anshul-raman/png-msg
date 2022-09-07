use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};
use anyhow::{bail, Result};
use clap::Args;
use std::{fs, path::PathBuf, str::FromStr};

#[derive(Args, Debug)]
#[clap(about = "Does awesome things")]
pub struct EncodeArgs {
    #[clap(value_parser)]
    file_path: PathBuf,

    #[clap(value_parser)]
    chunk_type: String,

    #[clap(value_parser)]
    message: String,

    #[clap(value_parser)]
    output_file: Option<PathBuf>,
}

#[derive(Args)]
pub struct DecodeArgs {
    #[clap(value_parser)]
    file_path: PathBuf,

    #[clap(value_parser)]
    chunk_type: String,
}

#[derive(Args)]
pub struct RemoveArgs {
    #[clap(value_parser)]
    file_path: PathBuf,

    #[clap(value_parser)]
    chunk_type: String,
}

#[derive(Args)]
pub struct PrintArgs {
    #[clap(value_parser)]
    file_path: PathBuf,
}

impl EncodeArgs {
    pub fn encode(&self) -> Result<()> {
        let mut png_image = Png::from_file(&self.file_path)?;
        let new_chunk_type = ChunkType::from_str(&self.chunk_type)?;
        let new_chunk = Chunk::new(new_chunk_type, self.message.as_bytes().to_vec());

        png_image.append_chunk(new_chunk);

        if let Some(path) = &self.output_file {
            fs::write(path, png_image.as_bytes())?;
            println!("Saved output to: {}", path.display());
        } else {
            let path = std::env::current_dir()?.join("output.png");
            fs::write(&path, png_image.as_bytes())?;
            println!("Saved output to: {}", path.display());
        }

        Ok(())
    }
}

impl DecodeArgs {
    pub fn decode(&self) -> Result<()> {
        let png_image = Png::from_file(&self.file_path)?;
        let chunk = png_image.chunk_by_type(&self.chunk_type);
        if let Some(chunk) = chunk {
            println!("{}", chunk);
        } else {
            bail!("Not found");
        };

        Ok(())
    }
}

impl RemoveArgs {
    pub fn remove(&self) -> Result<()> {
        let mut png_image = Png::from_file(&self.file_path)?;
        png_image.remove_chunk(&self.chunk_type)?;
        fs::write(&self.file_path, png_image.as_bytes())?;
        println!("Chunk Removed");
        Ok(())
    }
}

impl PrintArgs {
    pub fn print(&self) -> Result<()> {
        let png_image = Png::from_file(&self.file_path)?;
        println!("{}", png_image);
        Ok(())
    }
}
