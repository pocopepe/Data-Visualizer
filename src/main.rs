use std::fs::{self, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Result, Write};
use std::path::Path;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "file-tool")]
#[command(about = "A tool for file analysis and secure overwriting")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Visualize file content in a 200x200 grid
    Visualize {
        /// Input file path
        #[arg(short, long)]
        file: String,
    },
    /// Securely overwrite file with zeros
    Overwrite {
        /// Input file or directory path
        #[arg(short, long)]
        file: String,
        /// Skip confirmation prompt
        #[arg(short = 'y', long)]
        yes: bool,
        /// Recursively overwrite all files in a directory
        #[arg(short, long)]
        recursive: bool,
    },
}

// Finds the most frequent byte in a chunk of data.
fn most_frequent_byte(chunk: &[u8]) -> u8 {
    if chunk.is_empty() {
        return 0;
    }
    let mut counts = [0usize; 256];
    for &byte in chunk {
        counts[byte as usize] += 1;
    }
    let (max_byte, _) = counts.iter().enumerate()
        .max_by_key(|&(_, count)| count)
        .unwrap();
    max_byte as u8
}

// Visualizes a file by analyzing the most frequent byte in 200x200 chunks.
fn visualize_file(filename: &str) -> Result<()> {
    println!("=== Visualizing {} in 200x200 grid ===", filename);
    
    let mut file = std::fs::File::open(filename)?;
    let file_size = file.metadata()?.len() as usize;
    let chunk_size = file_size / 40000;
    
    if file_size == 0 {
        println!("File is empty!");
        return Ok(());
    }
    
    if chunk_size == 0 {
        println!("File too small for 200x200 grid analysis (file size: {} bytes)", file_size);
        println!("Using single byte per grid cell instead.");
    }
    
    let effective_chunk_size = std::cmp::max(chunk_size, 1);
    let mut buffer = vec![0u8; effective_chunk_size];
    
    println!("File size: {} bytes | Chunk size: {} bytes", file_size, effective_chunk_size);
    println!("Grid (each cell = most frequent byte in chunk):\n");
    
    for row in 0..200 {
        for col in 0..200 {
            let chunk_idx = row * 200 + col;
            let offset = (chunk_idx * effective_chunk_size) as u64;
            
            if offset >= file_size as u64 {
                print!("00 ");
                continue;
            }
            
            file.seek(SeekFrom::Start(offset))?;
            let bytes_read = file.read(&mut buffer)?;
            
            if bytes_read == 0 {
                print!("00 ");
            } else {
                let representative_byte = most_frequent_byte(&buffer[..bytes_read]);
                print!("{:02x} ", representative_byte);
            }
        }
        println!();
    }
    
    Ok(())
}

// Handles overwriting a single file with zeros.
fn overwrite_single_file(filename: &str) -> Result<()> {
    println!("=== Overwriting {} ===", filename);
    
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(filename)?;
        
    let file_size = file.metadata()?.len();
    file.seek(SeekFrom::Start(0))?;
    let zero_chunk = vec![0u8; 1024 * 1024];
    let mut bytes_written = 0u64;
    
    println!("File size: {} bytes", file_size);
    println!("Starting overwrite...");
    
    while bytes_written < file_size {
        let bytes_to_write = std::cmp::min(zero_chunk.len() as u64, file_size - bytes_written) as usize;
        file.write_all(&zero_chunk[..bytes_to_write])?;
        bytes_written += bytes_to_write as u64;
        
        // Progress indicator for large files
        if bytes_written % (100 * 1024 * 1024) == 0 || bytes_written == file_size {
            let progress = (bytes_written as f64 / file_size as f64) * 100.0;
            println!("Progress: {:.1}% ({} / {} bytes)", progress, bytes_written, file_size);
        }
    }
    file.flush()?;
    file.sync_all()?;
    
    println!("File overwrite completed successfully.");
    Ok(())
}

// Handles overwriting a single file or recursively overwriting a directory.
fn overwrite_file(path_str: &str, skip_confirmation: bool, recursive: bool) -> Result<()> {
    let path = Path::new(path_str);
    
    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Path not found: {}", path_str)));
    }
    
    if path.is_dir() {
        if !recursive {
            println!("Error: '{}' is a directory. Use the --recursive flag to overwrite files within it.", path_str);
            return Ok(());
        }
        
        // Confirmation for recursive overwrite
        if !skip_confirmation {
            println!("WARNING: This will permanently overwrite ALL data in ALL files within the directory '{}' and its subdirectories.", path_str);
            print!("Are you absolutely sure you want to continue? (y/N): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if !input.trim().to_lowercase().starts_with('y') {
                println!("Operation cancelled.");
                return Ok(());
            }
        }
        
        // Recursively walk the directory and overwrite files
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            
            if entry_path.is_file() {
                overwrite_single_file(entry_path.to_str().unwrap())?;
            } else if entry_path.is_dir() && recursive {
                overwrite_file(entry_path.to_str().unwrap(), true, true)?;
            }
        }
        
    } else if path.is_file() {
        // Confirmation for single file overwrite
        if !skip_confirmation {
            println!("WARNING: This will permanently overwrite all data in '{}'", path_str);
            print!("Are you sure you want to continue? (y/N): ");
            io::stdout().flush()?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if !input.trim().to_lowercase().starts_with('y') {
                println!("Operation cancelled.");
                return Ok(());
            }
        }
        
        overwrite_single_file(path_str)?;
    }
    
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Visualize { file } => {
            visualize_file(&file)?;
        }
        Commands::Overwrite { file, yes, recursive } => {
            overwrite_file(&file, *yes, *recursive)?;
        }
    }
    
    Ok(())
}
