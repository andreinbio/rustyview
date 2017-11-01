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
        let template_blocks: Option<Vec<BlockFields>> = index_blocks(&file_content[..]);
        let clean_template: String = clean_template(template_blocks, &file_content[..]);
        let final_render: String = replace_template_variables(model, &clean_template[..]);

        //String::from("helllo")
        final_render
    }
}