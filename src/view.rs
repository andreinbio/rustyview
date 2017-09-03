
use serde_json;
use loader::*;
use parser::*;

pub struct View<'a> {
    templates_path: &'a str
}

impl<'a> View<'a> {
    pub fn new() -> View<'a> {
        View {templates_path: "src/admin/templates/default/"}
    }

    /// # Render provided template
    pub fn render(&self, template: &str, model: serde_json::Value) -> String {
        let file_content = self.get_file(template);
        let parsed_content = self.parse_template(file_content);

        parsed_content
    }

    /// # Get file
    fn get_file(&self, template: &str) -> String {
        let ref file_path = format!("{}{}", self.templates_path, template);
        let file_content = load_file(file_path);

        file_content
    }

    /// # Parse template
    fn parse_template(&self, file_content: String) -> String {
        let template_blocks = index_blocks(file_content);

        String::from("helllo")
    }
}