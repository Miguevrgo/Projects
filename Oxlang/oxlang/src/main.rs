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

    /// Peeks at the current character without consuming it.
    ///
    /// Returns:
    /// - `Some(char)` if there's input left.
    /// - `None` if at the end.
    fn look(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    /// Consumes the current character and advances the position.
    fn get_char(&mut self) {
        if let Some(c) = self.look() {
            self.pos += c.len_utf8();
        }
    }

    /// Prints an error and exits the program.
    ///
    /// # Parameters
    /// - `what`: Description of what was expected.
    fn expected(&self, what: &str) -> ! {
        eprintln!("\x07Error: Expected: {}", what);
        std::process::exit(1);
    }

    /// Skips over any whitespace characters.
    fn skip_whitespace(&mut self) {
        while self.look().is_some_and(|c| c.is_ascii_whitespace()) {
            self.get_char();
        }
    }

    /// Matches and consumes a specific character.
    ///
    /// # Parameters
    /// - `expected`: The character to match.
    fn match_char(&mut self, expected: char) {
        match self.look() {
            Some(c) if c == expected => self.get_char(),
            _ => self.expected(&format!("'{}'", expected)),
        }
    }

    /// Matches and consumes a specific keyword.
    ///
    /// # Parameters
    /// - `kw`: The keyword string to match.
    fn match_keyword(&mut self, kw: &str) {
        self.skip_whitespace();
        if self.input[self.pos..].starts_with(kw) {
            self.pos += kw.len();
        } else {
            self.expected(&format!("keyword `{kw}`"));
        }
    }

    /// Parses and returns a numeric literal as a string.
    ///
    /// Fails if no digits are found.
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

    /// Generates a fresh temporary variable name like `%t0`.
    fn fresh_temp(&mut self) -> String {
        let name = format!("%t{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    /// Emits a single line of output (LLVM-like IR).
    ///
    /// # Parameters
    /// - `msg`: The line to emit.
    fn emit(&self, msg: &str) {
        println!("\t{}", msg);
    }

    /// Parses and returns an identifier name.
    ///
    /// Fails if no valid identifier is found.
    fn get_name(&mut self) -> String {
        self.skip_whitespace();
        let mut name = String::new();
        while let Some(c) = self.look() {
            if c.is_ascii_alphabetic() {
                name.push(c);
                self.get_char();
            } else {
                break;
            }
        }

        if name.is_empty() {
            self.expected("Identifier")
        } else {
            name
        }
    }

    fn ident_or_func(&mut self) -> String {
        let name = self.get_name();
        if self.look() == Some('(') {
            self.match_char('(');
            self.match_char(')');
            self.emit(&format!("call void @{name}()"));
            "0".to_string()
        } else {
            let temp = self.fresh_temp();
            self.emit(&format!("{temp} = load i32, i32* %{name}"));
            temp
        }
    }

    fn factor(&mut self) -> String {
        self.skip_whitespace();
        match self.look() {
            Some('(') => {
                self.match_char('(');
                let result = self.expression();
                self.match_char(')');

                result
            }
            Some(c) if c.is_ascii_alphabetic() => self.ident_or_func(),
            _ => self.get_num(),
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

    fn parse_fn(&mut self) {
        self.match_keyword("fn");
        let name = self.get_name();
        self.match_char('(');
        self.match_char(')');
        self.skip_whitespace();
        self.match_char('{');

        println!("define i32 @{name}() {{");
        println!("entry:");

        let results = self.expression();

        self.emit(&format!("ret i32 {}", results));
        println!("}}");
    }
}

fn main() {
    let mut lexer = Lexer::new();
    lexer.parse_fn();
}
