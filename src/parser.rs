enum TagType {
    start,
    end,
}

struct Pattern {
    start_pattern: String,
    end_pattern: String,
}

struct BlockTag {
    tag_type: TagType,
    content: String,
    content_index: usize,
}

struct ChildBlock {
    exist: bool,
    content: Option<Box<BlockFields>>,
}

struct BlockFields {
    start_tag: BlockTag,
    end_tag: BlockTag,
    title: String,
    content: String,
    child_block: ChildBlock,
}

pub struct Block {
    blocks: Vec<BlockFields>,
}

pub fn index_blocks(file_content: String) -> Block {
    let pattern = Pattern {
        start_pattern: String::from("{%"),
        end_pattern: String::from("%}")
    };

    let mut blocks: Vec<BlockFields> = vec![];


    let block_tags: Vec<BlockTag> = index_block_tags(file_content, &pattern);

    //loop {
    //    if !hasBlock(&file_content[*current_index..], &pattern) {
    //        break;
    //    }
    //    blocks.push(getNextBlock(&file_content[*current_index..], current_index, &pattern));
    //}

    Block {blocks: blocks}
}

fn index_block_tags(file_content: String, pattern: &Pattern) -> Vec<BlockTag> {
    let block_tags: Vec<BlockTag> = vec![];
    let ref mut current_index: usize = 0;

    loop {
        if !has_block_tag(&file_content[*current_index..], &pattern) {
            break;
        }
        //block_tags.push();
    }

    block_tags
}

fn get_block_tag(file_content: &str, current_index: &mut usize, pattern: &Pattern) -> BlockTag {

    let start_index = file_content.find(&*pattern.start_pattern).expect("start_index");
    let next_index: usize = file_content.find(&*pattern.end_pattern).expect("next_index") + (*pattern.end_pattern).len();
    let tag_content: String = String::from(&file_content[start_index..next_index]);
    *current_index += next_index + 1;

    BlockTag {
        tag_type: TagType::start,
        content: tag_content,
        content_index: next_index,
    }
}

//fn get_next_block(file_content: &str, current_index: &mut usize, pattern: &Pattern) -> BlockFields {
//    //println!("current index:{}", current_index);
//    let start_index = file_content.find(&*pattern.start_pattern).expect("start_index");
//    let next_index: usize = file_content.find(&*pattern.end_pattern).expect("next_index") + (*pattern.end_pattern).len();
//    let start_tag: String = String::from(&file_content[start_index..next_index]);
//    *current_index += next_index + 1;
//
//    //println!("start_index letter is: {:?}", &file_content[start_index..start_index+2]);
//    //println!("next_index letter is: {:?}", &file_content[next_index-2..next_index]);
//    //
//    //println!("bock tag is: {:?}", start_tag);
//
//    BlockFields {
//        start_tag: start_tag,
//        end_tag: String::from("%}"),
//        title: String::from("Test"),
//        content: String::from("Content here"),
//        child_block: ChildBlock {
//            exist: false,
//            content: None
//        },
//    }
//}

fn has_block_tag(file_content: &str, pattern: &Pattern) -> bool {
    // for the moment check for the starting pattern
    file_content.find(&*pattern.start_pattern).is_some()
}
