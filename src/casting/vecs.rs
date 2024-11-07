use crate::traits::casting::Casting;



/// # Casting of Vectors
/// It converts a `Vec` tipe to another one.The Datatype must implement `Casting<O,D>`. 
/// The trait is already implemented for basic datatypes.
/// 
/// ## Example:
/// 
/// ```
/// let vec = vec![2.3,0.2];
/// let casted = cast_vec::<f64,i64>(vec);
/// ```
pub fn cast_vec<O, D>(input: Vec<O>) -> Vec<D>
where
    O: Casting<O, D> + std::marker::Copy,  // Ensure that O can be cast to D
    D: Copy,           // Ensure that D can be copied (required for collecting the results)
{
    input.into_iter().map(|value| O::cast(value)).collect()
}






#[cfg(test)]
mod test{
    use super::cast_vec;


    #[test]
    fn test_cast(){
        let vec = vec![2.3,0.2];
        let casted = cast_vec::<f64,i64>(vec);
        for el in casted {
            println!("{}",el)
        }
    }
}