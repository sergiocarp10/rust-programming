fn main() {
    mutex::ejemplo();
}

mod mutex {
    use std::{sync::{Mutex, Arc}, thread};

    pub fn ejemplo(){
        // Cada hilo tomará ownership del mutex, la solución es usar Arc<T> (atomic reference count)
        let contador = Arc::new(Mutex::new(0));
        let mut manejadores = vec![];

        for _ in 1..10 {
            let referencia_mutex = Arc::clone(&contador);
            let man = thread::spawn(move || f(&referencia_mutex));
            manejadores.push(man);
        }

        // esperar a que terminen los hilos
        for man in manejadores {
            man.join().unwrap();
        }

        // imprimir valor final del contador (dereferenciar)
        println!("Contador = {}", *contador.lock().unwrap());
    }

    fn f(count: &Mutex<i32>){
        // solicitar exclusion mutua y dormise hasta obtenerlo
        let mut valor = count.lock().unwrap();

        // modificar contador con exclusion mutua
        *valor += 1;
    }
}

mod canales {
    use std::{thread, sync::mpsc::{self, Sender}, time::Duration};

    pub fn _ejemplo() {
        // Para pasaje de mensajes, creamos un canal
        // let canal = mpsc::channel::<i32>();
        
        // Como devuelve una tupla, podemos usar let (nombre1, nombre2)
        let (tx, rx) = mpsc::channel::<i32>();
    
        // Libreria mpsc: multiples productores, single consumidor
        // Entonces podemos duplicar el transmisor para otro thread
        let tx2 = tx.clone();
    
        // Creamos el primer thread con tx
        thread::spawn(|| _productor_1(tx));
    
        // Creamos el segundo thread con tx2
        thread::spawn(|| _productor_2(tx2));
    
        // Recibimos los valores en el main thread
        // Bucle finaliza cuando los transmisores son dropeados
        for valor in rx {
            println!("Main thread recibió el valor {valor}");
        }
    }
    
    fn _productor_1(tx: Sender<i32>) {
        // datos a enviar (numeros impares)
        let arreglo = vec![1, 3, 5, 7];
    
        for num in arreglo {
            tx.send(num).unwrap();      // enviar por el canal
            thread::sleep(Duration::from_secs(1))
        }
    }
    
    fn _productor_2(tx: Sender<i32>) {
        // datos a enviar (numeros pares)
        let arreglo = vec![2, 4, 8, 16];
    
        for num in arreglo {
            tx.send(num).unwrap();      // enviar por el canal
            thread::sleep(Duration::from_secs(2))
        }
    }
}

mod threads {
    use std::thread;

    pub fn _ejemplo(){
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
}

