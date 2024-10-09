fn main() {
    // if true{
    //     println!("hola mundo");
    // } else {
    //     println!("adios mundo");
    // }
    
    // obtener la edad de la consola
    println!("Que edad tienes?:");
    let mut edad: String = String::new();
    std::io::stdin().read_line(&mut edad).unwrap();
    
    //convertir esa edad a un numero
    let edad_int: u8 = edad.trim().parse().unwrap();
    if edad_int >= 18 && edad_int != 30{
        println!("puedes entrar a la disco.");
    }
    else if edad_int == 30{
        println!("Lo siento, no puedes entrar, SE PROHIBE EXCLUSIVAMENTE LA ENTRADA A PERSONAS DE 30.");
    }
    else{
        println!("no puedes entrar");
    }
        println!("tienes {} anios", edad_int);
}