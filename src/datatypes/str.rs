use std::str::FromStr;
use crate::traits::str_manipulate::StrManipulate;

impl StrManipulate for &str {
    fn to_string(&self) -> String {
        String::from_str(self).unwrap_or_else(|_| String::new())
    }
}