struct BlockFields {
    startIndex: i64,
    endIndex: i64,
    blockContent: String
}

pub struct Parser {
    blocks: Vec<BlockFields>
}

impl Parser {
    pub fn replace_template_blocks(file_content: String) -> String {
        file_content
    }

}