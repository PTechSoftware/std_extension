use std::io;

use crate::datatypes::numbers::Number;
/// Trait to easyt cast a numeric value, a expenses of performance.
pub trait ParseNumber{
    /// # ConvertTo Method
    /// Transforms a String reference into a number. 
    /// 
    /// # Usage:
    /// ```
    ///  let cas = Convert::new("10.000,45", Number::F32(0_f32));
    ///  let salida = cas.convert_to().unwrap();
    ///  // Verificar si el valor convertido coincide con el esperado
    ///  if let Number::F32(valor) = salida {
    ///      println!("{}",valor)
    ///  } else {
    ///      panic!("La conversión no devolvió un Number::F32");
    ///  }
    /// ```
    /// 
    /// 
    /// 
    fn convert_to(&self) -> io::Result<Number>;
}