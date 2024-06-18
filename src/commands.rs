use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader, Read, Write},
    str::FromStr,
};

use crate::{
    args::{Args, Command},
    chunk::Chunk,
    chunk_type::ChunkType,
    png::Png,
};

pub fn read_file(path: &String) -> io::Result<Vec<u8>> {
    let f = File::open(path)?;
    let buf = BufReader::new(f);

    let mut bytes: Vec<u8> = vec![];

    for byte in buf.bytes() {
        bytes.push(byte?)
    }

    Ok(bytes)
}

pub fn write_file(path: &String, contents: &[u8]) -> io::Result<()> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();

    f.write_all(contents)?;

    Ok(())
}

pub fn process(args: Args) -> io::Result<()> {
    let bytes = read_file(&args.path)?;
    if let Ok(mut png) = Png::try_from(bytes.as_slice()) {
        match args.command {
            Command::Encode => {
                let ctype = if let Ok(ctype) =
                    ChunkType::from_str(&args.chunk.clone().unwrap_or_default())
                {
                    ctype
                } else {
                    panic!("Invalid Chunk type {:?}", &args.chunk.clone())
                };
                let data = &args.message.clone();
                let chunk = Chunk::new(ctype, data.clone().unwrap_or_default().as_bytes().to_vec());
                png.append_chunk(chunk);

                write_file(&args.path, png.as_bytes().as_slice())?;
            }
            Command::Decode => {
                if let Some(chunk) = png.chunk_by_type(&args.chunk.clone().unwrap()) {
                    if let Some(data) = chunk.data_as_string() {
                        println!("Decoded data for {}:\n {}", &args.chunk.unwrap(), data);
                    }
                }
            }
            Command::Remove => {
                let ctype = &args.chunk.clone().unwrap();
                write_file(&args.path, png.as_bytes().as_slice())?;
                println!("Removed chunk {} from the file {}", ctype, &args.path);
            }
            Command::Print => {
                for c in png.chunks().iter() {
                    if let Some(msg) = c.data_as_string() {
                        if !msg.is_empty() {
                            println!("{:?} : {:?}", c.chunk_type().to_string(), msg);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
