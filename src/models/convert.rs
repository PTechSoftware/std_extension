use super::number::Number;

pub struct Convert {
    pub entrada: String,
    pub salida: Number,
}

 impl Convert {
    pub fn new(entrada: &str, salida: Number) -> Self {
        Self {
            entrada: entrada.to_string(),
            salida,
        }
    }
}