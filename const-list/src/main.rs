// cabeceras
use crate::Lista::{Cons, Nil};
use std::rc::Rc;

// Sin Box no compila por ser recursivo: tamaño indeterminable
// Box utiliza la memoria heap en lugar del stack
enum Lista {
    // Para poder enlazar a otra sublista usamos Rc (reference count) en lugar de Box
    Cons(i32, Rc<Lista>),
    Nil,
}

fn main() {
    let lista_1 = Rc::new(
        Cons(2, Rc::new(
            Cons(3, Rc::new(Nil))
        ))
    );
    println!("Rc count: {}", Rc::strong_count(&lista_1));   // 1

    // usamos Rc::clone en lugar de lista.clone() ya que solo incrementa contador
    let lista_2 = Cons(1, Rc::clone(&lista_1));
    println!("Rc count: {}", Rc::strong_count(&lista_1));   // 2

    // otro scope
    {
        let lista_3 = Cons(4, Rc::clone(&lista_1));
        println!("Rc count: {}", Rc::strong_count(&lista_1));   // 3
        // muere lista 3
    }

    println!("Rc count: {}", Rc::strong_count(&lista_1));   // 2
    
}
