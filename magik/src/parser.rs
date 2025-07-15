use crate::template::TemplateData;

#[derive(Debug, PartialEq)]
enum ParserState {
    Normal,
    InKey,
}

pub struct Parser {
    source: Vec<char>,
    pos: usize,
    last_pos: usize,
    length: usize,
    state: ParserState,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let source: Vec<char> = input.chars().collect();
        let length = source.len();

        Parser {
            source,
            pos: 0,
            last_pos: 0,
            length,
            state: ParserState::Normal,
        }
    }

    pub fn next(&mut self) -> Option<TemplateData> {
        while self.pos < self.length {
            let char = self.source[self.pos];

            match self.state {
                ParserState::Normal => {
                    if char == '{' && self.pos + 1 < self.length && self.source[self.pos + 1] == '{'
                    {
                        let str = self.source[self.last_pos..self.pos].to_vec();
                        self.pos += 2;
                        self.last_pos = self.pos;

                        self.state = ParserState::InKey;

                        return Some(TemplateData::Html(str.iter().collect::<String>()));
                    }
                }
                ParserState::InKey => {
                    if char == '}' && self.pos + 1 < self.length && self.source[self.pos + 1] == '}'
                    {
                        let code = self.source[self.last_pos - 1..self.pos + 1]
                            .iter()
                            .collect::<String>()
                            .trim()
                            .to_string();

                        self.pos += 2;
                        self.last_pos = self.pos;
                        self.state = ParserState::Normal;

                        return Some(TemplateData::Code(code));
                    }
                }
            }

            self.pos += 1;
        }

        if self.last_pos < self.length {
            let str = self.source[self.last_pos..].iter().collect::<String>();
            self.last_pos = self.length; // Update last_pos to the end of the source

            if str.is_empty() {
                return None; // No more data to return
            }

            return Some(TemplateData::Html(str));
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::{Parser, TemplateData};

    #[test]
    fn test_parser() {
        let input = "<h1>Hello, {{ name }}!</h1> {{ test }}";
        let mut parser = Parser::new(input);

        let next = parser.next().unwrap();
        assert_eq!(next, TemplateData::Html("<h1>Hello, ".to_string()));

        let next = parser.next().unwrap();
        assert_eq!(next, TemplateData::Code("{ name }".to_string()));

        let next = parser.next().unwrap();
        assert_eq!(next, TemplateData::Html("!</h1> ".to_string()));

        let next = parser.next().unwrap();
        assert_eq!(next, TemplateData::Code("{ test }".to_string()));

        assert!(parser.next().is_none());
    }
}
