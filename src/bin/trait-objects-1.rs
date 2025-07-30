#![allow(unused_variables)]
#![allow(dead_code)]

use anyhow::Error;

extern crate tutorials;

fn invoke_trait_object<T: Clonable>(x: &mut dyn Foo<T>) {
    x.dont_need_sized();
}

trait Foo<A: Clonable> {
    fn has_generic(&self, gen: A);
    fn dont_need_sized(&self);
    // fn need_sized(self) -> Self;
    // where
    //     Self: Sized;
}

trait Clonable: Clone {
    fn is_clonable(&self) -> bool;
}

// Blanket impl on any T that impls the provided traits.
impl<T> Clonable for T
where
    T: std::fmt::Display + std::clone::Clone,
{
    fn is_clonable(&self) -> bool {
        true
    }
}

struct S;

impl<A> Foo<A> for S
where
    A: Clonable + std::fmt::Display,
{
    fn has_generic(&self, gen: A) {
        let a = gen.is_clonable();
        let x = gen.clone();
        let x_ptr = &x as *const A;
        let x_points_at = unsafe { (*x_ptr).clone() };

        println!("this is x: {}", x_points_at);
    }

    fn dont_need_sized(&self) {}

    // fn need_sized(self) -> Self {
    //     Self {}
    // }
}

pub fn main() -> Result<(), Error> {
    let mut s = S {};
    let x = invoke_trait_object::<String>(&mut s);

    s.has_generic("foo".to_string());

    Ok(())
}
