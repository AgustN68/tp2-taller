use std::{io, thread};
use std::io::Read;
use std::net::*;
use crate::Error::OperacionInvalida;
use crate::ParseError::FaltaDireccion;

#[derive(PartialEq, Debug)]
struct Calculadora {
    acumulador : u8
}
#[derive(PartialEq, Eq, Debug)]
enum Operacion {
    Suma(u8),
    Resta(u8),
    Multiplicacion(u8),
    Division(u8),
    Get(),
}
impl Calculadora { 
    pub fn new() -> Self{
        Calculadora { acumulador: 0 }
    }
    pub fn aplicar_operacion(&mut self, operacion: Operacion){
        match operacion { 
            Operacion::Suma(valor) => self.sumar(valor),
            Operacion::Resta(valor) => self.restar(valor),
            Operacion::Multiplicacion(valor) => self.multiplicar(valor),
            Operacion::Division(valor) => self.dividir(valor),
            _ => {}
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
enum Error {
    FaltaDireccion,
    DireccionInvalida,
    OperacionInvalida,
}
fn parsear_argumentos() -> Result<String, Error> {
    let mut entrada = std::env::args();
    entrada.next();

    let address = entrada.next().ok_or(Error::FaltaDireccion)?;
    Ok(address)
}

fn operacion_a_calculadora(palabras: Vec<&str>) -> Result<Operacion, Error>{
    if  {  }
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
        _ => Err(Error::FaltaDireccion),
    };
}

fn manejar_cliente(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let bytes = stream.read(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer[0..bytes]);
    println!("Recibido: {}",&input);
    let operacion = parsear_peticion_cliente(&input)?;
    Ok(())

}

fn crear_servidor(addres: &str) -> Result<(),io::Error>{
    let listener = TcpListener::bind(&addres)?;

    for stream in listener.incoming() {
        let stream = stream?;
        println!("Conexion establecida!");

        thread::spawn(move || {
            if let Err(e) = manejar_cliente(stream) {
                eprintln!("Error en cliente: {}", e);
            }
        });
    }
    Ok(())

}

fn main() {
    let calculadora = Calculadora::new();

    let addres = match parsear_argumentos() {
        Ok(addres) => addres,
        Err(Error::FaltaDireccion) => {
            eprintln!("Falda la dirección");
            return;
        },
        Err(Error::DireccionInvalida) => {
            eprintln!("Dirección invalida");
            return;
        },
    };

    let _ = match crear_servidor(&addres) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        },
    };




    
}