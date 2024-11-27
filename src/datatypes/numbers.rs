use regex::Regex;
use crate::traits::numbers_manipulate::ParseNumber;

pub enum Number {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    ISize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    F32(f32),
    F64(f64),
    USize(usize),
}

pub struct Convert {
    entrada: String,
    salida: Number,
}

impl Convert {
    pub fn new(entrada: &str, salida: Number) -> Self {
        Self {
            entrada: entrada.to_string(),
            salida,
        }
    }
}
impl ParseNumber for Convert {
    fn convert_to(&self) -> std::io::Result<Number> {
        // Detectar el último separador decimal
        let decimal_separator = self
            .entrada
            .rfind(|c| c == '.' || c == ',')
            .and_then(|idx| self.entrada.chars().nth(idx))
            .unwrap_or('.'); // Asumir punto si no hay separador

        // Normalizar la entrada con base en el separador detectado
        let normalized = match decimal_separator {
            ',' => {
                // Si el separador decimal es `,`, los puntos son separadores de miles y deben eliminarse
                let re_thousands_separator = Regex::new(r"\.").unwrap();
                re_thousands_separator.replace_all(&self.entrada, "").replace(',', ".")
            }
            '.' => {
                // Si el separador decimal es `.`, las comas son separadores de miles y deben eliminarse
                let re_thousands_separator = Regex::new(r",").unwrap();
                re_thousands_separator.replace_all(&self.entrada, "").to_string()
            }
            _ => self.entrada.clone(),
        };

        // Validar que el formato sea correcto (un único separador decimal)
        if normalized.matches('.').count() > 1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Formato inválido: múltiples separadores decimales",
            ));
        }

        // Convertir la cadena normalizada al tipo deseado
        let output = match self.salida {
            Number::I8(_) => Number::I8(normalized.parse::<i8>().unwrap_or(0_i8)),
            Number::I16(_) => Number::I16(normalized.parse::<i16>().unwrap_or(0_i16)),
            Number::I32(_) => Number::I32(normalized.parse::<i32>().unwrap_or(0_i32)),
            Number::I64(_) => Number::I64(normalized.parse::<i64>().unwrap_or(0_i64)),
            Number::I128(_) => Number::I128(normalized.parse::<i128>().unwrap_or(0_i128)),
            Number::ISize(_) => Number::ISize(normalized.parse::<isize>().unwrap_or(0_isize)),
            Number::U8(_) => Number::U8(normalized.parse::<u8>().unwrap_or(0_u8)),
            Number::U16(_) => Number::U16(normalized.parse::<u16>().unwrap_or(0_u16)),
            Number::U32(_) => Number::U32(normalized.parse::<u32>().unwrap_or(0_u32)),
            Number::U64(_) => Number::U64(normalized.parse::<u64>().unwrap_or(0_u64)),
            Number::U128(_) => Number::U128(normalized.parse::<u128>().unwrap_or(0_u128)),
            Number::USize(_) => Number::USize(normalized.parse::<usize>().unwrap_or(0_usize)),
            Number::F32(_) => Number::F32(normalized.parse::<f32>().unwrap_or(0_f32)),
            Number::F64(_) => Number::F64(normalized.parse::<f64>().unwrap_or(0_f64)),
        };

        Ok(output)
    }
}

#[cfg(test)]
mod test {
    use crate::traits::numbers_manipulate::ParseNumber;

    use super::{Convert, Number};

    #[test]
    fn cast_test_1() {
        let cas = Convert::new("10.000,45", Number::F32(0_f32));
        let salida = cas.convert_to().unwrap();

        // Verificar si el valor convertido coincide con el esperado
        if let Number::F32(valor) = salida {
            println!("{}",valor)
        } else {
            panic!("La conversión no devolvió un Number::F32");
        }
    }

    #[test]
    fn cast_test_2() {
        let cas = Convert::new("1,234.56", Number::F64(0_f64));
        let salida = cas.convert_to().unwrap();

        if let Number::F64(valor) = salida {
            println!("{}",valor)
        } else {
            panic!("La conversión no devolvió un Number::F64");
        }
    }

    #[test]
    fn cast_test_3() {
        let cas = Convert::new("123,456", Number::F32(0_f32));
        let salida = cas.convert_to().unwrap();

        if let Number::F32(valor) = salida {
            println!("{}",valor)
        } else {
            panic!("La conversión no devolvió un Number::I32");
        }
    }
}
