/// Say hello in Norwegian
pub(crate) trait Hei {
    fn hei(&self);

    fn weird(&self);

    fn need_sized(self) -> Self
    where
        Self: Sized;
}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }

    fn weird(&self) {
        println!("you called wierd {}", self);
    }

    fn need_sized(self) -> Self {
        self
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
    }

    fn weird(&self) {
        println!("you called wierd {}", self);
    }

    fn need_sized(self) -> Self {
        self
    }
}

pub(crate) fn say_hei(s: &dyn Hei) {
    s.hei()
}

pub(crate) fn strlen<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

pub(crate) fn strlen2(s: String) -> usize {
    s.len()
}

// examples of trait objects
pub(crate) fn strlen_dyn2(s: Box<dyn AsRef<str>>) -> usize {
    s.as_ref().as_ref().len()
}

pub(crate) fn strlen_dyn(s: &dyn AsRef<str>) -> usize {
    s.as_ref().len()
}
