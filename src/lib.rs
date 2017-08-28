extern crate serde;
extern crate serde_json;

mod parser;

pub mod view {
    use serde_json;

    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Read;

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
            // get file here
            let file_content = self.get_file(template);
            let parsed_content = self.parse_template(file_content);

            parsed_content
        }

        /// # Get file
        fn get_file(&self, template: &str) -> String {
            let ref full_file_path = format!("{}{}", self.templates_path, template);

            let mut f = File::open(full_file_path).expect("Unable to open");
            let mut contents = String::new();
            f.read_to_string(&mut contents).expect("Error reading file");

            contents
        }

        /// # Parse template variables
        fn parse_template(&self, file_content: String) -> String {
            let mut file_content = Parser::replace_template_blocks(file_content);

            file_content = file_content.replace("{{title}}", "Testing");

            file_content
        }
    }
}