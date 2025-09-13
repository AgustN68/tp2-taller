use std::sync::{Arc, Mutex};
use tp2::*;

fn main() {
    let calculadora = Arc::new(Mutex::new(Calculadora::new()));

    let addres = match parsear_argumentos() {
        Err(Error::FaltaDireccion) => {
            eprintln!("Falta la dirección");
            return;
        }
        Err(tp2::Error::OperacionInvalida) | Err(tp2::Error::DivisionPorCero) => todo!(),
        Ok(addres) => addres,
    };

    match crear_servidor(&addres, calculadora) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {e}");
        }
    };
}
