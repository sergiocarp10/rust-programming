# rust-programming
The Rust Programming Language

https://doc.rust-lang.org/book

## Keywords
* as: utilizado luego de ```use + ruta al módulo o función``` para renombrarlo y evitar colisión de nombre (cap. 7.4)
* pub use: habilita el uso de ese módulo o función también en los módulos hijos, sino solo es usable en el mismo (cap 7.4)

## use
Para importar módulos o alguno de sus elementos. Para evitar escribir muchas líneas con el mismo prefijo se puede usar llaves.

### Ejemplo 1
```
use std::cmp::Ordering;
use std::io;
```

es lo mismo que ``` use std::{cmp::Ordering, io}; ```

### Ejemplo 2
```
use std::io;
use std::io::Write;
```

es lo mismo que ```use std::io::{self, Write};```
