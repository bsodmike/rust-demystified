#![allow(unused_variables)]
#![allow(dead_code)]

use std::fmt::Debug;

use anyhow::Error;

extern crate tutorials;

pub trait Storable: Debug {
    fn can_return_string(&self) -> String {
        let caller_type = std::any::type_name::<Self>();

        format!("Hello, this text is on the heap! {}", caller_type)
    }

    fn need_sized(self) -> Self
    where
        Self: Sized,
    {
        self
    }
}

#[derive(Debug)]
struct Holder<'a>(&'a dyn Storable);

#[derive(Debug)]
struct Carton {}

#[derive(Debug)]
struct Crate {}

impl Storable for Carton {}
impl Storable for Crate {}

// NOTE: This example demonstrates that we can call a method on a trait object, but notice that we
// have to use the `?Sized` bound on the type parameter `S` in the function signature.
//
// 38 | pub fn do_something<S>(s: &S) -> &S
//    |                     ^ required by the implicit `Sized` requirement on this type parameter in `do_something`
// help: consider relaxing the implicit `Sized` restriction
//    |
// 38 | pub fn do_something<S: ?Sized>(s: &S) -> &S
//    |                      ++++++++
pub fn do_something<S>(s: &S) -> &S
where
    S: ?Sized,
{
    let caller_type = std::any::type_name::<S>();
    tracing::info!("This is {:?}", caller_type);

    s
}

pub fn main() -> Result<(), Error> {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "tutorials=info,tower_http=trace,tokio=trace,runtime=trace",
        )
    }
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_file(true)
        .with_line_number(true)
        .init();

    let item = &Carton {};
    let holder1 = Holder(item);
    tracing::info!("holder1: {:?}", holder1);
    tracing::info!(
        "holder1: can_return_string(): {:?}",
        holder1.0.can_return_string()
    );
    let item = Crate {};
    let holder2 = Holder(&item);
    tracing::info!("holder2: {:?}", holder2);
    tracing::info!(
        "holder2: can_return_string(): {:?}",
        holder2.0.can_return_string()
    );

    tracing::info!(
        "need_sized() can be called directly on {:?}",
        &item.need_sized(),
    );

    // NOTE: However, we cannot call this on a trait object due to the Sized requirement
    //
    // dbg!(holder2.0.need_sized());
    // error: the `need_sized` method cannot be invoked on a trait object
    // --> src/bin/sized-1.rs:71:20
    // |
    // 19 |         Self: Sized,
    // |               ----- this has a `Sized` requirement
    // ...
    // 71 |     dbg!(holder2.0.need_sized());
    // |                    ^^^^^^^^^^

    let item = Crate {};
    let holder = Holder(&item);
    let s = do_something(holder.0);
    dbg!(s);

    Ok(())
}
