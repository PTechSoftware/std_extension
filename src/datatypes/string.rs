use crate::traits::string_manipulate::StringManipulate;


impl StringManipulate for String {

    /// # Split String
    /// * Splits the string provided ein a new `Vec<String>` 
    /// 
    /// ## Parameters:
    /// `&self`: The variable.
    /// `&str`: The delimiter used to split the *`String`*.
    /// `usize`: The position expected for the delimiter. If the delimitaer have more than one refererence , you can set the position desired.
    /// 
    /// ## Returns:
    /// `Vec<String>`: A splitted
    /// 
    /// ## Example:
    /// ```
    /// let data = String::from_str("Hello word, this is Rust, my friend.").unwrap();
    /// let split = &data.split_string(",", 2);
    /// let second = split.get(1).unwrap();
    /// ```
    fn split_string(&self, delimiter: &str, index: usize) -> Vec<String> {
        let mut parts: Vec<String> = self.split(delimiter).map(String::from).collect();
        if parts.len() > index {
            parts.truncate(index + 1);
        }
        parts
    }
    
    /// # SubString
    /// * Set in actual variable the calculated substring.
    /// 
    /// ## Parameters: 
    /// `&mut self`: The string mutable element.
    /// `usize start`: The index where the substring starts to extract.
    /// `usize lenght`: The count of positions it takes.
    ///  
    /// ## Returns
    /// `()` : Is a void function, it make the changes in the current variable.
    /// 
    /// ## Example:
    /// ```
    /// let mut data = String::from_str("Hello word, this is Rust, my friend.").unwrap();
    /// data.substring_replace(0, 5);
    /// ```
    fn substring_replace(&mut self, start: usize, length: usize) {
        let start = usize::min(start, self.len());
        let end = usize::min(start + length, self.len());
        let new_content = self[start..end].to_string();
        *self = new_content;
    }
    
    fn remove(&self, char:&str)-> String {
        self.replace(char, "")
    }

    
    
}





#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::traits::string_manipulate::StringManipulate;


    #[test]
    fn test_split_string(){
        let data = String::from_str("Hello word, this is Rust, my friend.").unwrap();
        let split = &data.split_string(",", 2);
        let second = split.get(1).unwrap();
        println!("{second}")
    }
    #[test]
    fn test_substring(){
        let mut data = String::from_str("Hello word, this is Rust, my friend.").unwrap();
        data.substring_replace(0, 5);
        println!("{data}")
    }
}
