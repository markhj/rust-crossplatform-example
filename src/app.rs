#[cfg(windows)]
#[allow(unused_imports)]
use crate::Windows;

#[cfg(unix)]
#[allow(unused_imports)]
use crate::Unix;

pub trait App {
    fn welcome(&self) -> &str;
    fn create_dir(&self, name: &str);
}

pub fn create_app() -> impl App {
    #[cfg(windows)]
    {
        Windows {}
    }

    #[cfg(unix)]
    {
        Unix {}
    }
}
