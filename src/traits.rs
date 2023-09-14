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
    // lesson_4::run();```````````````````````````````````
    // lesson_5::run();

    // lesson6::run();
    lesson7::run();
    let _ = lesson8::run();
    lesson9::run();
    lesson10::run();

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

// https://github.com/dtolnay/erased-serde/blob/master/explanation/main.rs
mod lesson_4 {
    /////////////////////////////////////////////////////////////////////
    // Suppose these are the real traits from Serde.

    trait Querializer {}

    trait Generic {
        // Not object safe because of this generic method.
        fn generic_fn<Q: Querializer>(&self, querializer: Q);
    }

    // Note: extra constraint `where T: Querializer` does not seem to be needed.
    impl<'a, T: ?Sized> Querializer for &'a T {}

    impl<'a, T: ?Sized> Generic for Box<T>
    where
        T: Generic,
    {
        fn generic_fn<Q: Querializer>(&self, querializer: Q) {
            println!("impl<'a, T: ?Sized> Generic for Box<T>: calling generic_fn()");

            (**self).generic_fn(querializer)
        }
    }

    /////////////////////////////////////////////////////////////////////
    // This is an object-safe equivalent that interoperates seamlessly.

    trait ErasedGeneric {
        fn erased_fn(&self, querializer: &dyn Querializer);
    }

    impl Generic for dyn ErasedGeneric {
        // Depending on the trait method signatures and the upstream
        // impls, could also implement for:
        //
        //   - &'a dyn ErasedGeneric
        //   - &'a (dyn ErasedGeneric + Send)
        //   - &'a (dyn ErasedGeneric + Sync)
        //   - &'a (dyn ErasedGeneric + Send + Sync)
        //   - Box<dyn ErasedGeneric>
        //   - Box<dyn ErasedGeneric + Send>
        //   - Box<dyn ErasedGeneric + Sync>
        //   - Box<dyn ErasedGeneric + Send + Sync>
        fn generic_fn<Q: Querializer>(&self, querializer: Q) {
            println!("impl Generic for dyn ErasedGeneric: call self.erased_fn()");
            self.erased_fn(&querializer)
        }
    }

    impl<T> ErasedGeneric for T
    where
        T: Generic,
    {
        fn erased_fn(&self, querializer: &dyn Querializer) {
            println!("erased_fn() caling self.generic_fn");
            self.generic_fn(querializer)
        }
    }

    pub fn run() {
        struct T;
        impl Querializer for T {}

        #[derive(Debug)]
        struct S {
            size: usize,
        }
        impl Generic for S {
            fn generic_fn<Q: Querializer>(&self, _querializer: Q) {
                println!("querying the real S");
            }
        }

        impl Generic for &S {
            fn generic_fn<Q: Querializer>(&self, _querializer: Q) {
                println!("querying the real &S");
                let s = *self;

                dbg!("{:#?}", s);
            }
        }

        // Construct a trait object.
        let trait_object: Box<dyn ErasedGeneric> = Box::new(S { size: 0 });

        // Seamlessly invoke the generic method on the trait object.
        //
        // THIS LINE LOOKS LIKE MAGIC. We have a value of type trait
        // object and we are invoking a generic method on it.
        trait_object.generic_fn(T);

        println!("");
        let trait_object: Box<dyn ErasedGeneric> = Box::new(&S { size: 0 });
        trait_object.generic_fn(T);
    }
}

mod lesson_5 {
    use std::any::Any;
    /////////////////////////////////////////////////////////////////////
    // Suppose these are the real traits from Serde.

    trait Task {}

    trait Generic {
        // Not object safe because of this generic method.
        fn generic_fn<Q: Task>(&self, task: Q);
    }

    // Note: extra constraint `where T: Task` does not seem to be needed.
    impl<'a, T: ?Sized> Task for &'a T {}

    impl<'a, T: ?Sized> Generic for Box<T>
    where
        T: Generic,
    {
        fn generic_fn<Q: Task>(&self, task: Q) {
            println!("impl<'a, T: ?Sized> Generic for Box<T>: calling generic_fn()");

            (**self).generic_fn(task)
        }
    }

    /////////////////////////////////////////////////////////////////////
    // This is an object-safe equivalent that interoperates seamlessly.

    trait AsAny {
        fn as_any(&self) -> &dyn Any;
    }

    trait ErasedGeneric {
        fn erased_fn(&self, task: &dyn Task);

        fn as_any(&self) -> &dyn Any;
    }

    impl Generic for dyn ErasedGeneric {
        // Depending on the trait method signatures and the upstream
        // impls, could also implement for:
        //
        //   - &'a dyn ErasedGeneric
        //   - &'a (dyn ErasedGeneric + Send)
        //   - &'a (dyn ErasedGeneric + Sync)
        //   - &'a (dyn ErasedGeneric + Send + Sync)
        //   - Box<dyn ErasedGeneric>
        //   - Box<dyn ErasedGeneric + Send>
        //   - Box<dyn ErasedGeneric + Sync>
        //   - Box<dyn ErasedGeneric + Send + Sync>
        fn generic_fn<Q: Task>(&self, task: Q) {
            println!("impl Generic for dyn ErasedGeneric: call self.erased_fn()");
            self.erased_fn(&task)
        }
    }

    impl<T> ErasedGeneric for T
    where
        T: Generic + 'static,
    {
        fn erased_fn(&self, task: &dyn Task) {
            println!("erased_fn() caling self.generic_fn");
            self.generic_fn(task)
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    pub fn run() {
        #[derive(Debug)]
        struct T;
        impl Task for T {}

        #[derive(Debug)]
        struct S {
            size: usize,
        }
        impl Generic for S {
            fn generic_fn<Q: Task>(&self, _task: Q) {
                println!("querying the real S");

                let s = self;

                dbg!("{:#?}", s);
            }
        }

        #[derive(Debug)]
        struct M {
            size: usize,
        }
        impl Generic for M {
            fn generic_fn<Q: Task>(&self, _task: Q) {
                println!("querying the real M");

                let s = self;

                dbg!("{:#?}", s);
            }
        }

        struct Tasks {
            tasks: Vec<Box<dyn ErasedGeneric>>,
        }

        // Construct a trait object.
        let mut tasks: Vec<Box<dyn ErasedGeneric>> = vec![];
        let trait_object1: Box<dyn ErasedGeneric> = Box::new(S { size: 0 });
        let trait_object2: Box<dyn ErasedGeneric> = Box::new(M { size: 0 });
        tasks.push(trait_object1);
        tasks.push(trait_object2);

        for (index, task) in tasks.iter().enumerate() {
            // task.generic_fn(T);

            if let Some(inner) = task.as_ref().as_any().downcast_ref::<S>() {
                println!("Downcast index {index} of type S: {:#?}", inner);
            }

            if let Some(inner) = task.as_ref().as_any().downcast_ref::<M>() {
                println!("Downcast index {index} of type M: {:#?}", inner);
            }
        }
    }
}

mod lesson6 {
    use axum::{
        body::Body,
        extract::{FromRequest, FromRequestParts, State},
        handler::{HandlerService as OtherHandlerService, Layered as OtherLayered},
        response::IntoResponse,
        routing::get,
        Router,
    };
    use futures::Future;
    use hyper::body::Buf;
    use hyper::http::{Request, Response};
    use std::{convert::Infallible, fmt, marker::PhantomData, pin::Pin};
    use tower_layer::Layer;
    use tower_service::Service;

    pub struct Layered<L, H, T, S, B, B2> {
        pub layer: L,
        pub handler: H,
        pub _marker: PhantomData<fn() -> (T, S, B, B2)>,
    }

    pub struct HandlerService<H, T, S, B> {
        pub handler: H,
        pub state: S,
        pub _marker: PhantomData<fn() -> (T, B)>,
    }

    pub trait Handler<T, S>: Clone + Send + Sized + 'static {
        /// The type of future calling this handler returns.
        type Future: Future<Output = Response<Self::RespType>> + Send + 'static;

        type RespType;

        /// Call the handler with the given request.
        fn call(self, req: Request<Self::RespType>, state: S) -> Self::Future;

        /// Apply a [`tower::Layer`] to the handler.
        fn layer<L, B, B2>(self, layer: L) -> Layered<L, Self, T, S, B, B2>
        where
            L: Layer<HandlerService<Self, T, S, B>> + Clone,
            L::Service: Service<Request<Self::RespType>>,
        {
            Layered {
                layer,
                handler: self,
                _marker: PhantomData,
            }
        }

        /// Convert the handler into a [`Service`] by providing the state
        fn with_state<B>(
            self,
            state: S,
            _: PhantomData<fn() -> (T, B)>,
        ) -> HandlerService<Self, T, S, B> {
            // NOTE calling `new` is private
            // HandlerService::new(self, state)
            HandlerService {
                handler: self,
                state,
                _marker: PhantomData,
            }
        }
    }

    impl<F, Fut, Res, S> Handler<((),), S> for F
    where
        F: FnOnce() -> Fut + Clone + Send + 'static,
        Fut: Future<Output = Res> + Send,
        Res: IntoResponse,
    {
        type Future = Pin<Box<dyn Future<Output = Response<Self::RespType>> + Send>>;
        type RespType = ();

        fn call(self, _req: Request<Self::RespType>, _state: S) -> Self::Future {
            Box::pin(async move {
                let real_resp = self().await.into_response();

                let response = Response::builder()
                    .status(200)
                    .header("X-Custom-Foo", "Bar")
                    .body(())
                    .unwrap();

                response
            })
        }
    }

    pub fn run() {
        // NOTE this breaks as we clash with existing expansion in axum.
        // let app = Router::new().route("/", get(root));
        //
        // 403  | top_level_handler_fn!(get, GET);
        //      | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `get`
    }

    pub fn root() {
        ()
    }
}

mod lesson7 {
    use super::*;

    // Each caller needs to care about the generic `F` and propagate this type up.
    // Also anyone using `Wrapper` needs to generic and name the type of `F`.
    // pub struct Wrapper<F: Fn()> {
    //     f: F,
    // }

    // This leads to a cleaner interface
    pub struct Wrapper {
        f: Box<dyn Fn()>,
    }

    // Either the following
    //
    // trait X {
    //     fn foo(&self, f: impl Fn());
    // }
    //
    // or
    //
    // trait X {
    //     fn foo<F: Fn()>(&self, f: F);
    // }
    //
    // fn quox(x: &dyn X) {}
    //
    // will not work as the trait needs to be object safe.

    trait X {
        fn foo(&self, f: ClosureType) -> u8;
    }
    // Now, there is only a single 'foo' in the V-table.

    fn quox(x: &dyn X) {}

    pub struct Entity {}

    impl X for Entity {
        fn foo(&self, f: ClosureType) -> u8 {
            let x = (*f)();

            x
        }
    }

    pub fn do_closure() -> u8 {
        10
    }

    pub type ClosureType<'a> = &'a dyn Fn() -> u8;

    pub fn run() {
        let e = Entity {};

        let x: u8 = 10;
        let c = move || x * x;
        let resp = e.foo(&c);
        let resp2 = e.foo(&do_closure);

        info!("lesson 7: resp: {}", resp);
        info!("lesson 7: resp2: {}", resp2);
    }
}

pub mod lesson8 {
    mod error {
        //! Error types.

        use std::error::Error as StdError;
        use std::fmt;
        use std::io::Error as IoError;
        use std::net::TcpStream;
        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        use std::net::TcpStream;
        use std::result;
        use std::str::Utf8Error;

        use base64::DecodeError;
        use bufstream::IntoInnerError as BufError;
        use imap_proto::{types::ResponseCode, Response};
        #[cfg(feature = "native-tls")]
        use native_tls::Error as TlsError;
        #[cfg(feature = "native-tls")]
        use native_tls::HandshakeError as TlsHandshakeError;
        use rustls_connector::HandshakeError;
        #[cfg(feature = "rustls-tls")]
        use rustls_connector::HandshakeError as RustlsHandshakeError;

        /// A convenience wrapper around `Result` for `imap::Error`.
        pub type Result<T> = result::Result<T, Error>;

        /// A BAD response from the server, which indicates an error message from the server.
        #[derive(Debug)]
        #[non_exhaustive]
        pub struct Bad {
            /// Human-redable message included with the Bad response.
            pub information: String,
            /// A more specific error status code included with the Bad response.
            pub code: Option<ResponseCode<'static>>,
        }

        impl fmt::Display for Bad {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.information)
            }
        }

        /// A NO response from the server, which indicates an operational error message from the server.
        #[derive(Debug)]
        #[non_exhaustive]
        pub struct No {
            /// Human-redable message included with the NO response.
            pub information: String,
            /// A more specific error status code included with the NO response.
            pub code: Option<ResponseCode<'static>>,
        }

        impl fmt::Display for No {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.information)
            }
        }

        /// A BYE response from the server, which indicates it is going to hang up on us.
        #[derive(Debug)]
        #[non_exhaustive]
        pub struct Bye {
            /// Human-redable message included with the response.
            pub information: String,
            /// A more specific error status code included with the response.
            pub code: Option<ResponseCode<'static>>,
        }

        impl fmt::Display for Bye {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.information)
            }
        }
        /// A set of errors that can occur in the IMAP client
        #[derive(Debug)]
        #[non_exhaustive]
        pub enum Error {
            /// An `io::Error` that occurred while trying to read or write to a network stream.
            Io(IoError),
            /// An error from the `rustls` library during the TLS handshake.
            #[cfg(feature = "rustls-tls")]
            RustlsHandshake(RustlsHandshakeError<TcpStream>),
            RustlsHandshakeErr(HandshakeError<TcpStream>),

            /// An error from the `native_tls` library during the TLS handshake.
            #[cfg(feature = "native-tls")]
            TlsHandshake(TlsHandshakeError<TcpStream>),
            /// An error from the `native_tls` library while managing the socket.
            #[cfg(feature = "native-tls")]
            Tls(TlsError),
            /// A BAD response from the IMAP server.
            Bad(Bad),
            /// A NO response from the IMAP server.
            No(No),
            /// A BYE response from the IMAP server.
            Bye(Bye),
            /// The connection was terminated unexpectedly.
            ConnectionLost,
            /// Error parsing a server response.
            Parse(ParseError),
            /// Command inputs were not valid [IMAP
            /// strings](https://tools.ietf.org/html/rfc3501#section-4.3).
            Validate(ValidateError),
            /// Error appending an e-mail.
            Append,
            /// An unexpected response was received. This could be a response from a command,
            /// or an unsolicited response that could not be converted into a local type in
            /// [`UnsolicitedResponse`](crate::types::UnsolicitedResponse).
            Unexpected(Response<'static>),
            /// In response to a STATUS command, the server sent OK without actually sending any STATUS
            /// responses first.
            MissingStatusResponse,
            /// StartTls is not available on the server
            StartTlsNotAvailable,
            #[cfg(all(not(feature = "native-tls"), not(feature = "rustls-tls")))]
            /// Returns when Tls is not configured
            TlsNotConfigured,
        }

        impl From<IoError> for Error {
            fn from(err: IoError) -> Error {
                Error::Io(err)
            }
        }

        impl From<ParseError> for Error {
            fn from(err: ParseError) -> Error {
                Error::Parse(err)
            }
        }

        impl<T> From<BufError<T>> for Error {
            fn from(err: BufError<T>) -> Error {
                Error::Io(err.into())
            }
        }

        #[cfg(feature = "rustls-tls")]
        impl From<RustlsHandshakeError<TcpStream>> for Error {
            fn from(err: RustlsHandshakeError<TcpStream>) -> Error {
                Error::RustlsHandshake(err)
            }
        }

        #[cfg(feature = "native-tls")]
        impl From<TlsHandshakeError<TcpStream>> for Error {
            fn from(err: TlsHandshakeError<TcpStream>) -> Error {
                Error::TlsHandshake(err)
            }
        }

        impl From<HandshakeError<TcpStream>> for Error {
            fn from(err: HandshakeError<TcpStream>) -> Error {
                Error::RustlsHandshakeErr(err)
            }
        }

        #[cfg(feature = "native-tls")]
        impl From<TlsError> for Error {
            fn from(err: TlsError) -> Error {
                Error::Tls(err)
            }
        }

        impl<'a> From<Response<'a>> for Error {
            fn from(err: Response<'a>) -> Error {
                Error::Unexpected(err.into_owned())
            }
        }

        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match *self {
                    Error::Io(ref e) => fmt::Display::fmt(e, f),
                    #[cfg(feature = "rustls-tls")]
                    Error::RustlsHandshake(ref e) => fmt::Display::fmt(e, f),
                    Error::RustlsHandshakeErr(ref e) => fmt::Display::fmt(e, f),
                    #[cfg(feature = "native-tls")]
                    Error::Tls(ref e) => fmt::Display::fmt(e, f),
                    #[cfg(feature = "native-tls")]
                    Error::TlsHandshake(ref e) => fmt::Display::fmt(e, f),
                    Error::Validate(ref e) => fmt::Display::fmt(e, f),
                    Error::Parse(ref e) => fmt::Display::fmt(e, f),
                    Error::No(ref data) => write!(f, "No Response: {}", data),
                    Error::Bad(ref data) => write!(f, "Bad Response: {}", data),
                    Error::Bye(ref data) => write!(f, "Bye Response: {}", data),
                    Error::ConnectionLost => f.write_str("Connection Lost"),
                    Error::Append => f.write_str("Could not append mail to mailbox"),
                    Error::Unexpected(ref r) => write!(f, "Unexpected Response: {:?}", r),
                    Error::MissingStatusResponse => write!(f, "Missing STATUS Response"),
                    Error::StartTlsNotAvailable => {
                        write!(f, "StartTls is not available on the server")
                    }
                    #[cfg(all(not(feature = "native-tls"), not(feature = "rustls-tls")))]
                    Error::TlsNotConfigured => write!(f, "No Tls feature is available"),
                }
            }
        }

        impl StdError for Error {
            #[allow(deprecated)]
            fn description(&self) -> &str {
                match *self {
                    Error::Io(ref e) => e.description(),
                    #[cfg(feature = "rustls-tls")]
                    Error::RustlsHandshake(ref e) => e.description(),
                    Error::RustlsHandshakeErr(ref e) => e.description(),
                    #[cfg(feature = "native-tls")]
                    Error::Tls(ref e) => e.description(),
                    #[cfg(feature = "native-tls")]
                    Error::TlsHandshake(ref e) => e.description(),
                    Error::Parse(ref e) => e.description(),
                    Error::Validate(ref e) => e.description(),
                    Error::Bad(_) => "Bad Response",
                    Error::No(_) => "No Response",
                    Error::Bye(_) => "Bye Response",
                    Error::ConnectionLost => "Connection lost",
                    Error::Append => "Could not append mail to mailbox",
                    Error::Unexpected(_) => "Unexpected Response",
                    Error::MissingStatusResponse => "Missing STATUS Response",
                    Error::StartTlsNotAvailable => "StartTls is not available on the server",
                    #[cfg(all(not(feature = "native-tls"), not(feature = "rustls-tls")))]
                    Error::TlsNotConfigured => "No Tls feature is available",
                }
            }

            fn cause(&self) -> Option<&dyn StdError> {
                match *self {
                    Error::Io(ref e) => Some(e),
                    #[cfg(feature = "rustls-tls")]
                    Error::RustlsHandshake(ref e) => Some(e),
                    Error::RustlsHandshakeErr(ref e) => Some(e),
                    #[cfg(feature = "native-tls")]
                    Error::Tls(ref e) => Some(e),
                    #[cfg(feature = "native-tls")]
                    Error::TlsHandshake(ref e) => Some(e),
                    Error::Parse(ParseError::DataNotUtf8(_, ref e)) => Some(e),
                    _ => None,
                }
            }
        }

        /// An error occured while trying to parse a server response.
        #[derive(Debug)]
        pub enum ParseError {
            /// Indicates an error parsing the status response. Such as OK, NO, and BAD.
            Invalid(Vec<u8>),
            /// The client could not find or decode the server's authentication challenge.
            Authentication(String, Option<DecodeError>),
            /// The client received data that was not UTF-8 encoded.
            DataNotUtf8(Vec<u8>, Utf8Error),
        }

        impl fmt::Display for ParseError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match *self {
                    ParseError::Invalid(_) => f.write_str("Unable to parse status response"),
                    ParseError::Authentication(_, _) => {
                        f.write_str("Unable to parse authentication response")
                    }
                    ParseError::DataNotUtf8(_, _) => {
                        f.write_str("Unable to parse data as UTF-8 text")
                    }
                }
            }
        }

        impl StdError for ParseError {
            fn description(&self) -> &str {
                match *self {
                    ParseError::Invalid(_) => "Unable to parse status response",
                    ParseError::Authentication(_, _) => "Unable to parse authentication response",
                    ParseError::DataNotUtf8(_, _) => "Unable to parse data as UTF-8 text",
                }
            }

            fn cause(&self) -> Option<&dyn StdError> {
                match *self {
                    ParseError::Authentication(_, Some(ref e)) => Some(e),
                    _ => None,
                }
            }
        }

        /// An [invalid character](https://tools.ietf.org/html/rfc3501#section-4.3) was found in a command
        /// argument.
        #[derive(Debug)]
        pub struct ValidateError {
            /// the synopsis of the invalid command
            pub(crate) command_synopsis: String,
            /// the name of the invalid argument
            pub(crate) argument: String,
            /// the invalid character contained in the argument
            pub(crate) offending_char: char,
        }

        impl fmt::Display for ValidateError {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                // print character in debug form because invalid ones are often whitespaces
                write!(
                    f,
                    "Invalid character {:?} in argument '{}' of command '{}'",
                    self.offending_char, self.argument, self.command_synopsis
                )
            }
        }

        impl StdError for ValidateError {
            fn description(&self) -> &str {
                "Invalid character in command argument"
            }

            fn cause(&self) -> Option<&dyn StdError> {
                None
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn validate_error_display() {
                assert_eq!(
                    ValidateError {
                        command_synopsis: "COMMAND arg1 arg2".to_owned(),
                        argument: "arg2".to_string(),
                        offending_char: '\n'
                    }
                    .to_string(),
                    "Invalid character '\\n' in argument 'arg2' of command 'COMMAND arg1 arg2'"
                );
            }
        }
    }

    mod client {
        use crate::traits::lesson8::error::{Error, Result};
        use bufstream::BufStream;
        use std::{
            io::{Read, Write},
            ops::{Deref, DerefMut},
        };

        const INITIAL_TAG: u32 = 0;
        const CR: u8 = 0x0d;
        const LF: u8 = 0x0a;

        /// An (unauthenticated) handle to talk to an IMAP server. This is what you get when first
        /// connecting. A succesfull call to [`Client::login`] or [`Client::authenticate`] will return a
        /// [`Session`] instance that provides the usual IMAP methods.
        // Both `Client` and `Session` deref to [`Connection`](struct.Connection.html), the underlying
        // primitives type.
        #[derive(Debug)]
        pub struct Client<T: Read + Write> {
            conn: Connection<T>,
        }

        /// The underlying primitives type. Both `Client`(unauthenticated) and `Session`(after succesful
        /// login) use a `Connection` internally for the TCP stream primitives.
        #[derive(Debug)]
        #[doc(hidden)]
        pub struct Connection<T: Read + Write> {
            pub(crate) stream: BufStream<T>,
            tag: u32,

            /// Enable debug mode for this connection so that all client-server interactions are printed to
            /// `STDERR`.
            pub debug: bool,

            /// Tracks if we have read a greeting.
            pub greeting_read: bool,
        }

        // `Deref` instances are so we can make use of the same underlying primitives in `Client` and
        // `Session`
        impl<T: Read + Write> Deref for Client<T> {
            type Target = Connection<T>;

            fn deref(&self) -> &Connection<T> {
                &self.conn
            }
        }

        impl<T: Read + Write> DerefMut for Client<T> {
            fn deref_mut(&mut self) -> &mut Connection<T> {
                &mut self.conn
            }
        }

        impl<T: Read + Write> Client<T> {
            /// Creates a new client over the given stream.
            ///
            /// This method primarily exists for writing tests that mock the underlying transport,
            /// but can also be used to support IMAP over custom tunnels. If you do not need to do
            /// that, then it is simpler to use the [`ClientBuilder`](crate::ClientBuilder) to get
            /// a new client.
            ///
            /// For an example, see `examples/timeout.rs` which uses a custom timeout on the
            /// tcp stream.
            ///
            /// **Note:** In case you do need to use `Client::new` instead of the `ClientBuilder`
            /// you will need to listen for the IMAP protocol server greeting before authenticating:
            ///
            /// ```rust,no_run
            /// # use imap::Client;
            /// # use std::io;
            /// # use std::net::TcpStream;
            /// # {} #[cfg(feature = "native-tls")]
            /// # fn main() {
            /// # let server = "imap.example.com";
            /// # let username = "";
            /// # let password = "";
            /// # let tcp = TcpStream::connect((server, 993)).unwrap();
            /// # use native_tls::TlsConnector;
            /// # let ssl_connector = TlsConnector::builder().build().unwrap();
            /// # let tls = TlsConnector::connect(&ssl_connector, server.as_ref(), tcp).unwrap();
            /// let mut client = Client::new(tls);
            /// client.read_greeting().unwrap();
            /// let session = client.login(username, password).unwrap();
            /// # }
            /// ```
            pub fn new(stream: T) -> Client<T> {
                Client {
                    conn: Connection {
                        stream: BufStream::new(stream),
                        tag: INITIAL_TAG,
                        debug: false,
                        greeting_read: false,
                    },
                }
            }
        }

        impl<T: Read + Write> Connection<T> {
            /// Read the greeting from the connection. Needs to be done after `connect`ing.
            ///
            /// Panics if called more than once on the same `Connection`.
            pub fn read_greeting(&mut self) -> Result<Vec<u8>> {
                assert!(!self.greeting_read, "Greeting can only be read once");

                let mut v = Vec::new();
                self.readline(&mut v)?;
                self.greeting_read = true;

                Ok(v)
            }

            pub(crate) fn readline(&mut self, into: &mut Vec<u8>) -> Result<usize> {
                use std::io::BufRead;
                let read = self.stream.read_until(LF, into)?;
                if read == 0 {
                    return Err(Error::ConnectionLost);
                }

                if self.debug {
                    // Remove CRLF
                    let len = into.len();
                    let line = &into[(len - read)..(len - 2)];
                    eprintln!("S: {}", String::from_utf8_lossy(line));
                }

                Ok(read)
            }
        }
    }

    mod conn {
        use super::*;
        use crate::traits::lesson8::extensions::idle::SetReadTimeout;
        use std::fmt::{Debug, Formatter};
        use std::io::{Read, Write};

        /// Imap connection trait of a read/write stream
        pub trait ImapConnection: Read + Write + Send + SetReadTimeout + private::Sealed {}

        impl<T> ImapConnection for T where T: Read + Write + Send + SetReadTimeout {}

        impl Debug for dyn ImapConnection {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "Imap connection")
            }
        }

        /// A boxed connection type
        pub type Connection = Box<dyn ImapConnection>;

        mod private {
            use super::{Read, SetReadTimeout, Write};

            pub trait Sealed {}

            impl<T> Sealed for T where T: Read + Write + SetReadTimeout {}
        }
    }

    pub mod extensions {
        pub mod idle {
            use rustls::{ClientConnection, StreamOwned};

            use crate::traits::lesson8::{
                conn::Connection,
                error::{Error, Result},
            };
            use std::{net::TcpStream, ops::DerefMut, time::Duration};

            /// Must be implemented for a transport in order for a `Session` to use IDLE.
            pub trait SetReadTimeout {
                /// Set the timeout for subsequent reads to the given one.
                ///
                /// If `timeout` is `None`, the read timeout should be removed.
                ///
                /// See also `std::net::TcpStream::set_read_timeout`.
                fn set_read_timeout(&mut self, timeout: Option<Duration>) -> Result<()>;
            }

            impl<'a> SetReadTimeout for Connection {
                fn set_read_timeout(&mut self, timeout: Option<Duration>) -> Result<()> {
                    self.deref_mut().set_read_timeout(timeout)
                }
            }

            impl<'a> SetReadTimeout for TcpStream {
                fn set_read_timeout(&mut self, timeout: Option<Duration>) -> Result<()> {
                    TcpStream::set_read_timeout(self, timeout).map_err(Error::Io)
                }
            }
        }
    }

    use self::{client::Client, extensions::idle::SetReadTimeout};
    use conn::{Connection, ImapConnection};
    use error::{Error, Result};
    use std::io::{Read, Write};
    use std::net::TcpStream;

    /// The connection mode we are going to use
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub enum ConnectionMode {
        /// Automatically detect what connection mode should be used.
        /// This will use TLS if the port is 993, and StartTLls if the server says it's available.
        /// If only Plaintext is available it will error out.
        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        AutoTls,
        /// Automatically detect what connection mode should be used.
        /// This will use TLS if the port is 993, and StartTLls if the server says it's available.
        /// Finally it will fallback to Plaintext
        Auto,
        /// A plain unencrypted Tcp connection
        Plaintext,
        /// an encrypted TLS connection
        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        Tls,
        /// a start tls connection
        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        StartTls,
    }

    /// The tls backend to use, either explicit or auto (default)
    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    #[derive(Clone, Debug, Eq, PartialEq)]
    #[non_exhaustive]
    pub enum TlsKind {
        /// Use the NativeTLS backend
        #[cfg(feature = "native-tls")]
        Native,
        /// Use the Rustls backend
        #[cfg(feature = "rustls-tls")]
        Rust,
        /// Use whatever backend is available (uses rustls if both are available)
        Any,
    }

    #[derive(Clone)]
    pub struct ClientBuilder<D>
    where
        D: AsRef<str>,
    {
        domain: D,
        port: u16,
        mode: ConnectionMode,
        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        tls_kind: TlsKind,
        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        skip_tls_verify: bool,
    }

    impl<D> ClientBuilder<D>
    where
        D: AsRef<str>,
    {
        /// Make a new `ClientBuilder` using the given domain and port.
        pub fn new(domain: D, port: u16) -> Self {
            ClientBuilder {
                domain,
                port,
                #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
                mode: ConnectionMode::AutoTls,
                #[cfg(all(not(feature = "native-tls"), not(feature = "rustls-tls")))]
                mode: ConnectionMode::Auto,
                #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
                tls_kind: TlsKind::Any,
                #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
                skip_tls_verify: false,
            }
        }

        /// Sets the Connection mode to use for this connection
        pub fn mode(&mut self, mode: ConnectionMode) -> &mut Self {
            self.mode = mode;
            self
        }

        /// Sets the TLS backend to use for this connection.
        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        pub fn tls_kind(&mut self, kind: TlsKind) -> &mut Self {
            self.tls_kind = kind;
            self
        }

        /// Controls the use of certificate validation.
        ///
        /// Defaults to `false`.
        ///
        /// # Warning
        ///
        /// You should only use this as a last resort as it allows another server to impersonate the
        /// server you think you're talking to, which would include being able to receive your
        /// credentials.
        ///
        /// See [`native_tls::TlsConnectorBuilder::danger_accept_invalid_certs`],
        /// [`native_tls::TlsConnectorBuilder::danger_accept_invalid_hostnames`],
        /// [`rustls::ClientConfig::dangerous`]
        #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
        pub fn danger_skip_tls_verify(&mut self, skip_tls_verify: bool) -> &mut Self {
            self.skip_tls_verify = skip_tls_verify;
            self
        }

        /// Make a [`Client`] using the configuration.
        ///
        /// ```no_run
        /// # use imap::ClientBuilder;
        /// # {} #[cfg(feature = "rustls-tls")]
        /// # fn main() -> Result<(), imap::Error> {
        /// let client = ClientBuilder::new("imap.example.com", 143)
        ///     .starttls().connect()?;
        /// # Ok(())
        /// # }
        /// ```
        pub fn connect(&self) -> Result<Client<Connection>> {
            #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
            return self.connect_with(|_domain, tcp| self.build_tls_connection(tcp));
            #[cfg(all(not(feature = "native-tls"), not(feature = "rustls-tls")))]
            return self.connect_with(|_domain, _tcp| -> Result<Connection> {
                return Err(Error::TlsNotConfigured);
            });
        }

        /// Make a [`Client`] using a custom initialization. This function is intended
        /// to be used if your TLS setup requires custom work such as adding private CAs
        /// or other specific TLS parameters.
        ///
        /// Note, if the connection does not end up as TLS, handshake will not be called.
        /// This can happen if connection mode is set to Tcp, or the server does not support starttls.
        ///
        /// The `handshake` argument should accept two parameters:
        ///
        /// - domain: [`&str`]
        /// - tcp: [`TcpStream`]
        ///
        /// and yield a `Result<C>` where `C` is `Read + Write + Send + SetReadTimeout + 'static,`.
        /// It should only perform TLS initialization over the given `tcp` socket and return the
        /// encrypted stream object, such as a [`native_tls::TlsStream`] or a
        /// [`rustls_connector::TlsStream`].
        ///
        /// If the caller is using `STARTTLS` and previously called [`starttls`](Self::starttls)
        /// then the `tcp` socket given to the `handshake` function will be connected and will
        /// have initiated the `STARTTLS` handshake.
        ///
        /// ```no_run
        /// # use imap::ClientBuilder;
        /// # use rustls_connector::RustlsConnector;
        /// # {} #[cfg(feature = "rustls-tls")]
        /// # fn main() -> Result<(), imap::Error> {
        /// let client = ClientBuilder::new("imap.example.com", 143)
        ///     .starttls()
        ///     .connect_with(|domain, tcp| {
        ///         let ssl_conn = RustlsConnector::new_with_native_certs()?;
        ///         Ok(ssl_conn.connect(domain, tcp)?)
        ///     })?;
        /// # Ok(())
        /// # }
        /// ```
        #[allow(unused_variables)]
        pub fn connect_with<F, C>(&self, handshake: F) -> Result<Client<Connection>>
        where
            F: FnOnce(&str, TcpStream) -> Result<C>,
            C: Read + Write + Send + SetReadTimeout + 'static,
        {
            #[allow(unused_mut)]
            let mut greeting_read = false;
            let tcp = TcpStream::connect((self.domain.as_ref(), self.port))?;

            let stream: Connection = match self.mode {
                #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
                ConnectionMode::AutoTls => {
                    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
                    if self.port == 993 {
                        Box::new(handshake(self.domain.as_ref(), tcp)?)
                    } else {
                        let (stream, upgraded) = self.upgrade_tls(Client::new(tcp), handshake)?;
                        greeting_read = true;

                        if !upgraded {
                            Err(Error::StartTlsNotAvailable)?
                        }
                        stream
                    }
                    #[cfg(all(not(feature = "native-tls"), not(feature = "rustls-tls")))]
                    Err(Error::StartTlsNotAvailable)?
                }
                ConnectionMode::Auto => {
                    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
                    if self.port == 993 {
                        Box::new(handshake(self.domain.as_ref(), tcp)?)
                    } else {
                        let (stream, _upgraded) = self.upgrade_tls(Client::new(tcp), handshake)?;
                        greeting_read = true;

                        stream
                    }
                    #[cfg(all(not(feature = "native-tls"), not(feature = "rustls-tls")))]
                    Box::new(tcp)
                }
                ConnectionMode::Plaintext => Box::new(tcp),
                #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
                ConnectionMode::StartTls => {
                    let (stream, upgraded) = self.upgrade_tls(Client::new(tcp), handshake)?;
                    greeting_read = true;

                    if !upgraded {
                        Err(Error::StartTlsNotAvailable)?
                    }
                    stream
                }
                #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
                ConnectionMode::Tls => Box::new(handshake(self.domain.as_ref(), tcp)?),
            };

            let mut client = Client::new(stream);
            if !greeting_read {
                client.read_greeting()?;
            } else {
                client.greeting_read = true;
            }

            Ok(client)
        }
    }

    pub fn run() -> Result<()> {
        let _client = ClientBuilder::new("imap.example.com", 143).connect_with(|domain, tcp| {
            let ssl_conn = rustls_connector::RustlsConnector::new_with_native_certs()?;
            Ok::<TcpStream, Error>(ssl_conn.connect(domain, tcp)?.sock)
        })?;

        Ok(())
    }
}

mod lesson9 {
    use std::error::Error;
    use std::result::Result;

    pub fn run() -> Result<(), &'static dyn Error> {
        Ok(())
    }
}

/// Nothing to do with Traits, needs to be stashed somewhere else.
mod lesson10 {
    use std::sync::Arc;

    #[derive(Debug, Clone)]
    struct Entity {
        counter: Arc<Box<u8>>,
    }

    impl Entity {
        pub fn new() -> Self {
            Self {
                counter: Arc::new(Box::new(0)),
            }
        }
    }

    pub fn run() {
        let new_box = Entity::new();

        let static_ref: &'static mut u8 = Box::leak(Box::new(**new_box.counter));
        assert_eq!(static_ref, &0);

        *static_ref = 2;
        assert_eq!(static_ref, &2);

        let counter = **new_box.counter;
        assert_eq!(counter, 0);
    }
}
