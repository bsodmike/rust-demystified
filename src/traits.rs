use anyhow::{Error, Result};
use log::info;
use std::{
    error::Error as StdError,
    fmt::{self, Display},
};

// Lesson 1
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

// Error
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct BlanketError {
    inner: BoxError,
}

impl BlanketError {
    /// Create a new `Error` from a boxable error.
    pub fn new(error: impl Into<BoxError>) -> Self {
        Self {
            inner: error.into(),
        }
    }

    /// Convert an `Error` back into the underlying boxed trait object.
    pub fn into_inner(self) -> BoxError {
        self.inner
    }
}

impl fmt::Display for BlanketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl StdError for BlanketError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.inner)
    }
}

// Lesson 2: Traits
pub trait RTC {
    type Error: ErrorType;
}

pub trait ErrorType: Display {
    /// Error type
    type Error: std::error::Error;
}

// What does this do exactly?
impl<T: ErrorType> ErrorType for &mut T {
    type Error = T::Error;
}

impl ErrorType for BlanketError {
    type Error = Self;
}

struct RTCDevice(u8);

impl RTC for RTCDevice {
    type Error = BlanketError;
}

impl fmt::Display for RTCDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl ErrorType for RTCDevice {
    type Error = BlanketError;
}

pub fn runner() -> Result<()> {
    lesson_1_add_trait_bound_to_parameter();

    lesson_2();

    Ok(())
}

fn test_mut_t<T>(device: T, message: &str)
where
    T: ErrorType,
{
    info!("{}", message);
}

fn lesson_2() {
    let device = RTCDevice(1);
    test_mut_t::<RTCDevice>(device, "This is a standard use of `T: Trait`");

    // This works with the "forwarding impl" for `&mut T`.  It is handy to note that `T: Trait`, doesn't automatically mean `&mut T: Trait`. You have to write a "forwarding impl" for that. These are fairly common. &mut has them for Iterator, Write, Display, Debug, and more, for example
    let mut device = RTCDevice(1);
    test_mut_t::<&mut RTCDevice>(&mut device, "Here we are using `&mut T: Trait`");
}

fn lesson_1_add_trait_bound_to_parameter() {
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
}
