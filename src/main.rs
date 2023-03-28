
mod app;
mod windows;
mod unix;

use app::{ App, create_app };

#[allow(unused_imports)]
use windows::Windows;

#[allow(unused_imports)]
use unix::Unix;

fn main() {
    let app = create_app();

    println!("Greeting: {}", app.welcome());
    
    app.create_dir("folder");
}
