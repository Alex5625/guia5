


/* SI SE DESEA MODIFICAR LA MATRIZ, DEBE SER OBLIGATORIAMENTE CUADRADA Y MODIFICAR EL
PASO DEL CICLO FOR | TAMAÑO DE LA MATRIZ = NUMERO DE CICLOS FOR */
fn crea_matriz() -> (){
    let mut matriz: [[i32;3];3] = [[0;3];3];
    println!("{:?}",matriz); 
    
    for fila in 0..3{
        matriz[fila][fila] = 1;
    }
    println!("{:?}",matriz);
}

fn main(){
    crea_matriz()


}