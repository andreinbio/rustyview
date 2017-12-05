use serde_json;
use loader::*;
use parser::*;

pub struct View<'a> {
    templates_path: &'a str
}

impl<'a> View<'a> {
    pub fn new(path: &'a str) -> View<'a> {
        View {templates_path: path}
    }

    /// # Render provided template
    pub fn render(&self, template: &str, model: serde_json::Value) -> String {
        let file_content = self.get_file(template);
        let parsed_content = self.parse_template(file_content, model);

        parsed_content
    }

    /// # Get file
    fn get_file(&self, template: &str) -> String {
        let ref file_path = format!("{}{}", self.templates_path, template);
        let file_content = load_file(file_path);

        file_content
    }

    /// # Parse template
    fn parse_template(&self, file_content: String, model: serde_json::Value) -> String {
        let parent_file: Option<String> = get_parent_name(&file_content[..]);
        let mut template_blocks: Option<Vec<BlockFields>> = None;
        let mut final_content: String = file_content.clone();

        if parent_file.is_some() {
            let parent_template: String = parent_file.unwrap();
            let parent_content: String = self.get_file(&parent_template[..]);
            let parent_template_blocks: Option<Vec<BlockFields>> = index_blocks(&parent_content[..]);
            let child_blocks = index_blocks(&file_content[..]);
            template_blocks = update_parent_blocks(parent_template_blocks, child_blocks);
            final_content = parent_content.clone();
        }

        if template_blocks.is_none() {
            template_blocks = index_blocks(&file_content[..]);
        }

        println!("template_blocks => {:?}", template_blocks);

        let clean_template: String = clean_template(template_blocks, &final_content[..]);
        let final_render: String = replace_template_variables(model, &clean_template[..]);

        final_render
    }
}