use std::arch::x86_64::_SIDD_LEAST_SIGNIFICANT;

use regex::Regex;
fn main() {
// Regex
let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

// Traer Datos del usuario
println!("Introduce tu expresion:");
let mut expression = String::new();
std::io::stdin().read_line(&mut expression).unwrap();
// Aplicar operaciones
let caps = re_add.captures(expression.as_str()).unwrap();
let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
println!("{:?}", caps);
//Mostrar resultados
println!("Resultado: {}", expression);
}