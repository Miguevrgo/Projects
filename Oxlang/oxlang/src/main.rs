#[derive(Debug)]
struct Lexer {
    input: String,
    pos: usize,
    temp_counter: usize,
}

impl Lexer {
    fn new() -> Self {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read input");
        Lexer {
            input: buffer.trim().to_string(),
            pos: 0,
            temp_counter: 0,
        }
    }

    fn look(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn get_char(&mut self) {
        if let Some(c) = self.look() {
            self.pos += c.len_utf8();
        }
    }

    fn expected(&self, what: &str) -> ! {
        eprintln!("\x07Error: Expected :{}", what);
        std::process::exit(1);
    }

    fn match_char(&mut self, expected: char) {
        match self.look() {
            Some(c) if c == expected => self.get_char(),
            _ => self.expected(&format!("'{}'", expected)),
        }
    }

    fn get_num(&mut self) -> String {
        match self.look() {
            Some(c) if c.is_ascii_digit() => {
                let digit = c.to_string();
                self.get_char();
                digit
            }
            _ => self.expected("Digit"),
        }
    }

    fn fresh_temp(&mut self) -> String {
        let name = format!("%t{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn emit(&self, msg: &str) {
        println!("\t{}", msg);
    }

    fn term(&mut self) -> String {
        self.get_num()
    }

    fn add(&mut self, left: String) -> String {
        self.match_char('+');
        let right = self.term();
        let result = self.fresh_temp();
        self.emit(&format!("{result} = add i32 {left}, {right}"));
        result
    }

    fn sub(&mut self, left: String) -> String {
        self.match_char('-');
        let right = self.term();
        let result = self.fresh_temp();
        self.emit(&format!("{result} = sub i32 {left}, {right}"));
        result
    }

    fn expression(&mut self) -> String {
        let left = self.term();
        match self.look() {
            Some('+') => self.add(left),
            Some('-') => self.sub(left),
            Some('_') => self.expected("Operator"),
            None => left,
            _ => unreachable!(),
        }
    }
}

fn main() {
    println!("define i32 @main() {{");
    println!("entry:");

    let mut lexer = Lexer::new();
    let result = lexer.expression();

    println!("  ret i32 {}", result);
    println!("}}");
}
