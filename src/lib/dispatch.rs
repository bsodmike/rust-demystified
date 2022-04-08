pub trait Hei {
    fn hei(&self);

    fn weird() {};
// error[E0038]: the trait `dispatch::Hei` cannot be made into an object
//   --> src/lib/dispatch.rs:19:20
//    |
// 19 | pub fn say_hei(s: &dyn Hei) {
//    |                    ^^^^^^^ `dispatch::Hei` cannot be made into an object
//    |
// note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
//   --> src/lib/dispatch.rs:4:8
//    |
// 1  | pub trait Hei {
//    |           --- this trait cannot be made into an object...
// ...
// 4  |     fn weird() {};
//    |        ^^^^^ ...because associated function `weird` has no `self` parameter
// help: consider turning `weird` into a method by giving it a `&self` argument
//    |
// 4  |     fn weird(&self) {};
//    |              +++++
// help: alternatively, consider constraining `weird` so it does not apply to trait objects
//    |
// 4  |     fn weird() where Self: Sized {};
//    |                +++++++++++++++++


}

impl Hei for &str {
    fn hei(&self) {
        println!("hei {}", self);
    }
}

impl Hei for String {
    fn hei(&self) {
        println!("hei {}", self);
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
