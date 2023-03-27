// cabecera requerida para HashMap
use std::collections::HashMap;

fn main() {
    let some_vector = vec![1, 2, 2, 4, 5, 7, 8, 8, 9];
    let median = ex_median(&some_vector);
    let mode = ex_mode(&some_vector);

    println!("median is {}", median);
    println!("mode is {}", mode);
}

// No vamos a alterar el vector, así que usamos referencia
fn ex_median(sorted_vec: &Vec<i32>) -> i32 {
    let len = sorted_vec.len();

    if len % 2 == 0 {
        // si la cantidad de elementos es par...
        // debo calcular promedio entre los dos del medio
        // ejemplo: if len == 6, median is (v[2] + v[3]) / 2 = (v[4/2] + v[6/2]) / 2
        let left = sorted_vec[(len-2) / 2];
        let right = sorted_vec[len / 2];
        return (left + right) / 2;
    } else {
        // si es impar, está justo en el medio
        // ejemplo: if len == 5, median is v[4/2]=v[2]
        return sorted_vec[(len-1) / 2];
    }
}

// No vamos a alterar el vector, así que usamos referencia
fn ex_mode(vec: &Vec<i32>) -> i32 {
    // lo inicializo como mutable porque cambiará durante esta función
    let mut aux: HashMap<i32, u32> = HashMap::new();

    for elem in vec {
        // si no está, crear con valor 0
        let entry = aux.entry(*elem).or_insert(0);

        // incrementar siempre (accion comun)
        // debo usar * porque or_insert nos devuelve referencia
        *entry += 1;
    }

    // ahora buscamos el máximo en el HashMap
    let mut max_value: u32 = 0;
    let mut max_key = -1;

    // acá tomo ownership del HashMap porque no lo voy a utilizar despues
    for (key, value) in aux {
        if value > max_value {
            max_value = value;
            max_key = key;
        }
    }

    return max_key;
}