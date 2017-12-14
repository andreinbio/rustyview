use serde_json;

#[derive(Debug)]
struct BlockData {
    start_tag: String,
    end_tag: String,
    content: String,
}

#[derive(Debug)]
enum BlockType {
    Block(BlockData),
    Extends(String),
}

pub struct Parser {
    pos: usize,
    input: String,
    nodes: Vec<BlockData>,
    extends: String,
}

impl Parser {
    /// Get a new instance of html Parser
    pub fn new(file_content: &str) -> Parser {
        let mut html_parser: Parser = Parser { pos: 0, input: String::from(file_content), nodes: Vec::new(), extends: String::new()};
        html_parser.parse_nodes();

        html_parser
    }

    /// Check if it's a child template
    pub fn is_child_template(&self) -> bool {
        !self.extends.is_empty()
    }

    /// Get parent file
    pub fn get_parent_file(&self) -> String {
        self.extends.clone()
    }

    /// Parse template
    pub fn parse_template(&self, model: serde_json::Value) -> String {
        let mut parsed_content: String = self.clean_template();
        parsed_content = self.parse_model(parsed_content, model);
        parsed_content
    }

    pub fn update_nodes(&mut self, template: Parser) -> () {
        template.nodes.iter().for_each(|block| {
            let parent_block = self.nodes.iter().find(|element| element.start_tag.eq(&block.start_tag[..]));
            if parent_block.is_some() {
                self.input = self.input.replace(&parent_block.unwrap().content[..], &block.content[..]);
            }
        });
    }

    /// Clean template
    fn clean_template(&self) -> String {
        let mut parsed_content: String = self.input.clone();
        self.nodes.iter().for_each(|block| {
            parsed_content = parsed_content
                .replace(&block.start_tag[..], "")
                .replace(&block.end_tag[..], "");
        });
        parsed_content
    }

    /// Parse model
    fn parse_model(&self, mut parsed_content: String, model: serde_json::Value) -> String {
        model.as_object().unwrap().iter().for_each(|(key, value)| {
            let string_key = format!("{}{}{}", "{{", key, "}}");
            parsed_content = parsed_content
                .replace(&string_key[..], value.as_str().unwrap());
        });
        parsed_content
    }

    /// start parsing
    fn parse_nodes(&mut self) -> () {
        loop {
            self.consume_while(|c| c != '{');
            if self.eof() {
                break;
            }
            let node: Option<BlockType> = self.parse_node();
            if node.is_some() {
                match node.unwrap() {
                    BlockType::Block(block_data) => self.nodes.push(block_data),
                    BlockType::Extends(string) => self.extends = string
                }
            } else {
                self.consume_char();
            }
        }
    }

    /// Parse one node
    fn parse_node(&mut self) -> Option<BlockType> {
        match &self.next_chars(2)[..] {
            "{%" => Some(self.parse_element()),
            "{{" => self.parse_variables(),
            _ => None,
        }
    }

    /// Parse block variables
    fn parse_variables(&mut self) -> Option<BlockType> {
        None
    }

    /// Parse block element, including its open tag, contents, and closing tag.
    fn parse_element(&mut self) -> BlockType {
        // Opening tag
        let mut start_tag: String = String::new();
        assert_eq!(self.next_char(), '{');
        start_tag.push(self.consume_char());
        assert_eq!(self.next_char(), '%');
        start_tag.push(self.consume_char());
        start_tag.push_str(&self.consume_whitespace()[..]);
        let open_tag: String = self.consume_while(|c| match c {
            'a'...'z'|'A'...'Z'|'0'...'9' => true,
            _ => false
        });
        //assert_eq!(open_tag, String::from("block"));
        start_tag.push_str(&open_tag[..]);
        start_tag.push_str(&self.consume_whitespace()[..]);
        let tag_name: String = self.consume_while(|c| c != '%');
        start_tag.push_str(&tag_name[..]);
        assert_eq!(self.next_char(), '%');
        start_tag.push(self.consume_char());
        assert_eq!(self.next_char(), '}');
        start_tag.push(self.consume_char());

        if open_tag.eq("extends") {
            return BlockType::Extends(String::from(tag_name.trim()))
        }

        // Contents
        let content = self.consume_until("{%");

        // Closing tag
        let mut end_tag: String = String::new();
        assert_eq!(self.next_char(), '{');
        end_tag.push(self.consume_char());
        assert_eq!(self.next_char(), '%');
        end_tag.push(self.consume_char());
        end_tag.push_str(&self.consume_whitespace()[..]);
        let close_tag: String = self.consume_while(|c| match c {
            'a'...'z'|'A'...'Z'|'0'...'9' => true,
            _ => false
        });
        end_tag.push_str(&close_tag[..]);
        end_tag.push_str(&self.consume_whitespace()[..]);
        let end_tag_name: String = self.consume_while(|c| c != '%');
        assert_eq!(end_tag_name, tag_name);
        end_tag.push_str(&end_tag_name[..]);
        assert_eq!(self.next_char(), '%');
        end_tag.push(self.consume_char());
        assert_eq!(self.next_char(), '}');
        end_tag.push(self.consume_char());

        BlockType::Block(
            BlockData {
                start_tag: start_tag,
                end_tag: end_tag,
                content: content,
            }
        )
    }

    /// Consume while
    fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
        let mut result = String::new();
        loop {
            if self.eof() || !test(self.next_char()) {
                break;
            }
            result.push(self.consume_char());
        }
        result
    }

    /// Consume until
    fn consume_until(&mut self, s: &str) -> String {
        let mut result = String::new();
        loop {
            if self.eof() || self.starts_with(s) {
                break;
            }
            result.push(self.consume_char());
        }
        result
    }

    /// Consume empty space
    fn consume_whitespace(&mut self) -> String {
        self.consume_while(char::is_whitespace)
    }

    /// Consume char
    fn consume_char(&mut self) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    /// Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    /// Read ...
    fn next_chars(&self, number_chars: usize) -> String {
        let mut iter = self.input[self.pos..].chars();
        let mut chars: String = String::new();
        let mut iteration: usize = 0;
        loop {
            if iteration == number_chars {
                break;
            }
            iteration += 1;
            chars.push(iter.next().unwrap());
        }
        chars
    }

    /// Does the current input start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos ..].starts_with(s)
    }

    /// Check for end of the line
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}
