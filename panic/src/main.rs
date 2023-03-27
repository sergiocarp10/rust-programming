use std::{fs::File, io::{self, Read}};

fn main() {
    // panic!("what the helllll");

    // let vec = vec![1, 2, 3, 4];
    // vec[99];

    // abrir un archivo inexistente no produce panic!, sino Result
    let my_file = File::open("hola_mundo.txt");

    match my_file {
        Ok(file) => println!("File exists! {:?}", file),
        Err(err) => println!("Error: {:?}", err)
    };

    // Para evitar el uso el match y luego asignar la variable si dio Ok, usamos expect
    // que además agrega un mensaje personalizado antes de imprimir el error

    // let second_file = File::open("two.txt").expect("File not found. More details");
    // println!("File exists: {:?}", second_file);

    let filename = String::from("three.txt");
    let primera_linea = read_first_line(&filename).expect("No se pudo leer la primera linea. Detalles");
    println!("Primera linea: {}", primera_linea);

    let ultimo = extract_line_last_char(&primera_linea);
    if ultimo.is_some() {
        println!("Ultimo caracter: {}", ultimo.unwrap());
    } 

}

fn read_first_line(filename: &String) -> Result<String, io::Error> {
    // Si queremos que la función llamante se encargue del error, podemos usar ? para retornarlo
    // ATENCION: '?' solo se puede usar dentro de FUNCIONES, porque adentro tiene un return
    let mut first_line = String::new();

    // let mut opened_file = File::open(filename)?;    // necesito mut para poder recorrerlo
    // opened_file.read_to_string(&mut first_line)?;   // si falla se devuelve error

    // forma mas corta (usando ?. como en Kotlin)
    File::open(filename)?.read_to_string(&mut first_line)?;

    // como el tipo de retorno es Result, debo devolver objeto Ok(String)
    return Ok(first_line);
}

// tambien podemos usar ? para retornar con Option
fn extract_line_last_char(text: &String) -> Option<char> {
   
    // necesariamente se debe retornar el tipo char al final o None en el medio
    return text.lines().next()?.chars().last();
}