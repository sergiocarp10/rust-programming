use std::{thread, sync::mpsc::{self, Sender}, time::Duration};

fn main() {
    // Para pasaje de mensajes, creamos un canal
    // let canal = mpsc::channel::<i32>();
    
    // Como devuelve una tupla, podemos usar let (nombre1, nombre2)
    let (tx, rx) = mpsc::channel::<i32>();

    // Libreria mpsc: multiples productores, single consumidor
    // Entonces podemos duplicar el transmisor para otro thread
    let tx2 = tx.clone();

    // Creamos el primer thread con tx
    thread::spawn(|| productor_1(tx));

    // Creamos el segundo thread con tx2
    thread::spawn(|| productor_2(tx2));

    // Recibimos los valores en el main thread
    // Bucle finaliza cuando los transmisores son dropeados
    for valor in rx {
        println!("Main thread recibió el valor {valor}");
    }

}

fn productor_1(tx: Sender<i32>) {
    // datos a enviar (numeros impares)
    let arreglo = vec![1, 3, 5, 7];

    for num in arreglo {
        tx.send(num).unwrap();      // enviar por el canal
        thread::sleep(Duration::from_secs(1))
    }
}

fn productor_2(tx: Sender<i32>) {
    // datos a enviar (numeros pares)
    let arreglo = vec![2, 4, 8, 16];

    for num in arreglo {
        tx.send(num).unwrap();      // enviar por el canal
        thread::sleep(Duration::from_secs(2))
    }
}

fn _ejemplo_threads(){
    // datos de inicio
    let arreglo = vec![1, 2, 3, 4];

    // crear thread y ejecutarlo
    // es necesario indicar 'move' para transferir el ownership del arreglo
    let handler = thread::spawn(move || _imprimir(&arreglo));

    // realizar acciones en main thread
    for i in 1..10 {
        println!("Iteracion {i} del main thread");
    }

    // ahora no podemos usar arreglo
    // drop(arreglo);

    // esperar por finalizacion de thread spawneado
    handler.join().unwrap();
}

fn _imprimir(numeros: &[i32]) {
    for i in 0..numeros.len() {
        println!("Hilo spawneado: numeros[{i}] = {}", numeros[i]);
    }

    // acá se ejecuta internamente un drop(), liberando el ownership del arreglo
}