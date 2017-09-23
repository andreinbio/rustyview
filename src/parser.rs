#[derive(Debug, Clone)]
enum TagType {
    start,
    end,
}

#[derive(Debug)]
struct Pattern {
    start_pattern: String,
    end_pattern: String,
    end_block_pattern: String
}

#[derive(Debug, Clone)]
struct BlockTag {
    tag_type: TagType,
    content: String,
    content_index: usize,
}

#[derive(Debug, Clone)]
struct ChildBlock {
    exist: bool,
    content: Option<Box<BlockFields>>,
}

#[derive(Debug, Clone)]
struct BlockFields {
    start_tag: Option<BlockTag>,
    end_tag: Option<BlockTag>,
    title: String,
    content: String,
    child_block: ChildBlock,
}

#[derive(Debug)]
pub struct Block {
    blocks: Vec<BlockFields>,
}

/// # Index blocks
pub fn index_blocks(file_content: &str) -> Block {
    let pattern = Pattern {
        start_pattern: String::from("{%"),
        end_pattern: String::from("%}"),
        end_block_pattern: String::from("endblock")
    };


    let block_tags: Vec<BlockTag> = index_block_tags(file_content, &pattern);
    let blocks: Vec<BlockFields> = get_block_data(block_tags);

    Block {blocks: blocks}
}

// Index Block Tag
fn index_block_tags(file_content: &str, pattern: &Pattern) -> Vec<BlockTag> {
    let mut block_tags: Vec<BlockTag> = vec![];
    let ref mut current_index: usize = 0;

    loop {
        if !has_block_tag(&file_content[*current_index..], &pattern) {
            break;
        }
        block_tags.push(get_block_tag(&file_content[*current_index..], current_index, &pattern));
    }

    block_tags
}

// Get Block Tag
fn get_block_tag(file_content: &str, current_index: &mut usize, pattern: &Pattern) -> BlockTag {

    let start_index: usize = file_content.find(&*pattern.start_pattern).expect("start_index");
    let next_index: usize = file_content.find(&*pattern.end_pattern).expect("next_index") + (*pattern.end_pattern).len();
    let tag_content: String = String::from(&file_content[start_index..next_index]);
    let tag_type: TagType = get_tag_type(&tag_content, pattern);
    let index_tag: usize = get_tag_index(&tag_type, &start_index, &next_index, current_index);
    *current_index += next_index;

    //println!("tag: {:?}", tag_content);
    //println!("start_index, next_index, index_tag: {:?}, {:?}, {:?}", start_index, next_index, index_tag);

    BlockTag {
        tag_type: tag_type,
        content: tag_content,
        content_index: index_tag,
    }
}

// Get Tag Type
fn get_tag_type(tag: &String, pattern: &Pattern) -> TagType {
    if tag.find(&*pattern.end_block_pattern).is_some() {
        TagType::end
    } else {
        TagType::start
    }
}

// Get Tag Index
fn get_tag_index(tag_type: &TagType, start_index: &usize, next_index: &usize, current_index: &usize) -> usize {
    match *tag_type {
        TagType::start => *current_index + *start_index,
        TagType::end => *current_index + *next_index
    }
}

// Checks if cotent has a block tag present
fn has_block_tag(file_content: &str, pattern: &Pattern) -> bool {
    // for the moment check for the starting pattern
    file_content.find(&*pattern.start_pattern).is_some()
}

// Get Block Data
fn get_block_data(block_tags: Vec<BlockTag>) -> Vec<BlockFields> {
    let mut blocks: Vec<BlockFields> = vec![];

    let mut index: usize = 0;
    let mut push_block: bool = false;
    let mut blockElement: BlockFields = get_default_block_fields();
    let mut block_start_number = 0;

    for block in block_tags {

        match block.tag_type {
            TagType::start => {
                
                blockElement = get_default_block_fields();
                blockElement.start_tag = Some(block);

                block_start_number += 1;
            },
            TagType::end => {
                blockElement.end_tag = Some(block);

                block_start_number -= 1;
            }
        }

        if block_start_number == 0 {
            blocks.push(blockElement.clone());
        }
    }

    blocks
}

fn get_default_block_fields() -> BlockFields {
    BlockFields {
        start_tag: None,
        end_tag: None,
        title: String::from(""),
        content: String::from(""),
        child_block: ChildBlock {
            exist: false,
            content: None
        }
    }
}

/// # Clean
pub fn clean_template(template_tags: Block, file_content: &str) -> String {
    let mut template_content: String = String::from(file_content);
    println!("!!!!!!!!!!!!!!!!!!!!!!!!{:?}", template_tags);
    for block in template_tags.blocks.iter() {
        //template_content.replace(block.content, "");
        println!("!!!!!!!!!!!!!!!!!!!!!!!!{:?}", block);
    }

    //String::from("hi")
    template_content
}
