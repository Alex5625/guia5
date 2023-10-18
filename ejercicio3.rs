/*esto puede usar matrices de cualquier tamaño pero con la condicion de que el tamaño de esta sea
igual a la cantidad de pasos del for */
fn matriz_transpuesta(){

    let matriz:[[i32;3];3] = [[1,2,3],[4,5,10],[20,12,3]];
    println!("la matriz original es: {:?}",matriz);

    let mut transpuesta: [[i32;3];3] = [[0;3];3];

    for fila in 0..3{
        for columna in 0..3{
            //traspaso de valores hacia la nueva matriz
            transpuesta[fila][columna] = matriz[columna][fila];
        }

    }

    println!("la matriz traNSPUESTA ES: {:?}",transpuesta);

}

fn main(){
    matriz_transpuesta();
}


