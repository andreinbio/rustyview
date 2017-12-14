use serde_json;
use loader;
use html;

#[derive(Clone)]
pub struct View {
    templates_path: String
}

impl View {
    pub fn new(path: String) -> View {
        View {templates_path: path}
    }

    /// # Render provided template
    pub fn render(&self, template: &str, model: serde_json::Value) -> String {
        let file_content: String = self.get_content(template);
        let parsed_content: String = self.parse_content(file_content, model);

        parsed_content
    }

    /// # Get file
    fn get_content(&self, template: &str) -> String {
        let ref file_path = format!("{}{}", self.templates_path, template);
        let file_content: String = loader::read_source(file_path);

        file_content
    }

    /// # Parse template
    fn parse_content(&self, file_content: String, model: serde_json::Value) -> String {
        let template: html::Parser = html::Parser::new(&file_content[..]);

        if template.is_child_template() {
            let parent_template: String = template.get_parent_file();
            let parent_content: String = self.get_content(&parent_template[..]);
            let mut parent_template: html::Parser = html::Parser::new(&parent_content[..]);
            parent_template.update_nodes(template);

            parent_template.parse_template(model)
        } else {
            template.parse_template(model)
        }
    }
}
