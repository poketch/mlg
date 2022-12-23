
#[derive(Debug, Clone)]
enum TokenKind {
    RBRACE,
    LBRACE,
    ATTR,
    SEMICOLON,
    STRING,
    WORD, //mword must remain as the last to maintain count
}

impl TokenKind {
    const COUNT: usize = (Self::WORD as usize) + 1;
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: TokenKind,
    text: String,
}

pub struct Tokenizer {
    source: String,
}

fn is_name(x: char) -> bool {
    x.is_alphabetic() || x == '_'
}

fn is_number(x: char) -> bool {
    x.is_numeric() || x == '.'
}
impl Token {
    fn new(kind: TokenKind) -> Self {
        let text = match kind {
            TokenKind::RBRACE => "}",
            TokenKind::LBRACE => "{",
            TokenKind::SEMICOLON => ";",
            TokenKind::STRING => panic!("To create a Token for a string use Token::from_string"),
            TokenKind::WORD => panic!("To create a Token for a word use Token::from_word"),
            TokenKind::ATTR => panic!("To create a Token for attirbutes use Token::from_attr"),
        };

        Self {
            kind,
            text: text.to_string(),
        }
    }

    fn from_word(word: impl Into<String>) -> Self {
        Self {
            kind: TokenKind::WORD,
            text: word.into(),
        }
    }

    fn from_string(string: impl Into<String>) -> Self {
        Self {
            kind: TokenKind::STRING,
            text: string.into(),
        }
    }

    fn from_attr(attr: impl Into<String>) -> Self {
        Self {
            kind: TokenKind::ATTR,
            text: attr.into(),
        }
    }
}

impl Tokenizer {
    pub fn new(s: impl Into<String>) -> Self {
        Self { source: s.into() }
    }

    fn pop_token(&mut self, length: usize) -> String {
        let mut popped = String::new();
        let mut chars = self.source.chars();
        for _ in 0..length {
            let ch = chars.next();
            popped.push(ch.unwrap());
        }

        self.source = chars.as_str().to_string();
        popped
    }

    fn pop_token_while(&mut self, pred: fn(char) -> bool) -> String {
        let mut len = 0;
        while len < self.source.len() && pred(self.source.chars().collect::<Vec<char>>()[len]) {
            len += 1;
        }

        self.pop_token(len)
    }
}

impl Iterator for Tokenizer { //this allows iteration and peeking
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.source = self.source.trim_start().to_string();

        if self.source.is_empty() {
            return None;
        }

        
        assert_eq!(TokenKind::COUNT, 6, "Exhaustive OPs check Iterator<Tokenizer>.next()");
        //above clause ensures source is not empty
        let token = match self.source.chars().nth(0).unwrap() {
            '{' => {
                // TODO: maybe validate closing brackets; maybe in parser?
                self.pop_token(1);
                Token::new(TokenKind::LBRACE)
            }

            '}' => {
                self.pop_token(1);
                Token::new(TokenKind::RBRACE)
            }

            '<' => {

                self.pop_token(1);
                let len = self.source.find('>').unwrap_or_else(||
                    {
                    eprintln!("Error: Closing > not found");
                    std::process::exit(1)
                    }
                );
                let tok = Token::from_attr(self.pop_token(len));
                self.pop_token(1); //pops the closing chevron

                tok
            }

            ';' => {
                self.pop_token(1);
                Token::new(TokenKind::SEMICOLON)
            }

            '"' => {
                self.pop_token(1);
                let len = self.source.find('"').unwrap_or_else(||
                    {
                    eprintln!("Error: Closing \" not found");
                    std::process::exit(1)
                    }
                );
                let tok = Token::from_string(self.pop_token(len));
                self.pop_token(1); //pops the closing quote

                tok
            }

            _ => Token::from_word(self.pop_token_while(is_name)),
        };

        Some(token)
    }
}

// region: Test

#[cfg(test)]
#[path = "../_tests/tokenizer.rs"]
mod tests;

// endregion: Test