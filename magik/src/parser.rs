use crate::template::TemplateData;

#[derive(Debug, PartialEq)]
enum State {
    Outside,
    Inside,
}

pub struct Parser<'a> {
    source: &'a str,
    chars: std::str::Chars<'a>,
    current: Option<char>,
    next_char: Option<char>,
    byte_pos: usize,
    last_byte_pos: usize,
    state: State,
}

impl Iterator for Parser<'_> {
    type Item = Result<TemplateData, String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_impl() {
            Ok(Some(data)) => {
                match data {
                    TemplateData::String(s) if s.is_empty() => self.next(), // Recursivamente busca el siguiente
                    TemplateData::Code(code) if code.trim().is_empty() => self.next(),
                    _ => Some(Ok(data)),
                }
            }
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current = chars.next();
        let next_char = chars.next();

        Parser {
            source: input,
            chars,
            current,
            next_char,
            byte_pos: 0,
            last_byte_pos: 0,
            state: State::Outside,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.current {
            self.byte_pos += ch.len_utf8();
            self.current = self.next_char;
            self.next_char = self.chars.next();
            Some(ch)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<char> {
        self.current
    }

    fn peek_next(&self) -> Option<char> {
        self.next_char
    }

    fn next_impl(&mut self) -> Result<Option<TemplateData>, String> {
        loop {
            match self.state {
                State::Outside => {
                    if let Some(ch) = self.peek() {
                        if ch == '{' && self.peek_next() == Some('{') {
                            // Extract string before {{
                            let str = &self.source[self.last_byte_pos..self.byte_pos];

                            // Skip {{
                            self.advance(); // '{'
                            self.advance(); // '{'

                            self.last_byte_pos = self.byte_pos;
                            self.state = State::Inside;

                            return Ok(Some(TemplateData::String(str.to_string())));
                        } else {
                            self.advance();
                        }
                    } else {
                        // End of input
                        if self.last_byte_pos < self.source.len() {
                            let str = &self.source[self.last_byte_pos..];
                            self.last_byte_pos = self.source.len();

                            if !str.is_empty() {
                                return Ok(Some(TemplateData::String(str.to_string())));
                            }
                        }
                        return Ok(None);
                    }
                }
                State::Inside => {
                    if let Some(ch) = self.peek() {
                        if ch == '}' && self.peek_next() == Some('}') {
                            // Extract code between {{ and }} but include single braces
                            let inner_code = &self.source[self.last_byte_pos..self.byte_pos];
                            let code = format!("{{ {} }}", inner_code.trim());

                            // Skip }}
                            self.advance(); // '}'
                            self.advance(); // '}'

                            self.last_byte_pos = self.byte_pos;
                            self.state = State::Outside;

                            return Ok(Some(TemplateData::Code(code)));
                        } else {
                            self.advance();
                        }
                    } else {
                        // End of input while in key state
                        return Err("Unexpected end of input while parsing key".to_string());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Parser, TemplateData};

    #[test]
    fn test_parser() {
        let input = "<h1>Hello, {{ name }}!</h1> {{ test }}";
        let mut parser = Parser::new(input);

        let next = parser.next().unwrap().unwrap();
        assert_eq!(next, TemplateData::String("<h1>Hello, ".to_string()));

        let next = parser.next().unwrap().unwrap();
        assert_eq!(next, TemplateData::Code("{ name }".to_string()));

        let next = parser.next().unwrap().unwrap();
        assert_eq!(next, TemplateData::String("!</h1> ".to_string()));

        let next = parser.next().unwrap().unwrap();
        assert_eq!(next, TemplateData::Code("{ test }".to_string()));

        assert!(parser.next().is_none());
    }

    #[test]
    fn test_parser_error_handling() {
        let input = "Hello {{ unclosed";
        let mut parser = Parser::new(input);

        let next = parser.next().unwrap().unwrap();
        assert_eq!(next, TemplateData::String("Hello ".to_string()));

        // This should return an error
        let result = parser.next().unwrap();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unexpected end of input"));
    }

    #[test]
    fn test_parser_collect_success() {
        let input = "Hello {{ name }}!";
        let parser = Parser::new(input);

        let results: Result<Vec<TemplateData>, String> = parser.collect();
        assert!(results.is_ok());

        let data = results.unwrap();
        assert_eq!(data.len(), 3);
    }

    #[test]
    fn test_parser_collect_error() {
        let input = "Hello {{ unclosed";
        let parser = Parser::new(input);

        let results: Result<Vec<TemplateData>, String> = parser.collect();
        assert!(results.is_err());
    }
}
