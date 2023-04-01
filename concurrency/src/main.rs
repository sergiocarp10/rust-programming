use std::thread;

fn main() {
    // datos de inicio
    let arreglo = vec![1, 2, 3, 4];

    // crear thread y ejecutarlo
    // es necesario indicar 'move' para transferir el ownership del arreglo
    let handler = thread::spawn(move || thread_function(&arreglo));

    // realizar acciones en main thread
    for i in 1..10 {
        println!("Iteracion {i} del main thread");
    }

    // ahora no podemos usar arreglo
    // drop(arreglo);

    // esperar por finalizacion de thread spawneado
    handler.join().unwrap();
}

fn thread_function(numeros: &[i32]) {
    for i in 0..numeros.len() {
        println!("Hilo spawneado: numeros[{i}] = {}", numeros[i]);
    }

    // acá se ejecuta internamente un drop(), liberando el ownership del arreglo
}