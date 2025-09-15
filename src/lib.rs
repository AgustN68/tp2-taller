use crate::Error::{DivisionPorCero, MensajeInesperado, OperacionInvalida};
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{io, thread};

#[derive(Debug)]
pub enum Error {
    MensajeInesperado,
    OperacionInvalida,
    DivisionPorCero,
    FaltaDireccion,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Operacion {
    Suma(u8),
    Resta(u8),
    Multiplicacion(u8),
    Division(u8),
    Get(),
}
#[derive(PartialEq, Debug)]
pub struct Calculadora {
    acumulador: u8,
}

impl Default for Calculadora {
    fn default() -> Self {
        Self::new()
    }
}

impl Calculadora {
    pub fn new() -> Self {
        Calculadora { acumulador: 0 }
    }
    pub fn aplicar_operacion(&mut self, operacion: Operacion) -> Result<(), Error> {
        match operacion {
            Operacion::Suma(valor) => {
                self.sumar(valor);
                Ok(())
            }
            Operacion::Resta(valor) => {
                self.restar(valor);
                Ok(())
            }
            Operacion::Multiplicacion(valor) => {
                self.multiplicar(valor);
                Ok(())
            }
            Operacion::Division(valor) => {
                self.dividir(valor);
                Ok(())
            }
            Operacion::Get() => Ok(()),
        }
    }

    fn sumar(&mut self, valor: u8) {
        self.acumulador += valor;
    }

    fn restar(&mut self, valor: u8) {
        self.acumulador -= valor;
    }

    fn multiplicar(&mut self, valor: u8) {
        self.acumulador *= valor;
    }
    fn dividir(&mut self, valor: u8) {
        self.acumulador /= valor;
    }
    pub fn valor(&self) -> u8 {
        self.acumulador
    }
}

pub fn parsear_argumentos() -> Result<String, Error> {
    let mut entrada = std::env::args();
    entrada.next();

    let address = entrada.next().ok_or(Error::FaltaDireccion)?;
    Ok(address)
}

fn operacion_a_calculadora(palabras: Vec<&str>) -> Result<Operacion, Error> {
    if palabras.len() < 3 {
        return Err(OperacionInvalida);
    }
    let operador = palabras[1];
    let valor: u8 = palabras[2].parse().map_err(|_| OperacionInvalida)?;

    match operador {
        "+" => Ok(Operacion::Suma(valor)),
        "-" => Ok(Operacion::Resta(valor)),
        "*" => Ok(Operacion::Multiplicacion(valor)),
        "/" => {
            if valor == 0 {
                Err(DivisionPorCero)
            } else {
                Ok(Operacion::Division(valor))
            }
        }
        _ => Err(OperacionInvalida),
    }
}

fn parsear_peticion_cliente(input: &str) -> Result<Operacion, Error> {
    let palabras: Vec<&str> = input.split_whitespace().collect();

    if palabras.is_empty() {
        return Err(OperacionInvalida);
    }
    let operacion = palabras[0];

    match operacion {
        "OP" => operacion_a_calculadora(palabras),
        "GET" => Ok(Operacion::Get()),
        _ => Err(MensajeInesperado),
    }
}

fn manejar_cliente(mut stream: TcpStream, calculadora: Arc<Mutex<Calculadora>>) -> io::Result<()> {
    let mut buffer = [0; 1024];

    while let Ok(bytes) = stream.read(&mut buffer) {
        if bytes == 0 {
            stream.shutdown(Shutdown::Both)?;
            break;
        }

        let input = String::from_utf8_lossy(&buffer[0..bytes])
            .trim()
            .to_string();
        let operacion: Operacion = match parsear_peticion_cliente(&input) {
            Ok(op) => op,
            Err(OperacionInvalida) => {
                stream.write_all(b"ERROR \"parsing error\"\n")?;
                stream.flush()?;
                eprintln!("ERROR \"parsing error\"");
                continue;
            }
            Err(Error::FaltaDireccion) => {
                stream.write_all(b"ERROR \"Falta direccion\"\n")?;
                stream.flush()?;
                eprintln!("ERROR \"falta direccion\"");
                continue;
            }
            Err(DivisionPorCero) => {
                stream.write_all(b"ERROR \"division by zero\"\n")?;
                stream.flush()?;
                eprintln!("ERROR \"division by zero\"");
                continue;
            }
            Err(MensajeInesperado) => {
                stream.write_all(b"ERROR \"unexpected message\"\n")?;
                stream.flush()?;
                eprintln!("ERROR \"unexpected message\"");
                continue;
            }
        };

        let mut calc = match calculadora.lock() {
            Ok(calc) => calc,
            Err(e) => {
                eprintln!("Error al lockear calculadora: {:?}", e);
                return Ok(());
            }
        };

        if operacion == Operacion::Get() {
            let valor = calc.valor();
            stream.write_all(format!("{}", valor).as_bytes())?;
            stream.flush()?;
            continue;
        }

        if calc.aplicar_operacion(operacion).is_ok() {
            stream.write_all(b"OK")?;
            stream.flush()?;
        }
    }
    Ok(())
}

pub fn crear_servidor(
    listener: TcpListener,
    calculadora: Arc<Mutex<Calculadora>>,
) -> Result<(), io::Error> {
    for stream in listener.incoming() {
        let stream = stream?;
        let calc = Arc::clone(&calculadora);

        thread::spawn(move || {
            if let Err(e) = manejar_cliente(stream, calc) {
                eprintln!("Error en cliente: {}", e);
            }
        });
    }
    Ok(())
}
