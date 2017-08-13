pub mod view {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::Read;

    pub struct View<'a> {
        templates_path: &'a str
    }

    impl<'a> View<'a> {
        pub fn new() -> View<'a> {
            View {templates_path: "src/admin/templates/default/"}
        }

        pub fn render(&self, template: &str) -> String {
            // get file here
            self.get_file(template)

            // parse contents...
        }

        fn get_file(&self, template: &str) -> String {
            let ref full_file_path = format!("{}{}", self.templates_path, template);

            let mut f = File::open(full_file_path).expect("Unable to open");
            let mut contents = String::new();
            f.read_to_string(&mut contents).expect("Error reading file");

            contents
        }
    }
}