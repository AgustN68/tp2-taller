use std::{io, thread};
use std::io::Read;
use std::net::*;
use std::sync::{Arc, Mutex};
use crate::Error::OperacionInvalida;

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
    pub fn aplicar_operacion(&mut self, operacion: Operacion) -> Result<(), Error>{
        match operacion { 
            Operacion::Suma(valor) => {
                self.sumar(valor);
                Ok(())
            },
            Operacion::Resta(valor) => {
                self.restar(valor);
                Ok(())
            },
            Operacion::Multiplicacion(valor) => {
                self.multiplicar(valor);
                Ok(())
            },
            Operacion::Division(valor) => {
                if valor == 0 {
                    return Err(Error::DivisionPorCero);
                }
                self.dividir(valor);
                Ok(())
            },
            Operacion::Get() => {

                Ok(()) },
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
    DivisionPorCero,
}
fn parsear_argumentos() -> Result<String, Error> {
    let mut entrada = std::env::args();
    entrada.next();

    let address = entrada.next().ok_or(Error::FaltaDireccion)?;
    Ok(address)
}

fn operacion_a_calculadora(palabras: Vec<&str>) -> Result<Operacion, Error>{
    if palabras.len() < 3 {
        return Err(OperacionInvalida)
    }
    let operador = palabras[1];
    let valor: u8 = palabras[2].parse().map_err(|_| OperacionInvalida)?;

    match operador {
        "+" => Ok(Operacion::Suma(valor)),
        "-" => Ok(Operacion::Resta(valor)),
        "*" => Ok(Operacion::Multiplicacion(valor)),
        "/" => Ok(Operacion::Division(valor)),
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
        _ => Err(Error::FaltaDireccion),
    }
}

fn manejar_cliente(mut stream: TcpStream, calculadora: Arc<Mutex<Calculadora>>) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let bytes = stream.read(&mut buffer)?;
    let input = String::from_utf8_lossy(&buffer[0..bytes]);
    println!("Recibido: {}",&input);
    let operacion: Operacion = match parsear_peticion_cliente(&input){
        Err(_) => eprintln!("Error al parsear operacion"),

        Ok(op) => op,
    };

    let mut calc = match calculadora.lock() {
        Ok(calc) => calc,
        Err(e) => {
            eprintln!("Error al lockear calculadora: {:?}", e);
            return Ok(());
        }
    };
    calc.aplicar_operacion(operacion);
    Ok(())

}

fn crear_servidor(addres: &str, calculadora: Arc<Mutex<Calculadora>>) -> Result<(),io::Error>{

    let listener = TcpListener::bind(&addres)?;

    for stream in listener.incoming() {
        let stream = stream?;
        let calc = Arc::clone(&calculadora);
        println!("Conexion establecida!");

        thread::spawn(move || {
            if let Err(e) = manejar_cliente(stream, calc) {
                eprintln!("Error en cliente: {}", e);
            }
        });
    }
    Ok(())

}

fn main() {

    let calculadora = Arc::new(Mutex::new(Calculadora::new()));

    let addres = match parsear_argumentos() {
        Err(Error::FaltaDireccion) => {
            eprintln!("Falta la dirección");
            return;
        },
        Err(Error::DireccionInvalida) => {
            eprintln!("Dirección invalida");
            return;
        },
        Ok(addres) => addres,
        Err(Error::OperacionInvalida) | Err(Error::DivisionPorCero) => todo!(),
    };

    let _ = match crear_servidor(&addres, calculadora) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {e}");
            return;
        },
    };




    
}