mod test {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use tp2::*;

    #[test]
    fn test01un_cliente_se_conecta_yhace_una_suma() {
        let calculadora = Arc::new(Mutex::new(Calculadora::new()));
        let calc_clone = Arc::clone(&calculadora);
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn({
            move || {
                crear_servidor(listener, calc_clone).unwrap();
            }
        });

        let mut stream = TcpStream::connect(addr).expect("No se pudo conectar");

        stream.write_all(b"OP + 5\n").unwrap();

        let mut buffer = [0; 1024];
        let bytes = stream.read(&mut buffer).unwrap();
        let respuesta = String::from_utf8_lossy(&buffer[0..bytes]);
        assert_eq!(respuesta, "OK");
    }

    #[test]
    fn test02un_cliente_se_conecta_yhace_una_resta() {
        let calculadora = Arc::new(Mutex::new(Calculadora::new()));
        let calc_clone = Arc::clone(&calculadora);
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn({
            move || {
                crear_servidor(listener, calc_clone).unwrap();
            }
        });

        let mut stream = TcpStream::connect(addr).expect("No se pudo conectar");
        let mut buffer = [0; 1024];

        stream.write_all(b"OP + 5\n").unwrap();

        let bytes = stream.read(&mut buffer).unwrap();
        let respuesta = String::from_utf8_lossy(&buffer[0..bytes]);
        assert_eq!(respuesta, "OK");

        stream.write_all(b"OP - 2\n").unwrap();

        let bytes = stream.read(&mut buffer).unwrap();
        let respuesta = String::from_utf8_lossy(&buffer[..bytes]);
        assert_eq!(respuesta, "OK");
    }

    #[test]
    fn test03un_cliente_se_conecta_yhace_una_multiplicacion() {
        let calculadora = Arc::new(Mutex::new(Calculadora::new()));
        let calc_clone = Arc::clone(&calculadora);
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn({
            move || {
                crear_servidor(listener, calc_clone).unwrap();
            }
        });

        let mut stream = TcpStream::connect(addr).expect("No se pudo conectar");
        let mut buffer = [0; 1024];

        stream.write_all(b"OP + 5\n").unwrap();

        let bytes = stream.read(&mut buffer).unwrap();
        let respuesta = String::from_utf8_lossy(&buffer[0..bytes]);
        assert_eq!(respuesta, "OK");

        stream.write_all(b"OP * 2\n").unwrap();

        let bytes = stream.read(&mut buffer).unwrap();
        let respuesta = String::from_utf8_lossy(&buffer[..bytes]);
        assert_eq!(respuesta, "OK");
    }

    #[test]
    fn test04un_cliente_se_conecta_y_hace_una_division() -> Result<(), Box<dyn std::error::Error>> {
        let calculadora = Arc::new(Mutex::new(Calculadora::new()));
        let calc_clone = Arc::clone(&calculadora);
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn({
            move || {
                crear_servidor(listener, calc_clone).unwrap();
            }
        });

        let mut stream = TcpStream::connect(addr).expect("No se pudo conectar");
        let mut buffer = [0; 1024];

        stream.write_all(b"OP + 6\n").unwrap();

        let bytes = stream.read(&mut buffer).unwrap();
        let respuesta = String::from_utf8_lossy(&buffer[0..bytes]);
        assert_eq!(respuesta, "OK");

        stream.write_all(b"OP / 2\n").unwrap();

        let bytes = stream.read(&mut buffer).unwrap();
        let respuesta = String::from_utf8_lossy(&buffer[0..bytes]);
        assert_eq!(respuesta, "OK");

        Ok(())
    }

    #[test]
    fn test05un_cliente_se_conecta_hace_pide_el_valor() {
        let calculadora = Arc::new(Mutex::new(Calculadora::new()));
        let calc_clone = Arc::clone(&calculadora);
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();

        thread::spawn({
            move || {
                crear_servidor(listener, calc_clone).unwrap();
            }
        });

        let mut stream = TcpStream::connect(addr).expect("No se pudo conectar");

        stream.write_all(b"GET\n").unwrap();

        let mut buffer = [0; 1024];
        let bytes = stream.read(&mut buffer).unwrap();
        let respuesta = String::from_utf8_lossy(&buffer[0..bytes]);
        assert_eq!(respuesta, "0");
    }
}
