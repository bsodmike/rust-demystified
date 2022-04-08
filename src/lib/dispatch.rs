pub trait Hei {
    fn hei(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

pub fn strlen<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

pub fn strlen2(s: String) -> usize {
    s.len()
}

pub fn strlen_dyn(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}

pub fn strlen_dyn2(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}
