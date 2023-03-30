// generic type
struct Coordenada<T> {
    latitude: T,
    longitude: T
}

// Funciones para pedir referencias
impl<T> Coordenada<T> {
    
    fn x(&self) -> &T {
        &self.latitude
    }

    fn y(&self) -> &T {
        &self.longitude
    }
}

// Funciones exclusivas para un tipo de los genericos
impl Coordenada<f32> {

    fn distancia_centro(&self) -> f32 {
        let aux = self.x().powi(2) + self.y().powi(2);
        aux.sqrt()
    }
}

// Genericos y Traits (interfaces)
impl<T: PartialOrd> Coordenada<T> {

    fn obtener_mayor(&self) -> &T {
        if &self.x() >= &self.y() {
            self.x()
        } else {
            self.y()
        }
    }
}

fn main() {
    let punto = Coordenada{latitude: 3.0, longitude: -4.0};

    println!("Latitud: {}, longitud: {}", punto.x(), punto.y());
    println!("Distancia al centro: {}", punto.distancia_centro());                          // solo si es f32
    println!("El valor más grande entre latitud y longitud es {}", punto.obtener_mayor())   // f32 es ordenable
}
