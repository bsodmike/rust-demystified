/// Say hello in Norwegian
pub trait Hei {
    fn hei(&self);

    fn weird(&self);
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }

    fn weird(&self) {
        println!("you called wierd {}", self);
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }

    fn weird(&self) {
        println!("you called wierd {}", self);
    }
}

pub fn say_hei(s: &dyn Hei) {
    s.hei()
}

pub fn strlen<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

pub fn strlen2(s: String) -> usize {
    s.len()
}

// examples of trait objects
pub fn strlen_dyn2(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}

pub fn strlen_dyn(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}
