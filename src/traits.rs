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
    lesson_3::run()?;

    Ok(())
}
/// Use `esp_idf_hal` as an example for advanced used of Traits and trait objects
mod lesson_3 {
    use crate::{
        into_ref,
        traits::lesson_3::peripheral::{Peripheral, PeripheralRef},
    };
    use anyhow::Result;

    mod core {

        #[macro_export]
        #[allow(unused_macros)]
        macro_rules! into_ref {
            ($($name:ident),*) => {
                $(
                    let $name = $name.into_ref();
                )*
            }
        }

        #[allow(unused_macros)]
        macro_rules! impl_peripheral_trait {
            ($type:ident) => {
                unsafe impl Send for $type {}

                impl $crate::traits::lesson_3::peripheral::sealed::Sealed for $type {}

                impl $crate::traits::lesson_3::peripheral::Peripheral for $type {
                    type P = $type;

                    #[inline]
                    unsafe fn clone_unchecked(&mut self) -> Self::P {
                        $type { ..*self }
                    }
                }
            };
        }

        #[allow(unused_macros)]
        macro_rules! impl_peripheral {
            ($type:ident) => {
                pub struct $type(::core::marker::PhantomData<*const ()>);

                impl $type {
                    /// # Safety
                    ///
                    /// Care should be taken not to instnatiate this peripheralinstance, if it is already instantiated and used elsewhere
                    #[inline(always)]
                    pub unsafe fn new() -> Self {
                        $type(::core::marker::PhantomData)
                    }
                }

                $crate::traits::lesson_3::core::impl_peripheral_trait!($type);
            };
        }

        #[allow(unused_imports)]
        pub(crate) use impl_peripheral;
        #[allow(unused_imports)]
        pub(crate) use impl_peripheral_trait;
        #[allow(unused_imports)]
        pub(crate) use into_ref;
    }

    mod peripheral {
        use core::marker::PhantomData;
        use core::ops::{Deref, DerefMut};

        pub struct PeripheralRef<'a, T> {
            inner: T,
            _lifetime: PhantomData<&'a mut T>,
        }

        impl<'a, T> PeripheralRef<'a, T> {
            #[inline]
            pub fn new(inner: T) -> Self {
                Self {
                    inner,
                    _lifetime: PhantomData,
                }
            }

            /// Unsafely clone (duplicate) a Peripheral singleton.
            ///
            /// # Safety
            ///
            /// This returns an owned clone of the Peripheral. You must manually ensure
            /// only one copy of the Peripheral is in use at a time. For example, don't
            /// create two SPI drivers on `SPI1`, because they will "fight" each other.
            ///
            /// You should strongly prefer using `reborrow()` instead. It returns a
            /// `PeripheralRef` that borrows `self`, which allows the borrow checker
            /// to enforce this at compile time.
            pub unsafe fn clone_unchecked(&mut self) -> PeripheralRef<'a, T>
            where
                T: Peripheral<P = T>,
            {
                PeripheralRef::new(self.inner.clone_unchecked())
            }

            /// Reborrow into a "child" PeripheralRef.
            ///
            /// `self` will stay borrowed until the child PeripheralRef is dropped.
            pub fn reborrow(&mut self) -> PeripheralRef<'_, T>
            where
                T: Peripheral<P = T>,
            {
                // safety: we're returning the clone inside a new PeripheralRef that borrows
                // self, so user code can't use both at the same time.
                PeripheralRef::new(unsafe { self.inner.clone_unchecked() })
            }

            /// Map the inner Peripheral using `Into`.
            ///
            /// This converts from `PeripheralRef<'a, T>` to `PeripheralRef<'a, U>`, using an
            /// `Into` impl to convert from `T` to `U`.
            ///
            /// For example, this can be useful to degrade GPIO pins: converting from PeripheralRef<'a, PB11>` to `PeripheralRef<'a, AnyPin>`.
            #[inline]
            pub fn map_into<U>(self) -> PeripheralRef<'a, U>
            where
                T: Into<U>,
            {
                PeripheralRef {
                    inner: self.inner.into(),
                    _lifetime: PhantomData,
                }
            }
        }

        impl<'a, T> Deref for PeripheralRef<'a, T> {
            type Target = T;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<'a, T> DerefMut for PeripheralRef<'a, T> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        /// Trait for any type that can be used as a Peripheral of type `P`.
        ///
        /// This is used in driver constructors, to allow passing either owned Peripherals (e.g. `TWISPI0`),
        /// or borrowed Peripherals (e.g. `&mut TWISPI0`).
        ///
        /// For example, if you have a driver with a constructor like this:
        ///
        /// ```ignore
        /// impl<'d, T: Instance> Twim<'d, T> {
        ///     pub fn new(
        ///         twim: impl Peripheral<P = T> + 'd,
        ///         irq: impl Peripheral<P = T::Interrupt> + 'd,
        ///         sda: impl Peripheral<P = impl GpioPin> + 'd,
        ///         scl: impl Peripheral<P = impl GpioPin> + 'd,
        ///         config: Config,
        ///     ) -> Self { .. }
        /// }
        /// ```
        ///
        /// You may call it with owned Peripherals, which yields an instance that can live forever (`'static`):
        ///
        /// ```ignore
        /// let mut twi: Twim<'static, ...> = Twim::new(p.TWISPI0, irq, p.P0_03, p.P0_04, config);
        /// ```
        ///
        /// Or you may call it with borrowed Peripherals, which yields an instance that can only live for as long
        /// as the borrows last:
        ///
        /// ```ignore
        /// let mut twi: Twim<'_, ...> = Twim::new(&mut p.TWISPI0, &mut irq, &mut p.P0_03, &mut p.P0_04, config);
        /// ```
        ///
        /// # Implementation details, for HAL authors
        ///
        /// When writing a HAL, the intended way to use this trait is to take `impl Peripheral<P = ..>` in
        /// the HAL's public API (such as driver constructors), calling `.into_ref()` to obtain a `PeripheralRef`,
        /// and storing that in the driver struct.
        ///
        /// `.into_ref()` on an owned `T` yields a `PeripheralRef<'static, T>`.
        /// `.into_ref()` on an `&'a mut T` yields a `PeripheralRef<'a, T>`.
        pub trait Peripheral: Sized + sealed::Sealed {
            /// Peripheral singleton type
            type P;

            /// Unsafely clone (duplicate) a Peripheral singleton.
            ///
            /// # Safety
            ///
            /// This returns an owned clone of the Peripheral. You must manually ensure
            /// only one copy of the Peripheral is in use at a time. For example, don't
            /// create two SPI drivers on `SPI1`, because they will "fight" each other.
            ///
            /// You should strongly prefer using `into_ref()` instead. It returns a
            /// `PeripheralRef`, which allows the borrow checker to enforce this at compile time.
            unsafe fn clone_unchecked(&mut self) -> Self::P;

            /// Convert a value into a `PeripheralRef`.
            ///
            /// When called on an owned `T`, yields a `PeripheralRef<'static, T>`.
            /// When called on an `&'a mut T`, yields a `PeripheralRef<'a, T>`.
            #[inline]
            fn into_ref<'a>(mut self) -> PeripheralRef<'a, Self::P>
            where
                Self: 'a,
            {
                PeripheralRef::new(unsafe { self.clone_unchecked() })
            }
        }

        impl<T: DerefMut> sealed::Sealed for T {}

        impl<T: DerefMut> Peripheral for T
        where
            T::Target: Peripheral,
        {
            type P = <T::Target as Peripheral>::P;

            #[inline]
            unsafe fn clone_unchecked(&mut self) -> Self::P {
                self.deref_mut().clone_unchecked()
            }
        }

        pub(crate) mod sealed {
            pub trait Sealed {}
        }
    }

    pub(crate) mod gpio {
        use super::core::impl_peripheral_trait;
        use super::peripheral::{Peripheral, PeripheralRef};
        use anyhow::Result;
        use core::marker::PhantomData;

        /// A trait implemented by every pin instance
        pub trait Pin: Peripheral<P = Self> + Sized + Send + 'static {
            fn pin(&self) -> i32;
        }

        /// A marker trait designating a pin which is capable of
        /// operating as an input pin
        pub trait InputPin: Pin + Into<AnyInputPin> {
            fn downgrade_input(self) -> AnyInputPin {
                self.into()
            }
        }

        /// A marker trait designating a pin which is capable of
        /// operating as an output pin
        pub trait OutputPin: Pin + Into<AnyOutputPin> {
            fn downgrade_output(self) -> AnyOutputPin {
                self.into()
            }
        }

        /// A marker trait designating a pin which is capable of
        /// operating as an input and output pin
        pub trait IOPin: InputPin + OutputPin + Into<AnyIOPin> {
            fn downgrade(self) -> AnyIOPin {
                self.into()
            }
        }

        /// Generic Gpio input-output pin
        pub struct AnyIOPin {
            pin: i32,
            _p: PhantomData<*const ()>,
        }

        impl AnyIOPin {
            /// # Safety
            ///
            /// Care should be taken not to instantiate this Pin, if it is
            /// already instantiated and used elsewhere, or if it is not set
            /// already in the mode of operation which is being instantiated
            pub unsafe fn new(pin: i32) -> Self {
                Self {
                    pin,
                    _p: PhantomData,
                }
            }
        }

        impl_peripheral_trait!(AnyIOPin);

        impl Pin for AnyIOPin {
            fn pin(&self) -> i32 {
                self.pin
            }
        }

        impl InputPin for AnyIOPin {}
        impl OutputPin for AnyIOPin {}

        /// Generic Gpio input pin
        pub struct AnyInputPin {
            pin: i32,
            _p: PhantomData<*const ()>,
        }

        impl AnyInputPin {
            /// # Safety
            ///
            /// Care should be taken not to instantiate this Pin, if it is
            /// already instantiated and used elsewhere, or if it is not set
            /// already in the mode of operation which is being instantiated
            pub unsafe fn new(pin: i32) -> Self {
                Self {
                    pin,
                    _p: PhantomData,
                }
            }
        }

        impl_peripheral_trait!(AnyInputPin);

        impl Pin for AnyInputPin {
            fn pin(&self) -> i32 {
                self.pin
            }
        }

        impl InputPin for AnyInputPin {}

        impl From<AnyIOPin> for AnyInputPin {
            fn from(pin: AnyIOPin) -> Self {
                unsafe { Self::new(pin.pin()) }
            }
        }

        /// Generic Gpio output pin
        pub struct AnyOutputPin {
            pin: i32,
            _p: PhantomData<*const ()>,
        }

        impl AnyOutputPin {
            /// # Safety
            ///
            /// Care should be taken not to instantiate this Pin, if it is
            /// already instantiated and used elsewhere, or if it is not set
            /// already in the mode of operation which is being instantiated
            pub unsafe fn new(pin: i32) -> Self {
                Self {
                    pin,
                    _p: PhantomData,
                }
            }
        }

        impl_peripheral_trait!(AnyOutputPin);

        impl Pin for AnyOutputPin {
            fn pin(&self) -> i32 {
                self.pin
            }
        }

        impl OutputPin for AnyOutputPin {}

        impl From<AnyIOPin> for AnyOutputPin {
            fn from(pin: AnyIOPin) -> Self {
                unsafe { Self::new(pin.pin()) }
            }
        }

        pub trait InputMode {
            const RTC: bool;
        }
        pub trait OutputMode {
            const RTC: bool;
        }

        pub struct Output;
        pub struct Input;
        pub struct InputOutput;

        impl InputMode for Input {
            const RTC: bool = false;
        }

        impl InputMode for InputOutput {
            const RTC: bool = false;
        }

        impl OutputMode for Output {
            const RTC: bool = false;
        }

        impl OutputMode for InputOutput {
            const RTC: bool = false;
        }

        /// A driver for a GPIO pin.
        ///
        /// The driver can set the pin as a disconnected/disabled one, input, or output pin, or both or analog.
        /// On some chips (i.e. esp32 and esp32s*), the driver can also set the pin in RTC IO mode.
        /// Depending on the current operating mode, different sets of functions are available.
        ///
        /// The mode-setting depends on the capabilities of the pin as well, i.e. input-only pins cannot be set
        /// into output or input-output mode.
        pub struct PinDriver<'d, T: Pin, MODE> {
            pin: PeripheralRef<'d, T>,
            _mode: PhantomData<MODE>,
        }

        impl<'d, T: InputPin> PinDriver<'d, T, Input> {
            /// Creates the driver for a pin in input state.
            #[inline]
            pub fn input(pin: impl Peripheral<P = T> + 'd) -> Result<Self> {
                crate::into_ref!(pin);

                Self {
                    pin,
                    _mode: PhantomData,
                }
                .into_input()
            }
        }

        impl<'d, T: InputPin + OutputPin> PinDriver<'d, T, InputOutput> {
            /// Creates the driver for a pin in input-output state.
            #[inline]
            pub fn input_output(pin: impl Peripheral<P = T> + 'd) -> Result<Self> {
                crate::into_ref!(pin);

                Self {
                    pin,
                    _mode: PhantomData,
                }
                .into_input_output()
            }
        }

        impl<'d, T: OutputPin> PinDriver<'d, T, Output> {
            /// Creates the driver for a pin in output state.
            #[inline]
            pub fn output(pin: impl Peripheral<P = T> + 'd) -> Result<Self> {
                crate::into_ref!(pin);

                Self {
                    pin,
                    _mode: PhantomData,
                }
                .into_output()
            }
        }

        impl<'d, T: Pin, MODE> PinDriver<'d, T, MODE> {
            /// Returns the pin number.
            pub fn pin(&self) -> i32 {
                self.pin.pin()
            }

            /// Put the pin into input mode.
            #[inline]
            pub fn into_input(self) -> Result<PinDriver<'d, T, Input>>
            where
                T: InputPin,
            {
                self.into_mode("input")
            }

            /// Put the pin into input + output mode.
            #[inline]
            pub fn into_input_output(self) -> Result<PinDriver<'d, T, InputOutput>>
            where
                T: InputPin + OutputPin,
            {
                self.into_mode("input_output")
            }

            /// Put the pin into output mode.
            #[inline]
            pub fn into_output(self) -> Result<PinDriver<'d, T, Output>>
            where
                T: OutputPin,
            {
                self.into_mode("output")
            }

            #[inline]
            fn into_mode<M>(mut self, mode: &str) -> Result<PinDriver<'d, T, M>>
            where
                T: Pin,
            {
                let pin = unsafe { self.pin.clone_unchecked() };

                drop(self);

                if mode != "disabled" {
                    // esp!(unsafe { gpio_set_direction(pin.pin(), mode) })?;
                }

                Ok(PinDriver {
                    pin,
                    _mode: PhantomData,
                })
            }

            /// Toggle pin output
            #[inline]
            pub fn toggle(&mut self) -> Result<()>
            where
                MODE: OutputMode,
            {
                // Todo
                Ok(())
            }
        }

        unsafe impl<'d, T: Pin, MODE> Send for PinDriver<'d, T, MODE> {}

        macro_rules! impl_input {
            ($pxi:ident: $pin:expr) => {
                $crate::traits::lesson_3::core::impl_peripheral!($pxi);

                impl $crate::traits::lesson_3::gpio::Pin for $pxi {
                    fn pin(&self) -> i32 {
                        $pin
                    }
                }

                impl InputPin for $pxi {}

                impl From<$pxi> for AnyInputPin {
                    fn from(pin: $pxi) -> Self {
                        unsafe { Self::new(pin.pin()) }
                    }
                }
            };
        }

        macro_rules! impl_input_output {
            ($pxi:ident: $pin:expr) => {
                $crate::traits::lesson_3::gpio::impl_input!($pxi: $pin);

                impl OutputPin for $pxi {}

                impl IOPin for $pxi {}

                impl From<$pxi> for AnyOutputPin {
                    fn from(pin: $pxi) -> Self {
                        unsafe { Self::new(pin.pin()) }
                    }
                }

                impl From<$pxi> for AnyIOPin {
                    fn from(pin: $pxi) -> Self {
                        unsafe { Self::new(pin.pin()) }
                    }
                }
            };
        }

        macro_rules! pin {
            ($pxi:ident: $pin:expr, Input) => {
                $crate::traits::lesson_3::gpio::impl_input!($pxi: $pin);
            };

            ($pxi:ident: $pin:expr, IO) => {
                $crate::traits::lesson_3::gpio::impl_input_output!($pxi: $pin);
            };
        }

        #[allow(unused_imports)]
        pub(crate) use impl_input;
        #[allow(unused_imports)]
        pub(crate) use impl_input_output;
        #[allow(unused_imports)]
        pub(crate) use pin;
    }

    pub fn run() -> Result<()> {
        use gpio::*;
        use std::ops::Deref;

        gpio::pin!(Gpio0:0, IO);
        gpio::pin!(Gpio34:34, Input);

        unsafe {
            let gpio0 = Gpio0::new();
            let mut io_pin = gpio0.downgrade();

            {
                let mut pin_driver = PinDriver::input_output(&mut io_pin)?;

                pin_driver.toggle()?;
            }
            // NOTE: by using a block here, the `pin_driver` is implicitly dropped when execution reaches the end of the block.  This allowes for the drowngraded pin to be used without clashing with the borrow-checker.
        }

        Ok(())
    }
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
