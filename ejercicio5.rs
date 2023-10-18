use std::io;

fn texto_numero() -> i32 {
    loop {
        //println!("Ingrese un número: ");
        let mut numero = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut numero).unwrap();
        let numero: i32 = match numero.trim().parse(){
            Ok(numero) => numero,
            Err(_) => {
                println!("Error, no es un número");
                continue;
            },
        };
        return numero;
    }
}

fn crea_matriz_numeros() -> [[i32; 2];2]{

    let mut matriz: [[i32; 2];2] = [[0; 2];2];
    
    println!("Esta matriz sera cuadrada de 2x2");

    for i in 0..4{
        println!("Ingrese el número que desea insertar:");
        let numero = texto_numero();
        println!("Ingrese la fila donde desea insertar el número:");
        let fila = texto_numero() as usize;
        println!("Ingrese la columna donde desea insertar el número:");
        let columna = texto_numero() as usize;
        matriz[fila][columna] = numero;
    }
    println!("La matriz sera la siguiente: {:?}",matriz);
    return matriz;
}

fn rotar_matriz(matriz:[[i32;2];2]){

    let mut matriz_rotada: [[i32; 2]; 2] = [[0; 2]; 2];
    for fila in 0..2 {
        for columna in 0..2 {
            matriz_rotada[fila][columna] = matriz[1 - columna][fila];
        }
    }
    println!("la matriz rotada en 90 grados en sentido horario es: {:?}",matriz_rotada);

}


fn main(){
    let matriz = crea_matriz_numeros();
    rotar_matriz(matriz);
}