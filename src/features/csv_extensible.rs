use crate::traits::str_manipulate::StrManipulate;

pub trait LineCSVParse {
    fn line_parse(&self, raw_line: &str, output: &mut Vec<String>, delimiter: &str);
}

impl LineCSVParse for &str {
    fn line_parse(&self, raw_line: &str, output: &mut Vec<String>, delimiter: &str) {
        output.clear();

        for part in raw_line.split(delimiter) {
            output.push(part.trim().to_string()); 
        }
    }
}

impl LineCSVParse for String {
    fn line_parse(&self, raw_line: &str, output: &mut Vec<String>, delimiter: &str) {
        output.clear();
        for part in raw_line.split(delimiter){
            output.push(part.to_string())
        }
    }
}