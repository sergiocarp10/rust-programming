# rust-programming
The Rust Programming Language

https://doc.rust-lang.org/book

# Collections

## Vectores
Se pueden instanciar vacíos mediante ```Vec::new()```, o bien con valores iniciales mediante la macro ```vec![v1, v2, v3...]```. Sus elementos se ubican de forma contigua en memoria. 

Para obtener elementos se puede usar el índice de toda la vida (que causa una excepción si se referencia a una posición fuera de rango) o el método ```get(pos)``` que devuelve Option<> para manejar el posible error.

Para agregar elementos, se usa el método ```push(elem)```. Se debe tener en cuenta que esto puede provocar que todo el vector deba re-localizarse a una porción de memoria más grande. Por tal motivo, no puede haber referencias inmutables a posiciones del vector previo al push.


# Keywords
* as: utilizado luego de ```use + ruta al módulo o función``` para renombrarlo y evitar colisión de nombre (cap. 7.4)

## use
Para importar módulos o alguno de sus elementos. 

### pub use
Permite la exportación de submódulos públicos para que otro código externo no necesite especificar los nombres intermedios que haya entre el nombre del crate y el submódulo a utilizar (cap 7.4). En el ejemplo 7.17 mostrado a continuación, código parte del crate ```restaurant```, se puede usar el keyword ```pub use``` para que el submódulo ```hosting``` (público) del módulo ```front_of_house``` (privado) se pueda utilizar como ```restaurant::hosting``` en lugar de ```restaurant::front_of_house::hosting```.

```
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

### use con llaves
Para evitar escribir muchas líneas con el mismo prefijo se puede usar llaves.

#### Ejemplo 1
```
use std::cmp::Ordering;
use std::io;
```

es lo mismo que ``` use std::{cmp::Ordering, io}; ```

#### Ejemplo 2
```
use std::io;
use std::io::Write;
```

es lo mismo que ```use std::io::{self, Write};```
