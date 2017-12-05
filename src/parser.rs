use serde_json;

#[derive(Debug)]
struct Pattern {
    start_pattern: String,
    end_pattern: String,
    block_pattern: String,
    end_block_pattern: String,
}

#[derive(Debug, Clone)]
enum TagType {
    start,
    end,
}

#[derive(Debug)]
struct TagLine {
    content: String,
    start_index: usize,
    next_index: usize,
}

#[derive(Debug, Clone)]
struct BlockTag {
    tag_type: TagType,
    content: String,
    content_index: usize,
}

#[derive(Debug, Clone)]
pub struct BlockFields {
    start_tag: Option<BlockTag>,
    end_tag: Option<BlockTag>,
    title: String,
    content: String,
    new_content: String
}

/// # Index blocks
pub fn index_blocks(file_content: &str) -> Option<Vec<BlockFields>> {
    let pattern = Pattern {
        start_pattern: String::from("{%"),
        end_pattern: String::from("%}"),
        block_pattern: String::from("block"),
        end_block_pattern: String::from("endblock")
    };

    let block_tags: Vec<BlockTag> = index_block_tags(file_content, &pattern);
    let blocks: Vec<BlockFields> = create_block_data(block_tags, &pattern, file_content);
    println!("blocks length {:?}", blocks.len());

    if blocks.len() > 0 {
        Some(blocks)
    } else {
        None
    }
}

// Index Block Tag
fn index_block_tags(file_content: &str, pattern: &Pattern) -> Vec<BlockTag> {
    let mut block_tags: Vec<BlockTag> = vec![];
    let ref mut current_index: usize = 0;

    loop {
        if !has_block_tag(&file_content[*current_index..], &pattern) {
            break;
        }
        let tag_string: TagLine = get_tag_line(&file_content[*current_index..], current_index, &pattern);

        // check for block tags
        //@TO DO latter check here for includes, filters tags also
        if tag_string.content.contains(&pattern.block_pattern) {
            // println!("{:?}", tag_string.content);
            block_tags.push(get_block_tag(tag_string, &pattern));
        }
    }

    block_tags
}

// Get Tag Line
fn get_tag_line(file_content: &str, current_index: &mut usize, pattern: &Pattern) -> TagLine {
    let start_index: usize = file_content.find(&*pattern.start_pattern).expect("start_index");
    let next_index: usize = file_content.find(&*pattern.end_pattern).expect("next_index") + (*pattern.end_pattern).len();
    *current_index += next_index;

    TagLine {
        content: String::from(&file_content[start_index..next_index]),
        start_index: *current_index - next_index + start_index,
        next_index: *current_index,
    }
}

// Get Block Tag
fn get_block_tag(tag_line: TagLine, pattern: &Pattern) -> BlockTag {
    let tag_type: TagType = get_tag_type(&tag_line.content, pattern);
    let index_tag: usize = get_tag_index(&tag_type, &tag_line.start_index, &tag_line.next_index);

    //println!("tag: {:?}", tag_content);
    //println!("start_index, next_index, index_tag: {:?}, {:?}, {:?}", start_index, next_index, index_tag);

    BlockTag {
        tag_type: tag_type,
        content: tag_line.content,
        content_index: index_tag,
    }
}

// Get Tag Type
fn get_tag_type(tag: &String, pattern: &Pattern) -> TagType {
    if tag.find(&pattern.end_block_pattern).is_some() {
        TagType::end
    } else {
        TagType::start
    }
}

// Get Tag Index
fn get_tag_index(tag_type: &TagType, start_index: &usize, next_index: &usize) -> usize {
    match *tag_type {
        TagType::start => *next_index,
        TagType::end => *start_index
    }
}

// Checks if cotent has a block tag present
fn has_block_tag(file_content: &str, pattern: &Pattern) -> bool {
    // for the moment check for the starting pattern
    file_content.find(&*pattern.start_pattern).is_some()
}

// Get Block Data
fn create_block_data(block_tags: Vec<BlockTag>, pattern: &Pattern, file_content: &str) -> Vec<BlockFields> {
    let mut blocks: Vec<BlockFields> = vec![];
    let mut iteration: usize = 0;
    let mut index: usize = 0;

    for block in block_tags {
        if iteration % 2 == 0 {
            //@TO DO: check that is a opening tag !!!
            blocks.push(get_default_block_fields());
            blocks[index].title = get_tag_name(&block.content, pattern);
            blocks[index].start_tag = Some(block);
        } else {
            //@TO DO: check to see that the closing tag matches the opening one !!!
            blocks[index].end_tag = Some(block);
            blocks[index].content = get_block_content(&blocks[index], file_content);
            index += 1;
        }
        iteration += 1;
    }

    blocks
}

fn get_default_block_fields() -> BlockFields {
    BlockFields {
        start_tag: None,
        end_tag: None,
        title: String::from(""),
        content: String::from(""),
        new_content: String::from("")
    }
}

fn get_tag_name(tag_content: &str, pattern: &Pattern) -> String {
    String::from(tag_content.replace(&pattern.start_pattern[..], "").replace(&pattern.end_pattern[..], "").replace(&pattern.end_block_pattern[..], "").replace(&pattern.block_pattern[..], "").trim())
}

fn get_block_content(blockField: &BlockFields, file_content: &str) -> String {
    let start_index: usize = blockField.start_tag.as_ref().expect("start tag index").content_index;
    let end_index: usize = blockField.end_tag.as_ref().expect("end tag index").content_index;

    String::from(&file_content[start_index..end_index])
}

/// # Clean
pub fn clean_template(blocks: Option<Vec<BlockFields>>, file_content: &str) -> String {
    let mut template_content: String = String::from(file_content);

    if blocks.is_some() {
        // for block in blocks.as_ref().unwrap() {
        //     let start_tag: &BlockTag = block.start_tag.as_ref().expect("start tag");
        //     let end_tag: &BlockTag = block.end_tag.as_ref().expect("end tag");
        //     let content_to_replace: String = String::from(&template_content[start_tag.content_index..end_tag.content_index]);
        //
        //     // template_content = template_content.replace(&start_tag.content[..], "");
        //     // template_content = template_content.replace(&end_tag.content[..], "");
        //     template_content = template_content.replace(&content_to_replace[..], &block.content);
        // }
        //
        // for block in blocks.as_ref().unwrap() {
        //     let start_tag: &BlockTag = block.start_tag.as_ref().expect("start tag");
        //     let end_tag: &BlockTag = block.end_tag.as_ref().expect("end tag");
        //     // let content_to_replace: String = String::from(&template_content[start_tag.content_index..end_tag.content_index]);
        //
        //     template_content = template_content.replace(&start_tag.content[..], "");
        //     template_content = template_content.replace(&end_tag.content[..], "");
        //     // template_content = template_content.replace(&content_to_replace[..], &block.content);
        // }

        for block in blocks.as_ref().unwrap() {
            let start_tag: &BlockTag = block.start_tag.as_ref().expect("start tag");
            let end_tag: &BlockTag = block.end_tag.as_ref().expect("end tag");

            template_content = template_content.replace(&start_tag.content[..], "");
            template_content = template_content.replace(&end_tag.content[..], "");

            //@TODO change code to use Option structure
            if block.new_content != "" {
                template_content = template_content.replace(&block.content, &block.new_content);
            }
        }
    }

    template_content
}

/// # Check for a parent template and return the its name
pub fn get_parent_name(file_content: &str) -> Option<String> {
    let mut template_name: Option<String> = None;
    let start_index: Option<usize> = file_content.find("{% extends ");

    if start_index.is_some() {
        let end_index: Option<usize> = (&file_content[start_index.unwrap()..]).find("%}");

        if end_index.is_some() {
            let string_tag: &str = &file_content[start_index.unwrap()..end_index.unwrap()];
            template_name = Some(String::from(string_tag.replace("{% extends ", "").replace("%}", "").replace("\"", "").trim()));
        }
    }

    template_name
}

/// # Combine two template blocks
pub fn update_parent_blocks(mut parent_blocks: Option<Vec<BlockFields>>, child_blocks: Option<Vec<BlockFields>>) -> Option<Vec<BlockFields>>{
    if parent_blocks.is_some() && child_blocks.is_some() {
        for parent_block in parent_blocks.as_mut().unwrap() {
            for child_block in child_blocks.as_ref().unwrap() {
                println!("parent->title: {}", parent_block.title);
                println!("child->title: {}", child_block.title);
                if parent_block.title == child_block.title {
                    parent_block.new_content = child_block.content.clone();
                }
            }
        }
    }

    parent_blocks
}

/// # Replace template variables
pub fn replace_template_variables(model: serde_json::Value, file_content: &str) -> String {
    let mut template_content: String = String::from(file_content);
    let modelKeys = model.as_object().unwrap().keys();

    for key in modelKeys {
        //@TO DO: use pattern for brackets replacements...
        let newKey = format!("{}{}{}", "{{", key, "}}");
        template_content = template_content.replace(&newKey[..], model[key].as_str().unwrap());
    }

    template_content
}
