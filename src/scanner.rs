pub struct Scanner {
    pub start: u32,
    pub current: u32,
    pub line: u32,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            line: 1,
            current: 0,
            start: 0,
        }
    }

    pub fn scan_tokens() {}
}
