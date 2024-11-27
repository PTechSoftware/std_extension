
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

impl Number {
    pub fn is_float(&self) -> bool{
        match &self {
            Number::I8(_) => false,
            Number::I16(_) => false,
            Number::I32(_) => false,
            Number::I64(_) => false,
            Number::I128(_) => false,
            Number::ISize(_) => false,
            Number::U8(_) => false,
            Number::U16(_) => false,
            Number::U32(_) => false,
            Number::U64(_) => false,
            Number::U128(_) => false,
            Number::F32(_) => true,
            Number::F64(_) => true,
            Number::USize(_) => false,
        }
    }
}