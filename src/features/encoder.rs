

pub trait Encoding {
    fn to_base64(&self) -> String;
}

impl Encoding for String {
    fn to_base64(&self) -> String {
        base64_encode(self.as_bytes())
    }
}

impl Encoding for &str {
    fn to_base64(&self) -> String {
        base64_encode(self.as_bytes())
    }
}

// Helper function for Base64 encoding
fn base64_encode(input: &[u8]) -> String {
    // Base64 character set
    const BASE64_TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut encoded = Vec::new();
    let mut padding = 0;

    // Process each 3-byte block
    for chunk in input.chunks(3) {
        let mut buffer = 0u32;
        for &byte in chunk {
            buffer = (buffer << 8) | byte as u32;
        }

        // Calculate the number of padding characters
        padding = (3 - chunk.len()) % 3;

        // Shift the buffer and map to Base64
        for i in (0..4 - padding).rev() {
            let index = ((buffer >> (i * 6)) & 0x3F) as usize;
            encoded.push(BASE64_TABLE[index]);
        }
    }

    // Add '=' padding for alignment
    for _ in 0..padding {
        encoded.push(b'=');
    }

    // Convert to String and return
    String::from_utf8(encoded).expect("Base64 encoding produced invalid UTF-8")
}
