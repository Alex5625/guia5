use std::io;

fn texto_numero() -> i32 {
    loop {
        println!("Ingrese un número: ");
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



fn producto_escalar(matriz:[[i32;2];3]) -> (){
    
    println!("por que numero quieres multiplicar la matriz");
    let numero = texto_numero();



    let mut matriz_modificada = matriz;

    for (i, row) in matriz.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            matriz_modificada[i][j] = numero * matriz[i][j];
        }
    }
    println!("{:?}", matriz_modificada);
}

fn main(){
    let matriz:[[i32;2];3] = [[1;2];3];
    println!("matriz original: {:?}", matriz);
    producto_escalar(matriz);
}