use std::io::Read;
use std::{env, fs::File};
use std::process;

// Estructuras
struct Params {
    query: String,
    file_path: String
}

// Métodos de la estructura
impl Params {

    // Convención general: las funciones new() NO deben generar errores

    // El parseo de argumentos es inherente al tipo Params (evitamos escribirlo en main)
    // Devuelve error si se pasan menos de 3 parámetros (thisPath, query, filePath)
    fn from(args: &[String]) -> Result<Params, &'static str> {
        if args.len() < 3 {
            // si no es la ultima linea de la función debo usar return valor + semicolon
            return Err("Se requieren 2 parametros: query, file_path");
        }

        let object = Params { query: args[1].clone(), file_path: args[2].clone() };
        Ok(object)
    }
}

fn main() {
    // Obtenemos argumentos pasados al programa
    // Importante: en la posición 0 está el path del archivo actual (similar a C)
    let args: Vec<String> = env::args().collect();

    // Extraemos los que nos interesan
    // Solo los vamos a leer, así que pasamos referencia
    let params = Params::from(&args).unwrap_or_else(|err| {
        // Se debe devolver el tipo Params o bien forzar un process:exit()
        println!("Error: {err}");
        process::exit(1);
    });

    // Informamos argumentos leidos
    println!("Query: {}", params.query);
    println!("File Path: {}", params.file_path);

    // Abrimos el archivo
    // Para evitar evaluar el Result usamos expect para informar error si corresponde
    let mut file = File::open(params.file_path).expect("No se puede abrir el archivo!");

    // Buscamos el substring en cada linea del archivo
    // let mut results: Vec<&str> = Vec::new();
    let mut content = String::new();

    file.read_to_string(&mut content).expect("No se puede leer...");

    /*
    for line in content.lines() {
        if line.contains(&params.query) {
            results.push(line);
        }
    } */

    let results: Vec<&str> = content.lines()
        .filter(|l| l.contains(&params.query))
        .collect();

    // Mostrar resultados
    for line_matched in results {
        println!("{line_matched}");
    }

}

// Hint: usar [Tipo] es lo mismo que Vec<Tipo>
// fn parse_args(args: &[String]) -> Params { }