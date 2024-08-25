pub struct Scanner {
    pub start: usize,
    pub current: usize,
    pub line: usize,
    pub source: String,
}

impl Scanner {
    pub fn new(source: &String) -> Self {
        Self {
            line: 1,
            current: 0,
            start: 0,
            source: source.clone(),
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => println!("LEFT_PAREN"),
            ')' => println!("RIGHT_PAREN"),
            '{' => println!("LEFT_BRACE"),
            '}' => println!("RIGHT_BRACE"),
            ',' => println!("COMMA"),
            '.' => println!("DOT"),
            '-' => println!("MINUS"),
            '+' => println!("PLUS"),
            ';' => println!("SEMICOLON"),
            '*' => println!("STAR"),
            _ => println!("Unexpected character"),
        }
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
