fn main() {
    //ciclo loop de dos numeros a sumar
    let  numero_1 = 123;
    let numero_2 = 456;
    let suma = numero_1 + numero_2;
    println!("por favor, escribir la suma entre {} y {}", numero_1, numero_2);

    //obtener del usuario el numero a sumar
    loop {
        let mut suma_usuario = String::new();
        std::io::stdin()
            .read_line(&mut suma_usuario)
            .expect("Fallo al leer la entrada");
        let suma_usuario_int: i32 = suma_usuario.trim().parse().unwrap();
        if suma_usuario_int == suma{
            println!("muy bien, el resultado es: {} y es correcto", suma);
            break;
        } else {
            println!("el resultado es: {} y es incorrecto", suma_usuario_int);
        }
    }

}