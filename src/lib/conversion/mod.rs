use anyhow::Result;

pub fn runner() -> Result<()> {
    lesson_1()?;
    lesson_2()?;

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
