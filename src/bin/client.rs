use std::fs::File;
use std::io;
use std::io::{BufRead, Write};
use std::net::*;

fn main() -> Result<(),io::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Uso: cargo run --bin client -- <ip:puerto> <archivo>");
        return Ok(())
    }
    let addres = &args[1];
    let archivo = &args[2];

    let mut stream = TcpStream::connect(&addres)?;
    
    println!("Connectado al servidor!");
    
    
    let file = File::open(archivo)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let linea = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Error al leer linea {}", error);
                break;
            }
        };
        
        println!("Linea: {}",&linea);

        stream.write(linea.as_bytes())?;
        stream.flush()?;
    }
    
    Ok(())

}