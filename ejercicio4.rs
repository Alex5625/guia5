/*esto puede usar matrices de cualquier tamaño pero con la condicion de que el tamaño de esta sea
igual a la cantidad de pasos del for */

fn matrices_comparacion(matriz1:[[i32;3];3],matriz2:[[i32;3];3])-> bool{

    for fila in 0..3{
        for columna in 0..3{
            if matriz1[fila][columna] == matriz2[fila][columna]{
                continue
            } else {
                return false;
            }
        }
    }
    return true;
}




fn matriz_transpuesta(){

    let matriz:[[i32;3];3] = [[0;3];3];
    println!("la matriz original es: {:?}",matriz);

    let mut transpuesta: [[i32;3];3] = [[0;3];3];

    for fila in 0..3{
        for columna in 0..3{
            transpuesta[fila][columna] = matriz[columna][fila];
        }
    }
    println!("la matriz traNSPUESTA ES: {:?}",transpuesta);
    let check:bool = matrices_comparacion(matriz,transpuesta);

    if check {
        println!("la matriz {:?} es simetrica",matriz)
    } else {
        println!("la matriz {:?} NO es simetrica",matriz)
    }
}

fn main(){
    matriz_transpuesta();
}

