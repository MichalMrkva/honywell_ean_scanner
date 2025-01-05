pub fn parse_sequence(len: usize, buf: &[u8]) -> String {
    let mut code = String::new();
    for i in 0..len {
        let char = buf[i] as char;
        match char {
            '\r' | '\n' => break,
            _ => {}
        }
        code.push(char);
    }
    code
}

#[derive(Debug)]
pub struct EANCode {
    pub prefix: String,
    pub code: String,
}

impl EANCode {
    pub fn new(prefix: String, code: String) -> Option<Self> {
        if prefix.is_empty() || code.is_empty() {
            None
        } else {
            Some(Self { code, prefix })
        }
    }
}

impl ToString for EANCode {
    fn to_string(&self) -> String {
        format!("Prefix: {}\nCode: {}", self.prefix, self.code)
    }
}
