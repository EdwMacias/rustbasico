fn main() {
    let mut edad: u8 = 25;
    let temperatura: i8 = -3;
    let nombre: &str = "Edwar";
    edad = edad + 1;
    println!("Hola {}!", nombre);
    println!("tengo {} anios", edad);
    println!("hizo mucho frio, fueron {} grados", temperatura);
    
    // obtener la edad de la consola
    println!("Que edad tienes?:");
    let mut edad2: String = String::new();
    std::io::stdin().read_line(&mut edad2).unwrap();
    
    //convertir esa edad a un numero
    let edad_int: u8 = edad2.trim().parse().unwrap();
    println!("tienes {} anios", edad_int);

}
 