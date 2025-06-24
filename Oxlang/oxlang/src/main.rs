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
        eprintln!("\x07Error: Expected: {}", what);
        std::process::exit(1);
    }

    fn skip_whitespace(&mut self) {
        while self.look().is_some_and(|c| c.is_ascii_whitespace()) {
            self.get_char();
        }
    }

    fn match_char(&mut self, expected: char) {
        match self.look() {
            Some(c) if c == expected => self.get_char(),
            _ => self.expected(&format!("'{}'", expected)),
        }
    }

    fn get_num(&mut self) -> String {
        self.skip_whitespace();
        let mut num = String::new();
        while let Some(c) = self.look() {
            if c.is_ascii_digit() {
                num.push(c);
                self.get_char();
            } else {
                break;
            }
        }

        if num.is_empty() {
            self.expected("Digit");
        } else {
            num
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

    fn factor(&mut self) -> String {
        self.skip_whitespace();
        if self.look() == Some('(') {
            self.match_char('(');
            let result = self.expression();
            self.match_char(')');

            result
        } else {
            self.get_num()
        }
    }

    fn term(&mut self) -> String {
        let mut left = self.factor();
        loop {
            self.skip_whitespace();
            match self.look() {
                Some('*') => left = self.mul(left),
                Some('/') => left = self.div(left),
                _ => break,
            }
        }

        left
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

    fn mul(&mut self, left: String) -> String {
        self.match_char('*');
        let right = self.factor();
        let result = self.fresh_temp();
        self.emit(&format!("{result} = mul i32 {left}, {right}"));
        result
    }

    fn div(&mut self, left: String) -> String {
        self.match_char('/');
        let right = self.factor();
        let result = self.fresh_temp();
        if right == "0" {
            self.expected("Non-zero divisor");
        };
        self.emit(&format!("{result} = sdiv i32 {left}, {right}"));
        result
    }

    fn expression(&mut self) -> String {
        let mut left = self.term();

        while let Some(op) = self.look() {
            match op {
                '+' => {
                    left = self.add(left);
                }
                '-' => {
                    left = self.sub(left);
                }
                _ => break,
            }
        }

        left
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
