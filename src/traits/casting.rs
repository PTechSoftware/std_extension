

//! # Casting Trait:
//! `O`: Origin
//! `D`: Destination
//! 
//! Implements the basic convertion for each datatype. Is very util to transform vecs, due now wue have a trait to control that casting.
pub trait Casting<O,D> where
    O: Copy,  // `O` debe ser un tipo numérico primitivo (como i32, f64, etc.)
    D: Copy,  // `D` también debe ser un tipo numérico primitivo
{
    fn cast(value: O) -> D;
} 

