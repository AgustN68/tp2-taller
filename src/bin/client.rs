use std::fs::File;
use std::io;
use std::io::{BufRead, Read, Write};
use std::net::*;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Uso: cargo run --bin client -- <ip:puerto> <archivo>");
        return Ok(());
    }
    let addres = &args[1];
    let archivo = &args[2];

    let mut stream = TcpStream::connect(addres)?;

    // println!("Connectado al servidor!");

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

        //println!("client : {}", &linea);
        if let Some(c) = linea.chars().next() {
            match c {
                '+' | '-' | '*' | '/' => {
                    stream.write_all(format!("OP {}\n", linea).as_bytes())?;
                    stream.flush()?;
                }
                _ => {
                    stream.write_all(format!("{}\n", linea).as_bytes())?;
                    stream.flush()?;
                }
            }
        }
        let mut buffer = [0; 1024];
        let bytes = stream.read(&mut buffer)?;
        let _ = String::from_utf8_lossy(&buffer[0..bytes]);
        //println!("{}", &r);
    }
    stream.write_all("GET\n".as_bytes())?;
    stream.flush()?;
    let mut buffer = [0; 1024];
    let bytes = stream.read(&mut buffer)?;
    let r = String::from_utf8_lossy(&buffer[0..bytes]);
    println!("{}", &r);
    Ok(())
}
