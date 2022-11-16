use anyhow::Result;

pub fn runner() -> Result<()> {
    lesson_1()?;
    lesson_2()?;
    embedded_rtc::lesson_3()?;

    Ok(())
}

pub fn lesson_1() -> Result<()> {
    let value: u8 = 10;
    let resp: u16 = value.into();
    assert_eq!(resp as u8, value);

    let resp2 = u16::from(value);
    assert_eq!(resp, resp2);

    Ok(())
}

// This is not possible as we cannot convert from u32 to u16 without truncation and data loss.
//
// --> src/lib/conversion/mod.rs:13:21
// |
// 13 |     let resp: u16 = value.into();
// |                     ^^^^^ ---- required by a bound introduced by this call
// |                     |
// |                     the trait `From<u32>` is not implemented for `u16`
// |
// = help: the following other types implement trait `From<T>`:
//           <f32 as From<i16>>
//           <f32 as From<i8>>
//           <f32 as From<u16>>
//           <f32 as From<u8>>
//           <f64 as From<f32>>
//           <f64 as From<i16>>
//           <f64 as From<i32>>
//           <f64 as From<i8>>
//         and 67 others
// = note: required for `u32` to implement `Into<u16>`
pub fn lesson_2() -> Result<()> {
    let value: u32 = 10;
    // let resp: u16 = value.into();

    Ok(())
}

pub mod embedded_rtc {
    use super::*;
    use std::fmt;

    #[derive(Debug, Clone, Copy)]
    pub enum Weekday {
        Sunday = 1,
        Monday = 2,
        Tuesday = 4,
        Wednesday = 8,
        Thursday = 16,
        Friday = 32,
        Saturday = 64,
    }

    impl fmt::Display for Weekday {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_s())
        }
    }

    impl Weekday {
        pub fn value(&self) -> u8 {
            *self as u8
        }

        pub fn to_s(&self) -> String {
            match self {
                Self::Sunday => "Sunday".to_string(),
                Self::Monday => "Monday".to_string(),
                Self::Tuesday => "Tuesday".to_string(),
                Self::Wednesday => "Wednesday".to_string(),
                Self::Thursday => "Thursday".to_string(),
                Self::Friday => "Friday".to_string(),
                Self::Saturday => "Saturday".to_string(),
            }
        }

        // Returns the first 3-letters of the day of the week
        pub fn to_short(&self) -> Result<String> {
            let day = self.to_s();
            let result: String = day.chars().take(3).collect();

            Ok(result)
        }
    }

    impl From<u8> for Weekday {
        fn from(day: u8) -> Self {
            match day {
                1 => Self::Sunday,
                2 => Self::Monday,
                4 => Self::Tuesday,
                8 => Self::Wednesday,
                16 => Self::Thursday,
                32 => Self::Friday,
                64 => Self::Saturday,
                _ => Self::Sunday,
            }
        }
    }

    pub fn lesson_3() -> Result<()> {
        let weekday_num = 4 as u8;

        // This is possible since we added the `From<_>` trait above
        // via `impl From<u8> for Weekday { //... }`
        let weekday = Weekday::from(weekday_num);

        let weekday_value = weekday.value();
        assert_eq!(weekday_value, 4);
        assert_eq!(weekday.to_s(), "Tuesday");
        assert_eq!(weekday.to_short()?, "Tue");

        Ok(())
    }
}
