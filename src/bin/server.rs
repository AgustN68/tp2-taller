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
fn main() {
    let calculadora = Calculadora::new();
    
    let operacion = "+ 5";
    
    println!("{operacion}");
    
}