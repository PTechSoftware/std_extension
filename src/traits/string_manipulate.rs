use std::usize;




pub trait StringManipulate {
    fn split_string(&self, delimiter: &str , index: usize ) -> Vec::<String>;
    fn substring_replace(&mut self, start: usize, length: usize);
    fn remove(&self, char:&str)-> String;
}