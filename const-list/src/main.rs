// cabeceras
use crate::Lista::{Cons, Nil};

// Sin Box no compila por ser recursivo: tamaño indeterminable
// Box utiliza la memoria heap en lugar del stack
enum Lista {
    Cons(i32, Box<Lista>),
    Nil,
}

fn main() {
    let mi_lista = Cons(1, Box::new(
        Cons(2, Box::new(
            Cons(3, Box::new(Nil))
        ))
    ));
}
