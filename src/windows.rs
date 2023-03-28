use crate::App;

pub struct Windows {

}

impl App for Windows {
    fn welcome(&self) -> &'static str {
        "Hello from Windows!"
    }
    fn create_dir(&self, name: &str) {
        println!("Create directory Windows style: {}", name);
    }
}
