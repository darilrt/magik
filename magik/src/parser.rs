use std::borrow::Cow;

use crate::{Error, template::TemplateData};

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

impl<'a> Iterator for Parser<'a> {
    type Item = Result<TemplateData<'a>, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.next_impl() {
                Ok(Some(data)) => {
                    match &data {
                        TemplateData::String(s) if s.is_empty() => continue, // Continue loop instead of recursion
                        TemplateData::Code(code) if code.trim().is_empty() => continue,
                        _ => return Some(Ok(data)),
                    }
                }
                Ok(None) => return None,
                Err(e) => return Some(Err(e)),
            }
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

    fn next_impl(&mut self) -> Result<Option<TemplateData<'a>>, Error> {
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

                            return Ok(Some(TemplateData::String(Cow::Borrowed(str))));
                        } else {
                            self.advance();
                        }
                    } else {
                        // End of input
                        if self.last_byte_pos < self.source.len() {
                            let str = &self.source[self.last_byte_pos..];
                            self.last_byte_pos = self.source.len();

                            if !str.is_empty() {
                                return Ok(Some(TemplateData::String(Cow::Borrowed(str))));
                            }
                        }
                        return Ok(None);
                    }
                }
                State::Inside => {
                    if let Some(ch) = self.peek() {
                        if ch == '}' && self.peek_next() == Some('}') {
                            // Extract code between {{ and }} but include single braces
                            let code = &self.source[self.last_byte_pos - 1..self.byte_pos + 1];

                            // Skip }}
                            self.advance(); // '}'
                            self.advance(); // '}'

                            self.last_byte_pos = self.byte_pos;
                            self.state = State::Outside;

                            return Ok(Some(TemplateData::Code(Cow::Borrowed(code))));
                        } else {
                            self.advance();
                        }
                    } else {
                        // End of input while in key state
                        return Err(Error::ParseError(Cow::Borrowed(
                            "Unexpected end of input while parsing key",
                        )));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::Error;

    use super::{Parser, TemplateData};

    // Helper macro for cleaner test assertions
    macro_rules! assert_next {
        ($parser:expr, $expected:expr) => {
            let token = $parser
                .next()
                .expect("Expected more tokens from parser")
                .expect("Token should parse without errors");
            assert_eq!(token.as_str(), $expected);
        };
    }

    #[test]
    fn test_parser() {
        let input = "<h1>Hello, {{ name }}!</h1> {{ test }}";
        let mut parser = Parser::new(input);

        assert_next!(parser, "<h1>Hello, ");
        assert_next!(parser, "{ name }");
        assert_next!(parser, "!</h1> ");
        assert_next!(parser, "{ test }");

        assert!(parser.next().is_none());
    }

    #[test]
    fn test_parser_error_handling() {
        let input = "Hello {{ unclosed";
        let mut parser = Parser::new(input);

        assert_next!(parser, "Hello ");

        // This should return an error
        let result = parser
            .next()
            .expect("Should have next token (even if error)");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Unexpected end of input while parsing key")
        );
    }

    #[test]
    fn test_parser_collect_success() {
        let input = "Hello {{ name }}!";
        let parser = Parser::new(input);

        let results: Result<Vec<TemplateData>, Error> = parser.collect();
        assert!(results.is_ok());

        let data = results.expect("Collection should succeed");
        assert_eq!(data.len(), 3);
    }

    #[test]
    fn test_parser_collect_error() {
        let input = "Hello {{ unclosed";
        let parser = Parser::new(input);

        let results: Result<Vec<TemplateData>, Error> = parser.collect();
        assert!(results.is_err());
    }
}
