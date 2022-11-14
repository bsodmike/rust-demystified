use anyhow::{Error, Result};
use std::fmt::Display;

pub fn x(b: Box<impl Display + 'static>) -> Box<dyn Display> {
    b
}

#[derive(Debug)]
struct Device(u8);

impl Device {
    fn new(id: u8) -> Self {
        Self(id)
    }
}

// This is added to satisfy the trait bound on `x`
impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Device ({})", self.0))
    }
}

pub fn runner() -> Result<()> {
    let device = Device::new(1);

    // If we try to run `let resp = x(Box::new(device));`, notice that we have not satisfied the trait bound that is specified on `x`.
    //
    // error[E0277]: `Device` doesn't implement `std::fmt::Display`
    //   --> src/lib/traits.rs:19:18
    //    |
    // 19 |     let resp = x(Box::new(device));
    //    |                - ^^^^^^^^^^^^^^^^ `Device` cannot be formatted with the default formatter
    //    |                |
    //    |                required by a bound introduced by this call
    //    |
    //    = help: the trait `std::fmt::Display` is not implemented for `Device`
    //    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // note: required by a bound in `x`
    //   --> src/lib/traits.rs:4:22
    //    |
    // 4  | pub fn x(b: Box<impl Display + 'static>) -> Box<dyn Display> {
    //    |                      ^^^^^^^ required by this bound in `x`
    let resp = x(Box::new(device));

    Ok(())
}
