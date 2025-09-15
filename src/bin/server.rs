use std::io;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use tp2::*;

fn main() -> Result<(), io::Error> {
    let calculadora = Arc::new(Mutex::new(Calculadora::new()));

    let addres = match parsear_argumentos() {
        Err(Error::FaltaDireccion) => {
            eprintln!("Falta la dirección");
            return Ok(());
        }
        Err(Error::OperacionInvalida)
        | Err(Error::DivisionPorCero)
        | Err(Error::MensajeInesperado) => todo!(),
        Ok(addres) => addres,
    };
    let listener = TcpListener::bind(addres)?;
    match crear_servidor(listener, calculadora) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error: {e}");
        }
    };
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_suma() {
        let mut calc = Calculadora::new();
        calc.aplicar_operacion(Operacion::Suma(5)).unwrap();
        assert_eq!(calc.valor(), 5);
    }

    #[test]
    fn test_resta() {
        let mut calc = Calculadora::new();
        calc.aplicar_operacion(Operacion::Suma(10)).unwrap();
        calc.aplicar_operacion(Operacion::Resta(3)).unwrap();
        assert_eq!(calc.valor(), 7);
    }

    #[test]
    fn test_division() {
        let mut calc = Calculadora::new();
        calc.aplicar_operacion(Operacion::Suma(10)).unwrap();
        calc.aplicar_operacion(Operacion::Division(2)).unwrap();
        assert_eq!(calc.valor(), 5);
    }
    #[test]
    fn test_multiplicacion() {
        let mut calc = Calculadora::new();
        calc.aplicar_operacion(Operacion::Suma(4)).unwrap();
        calc.aplicar_operacion(Operacion::Multiplicacion(2))
            .unwrap();
        assert_eq!(calc.valor(), 8);
    }
}
