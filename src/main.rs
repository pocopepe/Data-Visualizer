use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Result};
use std::io::Write;
use std::env;

fn most_frequent_byte(chunk: &[u8]) -> u8 {
    let mut counts = [0usize; 256];
    for &byte in chunk {
        counts[byte as usize] += 1;
    }
    let (max_byte, _) = counts.iter().enumerate()
        .max_by_key(|&(_, count)| count)
        .unwrap();
    max_byte as u8
}

fn overwrite_entire_file(file: &mut std::fs::File) -> Result<()> {
    let file_size = file.metadata()?.len();
    file.seek(SeekFrom::Start(0))?;

    let zero_chunk = vec![0u8; 1024 * 1024]; // 1MB chunks
    let mut bytes_written = 0u64;

    println!("File size: {} bytes", file_size);
    
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
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    // Phase 1: Analyze original file content in 200x200 grid
    println!("=== Analyzing {} in 200x200 grid ===", filename);
    {
        let mut file = std::fs::File::open(filename)?;
        let file_size = file.metadata()?.len() as usize;
        let chunk_size = file_size / 40000; // 200*200 = 40000 chunks
        
        if chunk_size == 0 {
            println!("File too small for 200x200 grid analysis (file size: {} bytes)", file_size);
            println!("Using single byte per grid cell instead.");
        }
        
        let effective_chunk_size = std::cmp::max(chunk_size, 1);
        let mut buffer = vec![0u8; effective_chunk_size];
        
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
            println!(); // New line after each row
        }
    }

    // Phase 2: Complete file overwrite
    println!("\n=== Overwriting entire file ===");
    {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(filename)?;
        
        println!("Starting complete file overwrite...");
        overwrite_entire_file(&mut file)?;
        println!("File overwrite completed successfully.");
    }

    Ok(())
}