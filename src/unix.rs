use crate::App;

pub struct Unix {

}

impl App for Unix {
    fn welcome(&self) -> &'static str {
        "Hello from Unix!"
    }
    fn create_dir(&self, name: &str) {
        println!("Create directory Unix style: {}", name);
    }
}
