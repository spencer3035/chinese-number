/*!
# Chinese Number

Convert primitive numbers to Chinese numbers, or parse Chinese numbers to primitive numbers.

This crate can convert Rust's primitive number data types to Chinese numbers as strings. For example, **123** can be converted into **一百二十三** or **壹佰貳拾參**. It supports both Traditional Chinese and Simple Chinese.

## Example

```rust
extern crate chinese_number;

use chinese_number::{ChineseNumber, ChineseVariant, ChineseNumberToNumber, ChineseNumberCountMethod};

assert_eq!("壹佰貳拾參", 123i8.to_uppercase_ten_thousand(ChineseVariant::Traditional));
assert_eq!("壹佰贰拾参", 123i8.to_uppercase_ten_thousand(ChineseVariant::Simple));

assert_eq!("一百二十三", 123i8.to_lowercase_ten_thousand(ChineseVariant::Traditional));

assert_eq!("十二萬三千四百五十六億七千八百九十萬一千二百三十四兆五千六百七十八萬九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_high(ChineseVariant::Traditional));
assert_eq!("十二萬三千四百五十六京七千八百九十萬一千二百三十四兆五千六百七十八萬九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_middle(ChineseVariant::Traditional));
assert_eq!("十二穰三千四百五十六秭七千八百九十垓一千二百三十四京五千六百七十八兆九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_ten_thousand(ChineseVariant::Traditional));
assert_eq!("一极二载三正四涧五沟六穰七秭八垓九京零一亿二万三千四百五十六", 1234567890123456i64.to_lowercase_low(ChineseVariant::Simple));

assert_eq!("一角二分", 0.12f64.to_lowercase_ten_thousand(ChineseVariant::Traditional));

assert_eq!(123i8, "一百二十三".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
assert_eq!(-30303i16, "負三萬零三百零三".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
assert_eq!(3212345678u32, "三十二億一千二百三十四萬五千六百七十八".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
assert_eq!(10010001001001001000u64, "一千零一京零一兆零一十億零一百萬一千".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());

assert_eq!(1000000u64, "一兆".parse_chinese_number(ChineseNumberCountMethod::Low).unwrap());
assert_eq!(1000000000000u64, "一兆".parse_chinese_number(ChineseNumberCountMethod::TenThousand).unwrap());
assert_eq!(10000000000000000u64, "一兆".parse_chinese_number(ChineseNumberCountMethod::Middle).unwrap());
assert_eq!(10000000000000000u64, "一兆".parse_chinese_number(ChineseNumberCountMethod::High).unwrap());
```
*/

#![no_std]

extern crate alloc;

extern crate chinese_variant;

use alloc::string::String;
use alloc::vec::Vec;

mod constants;

pub(crate) use constants::*;

pub use chinese_variant::ChineseVariant;

mod chinese_number_case;

pub use self::chinese_number_case::ChineseNumberCase;

mod chinese_number_count_method;

pub use self::chinese_number_count_method::ChineseNumberCountMethod;

mod inner_functions;

use self::inner_functions::*;

mod chinese_number_parse_error;

use self::chinese_number_parse_error::ChineseNumberParseError;

/// 將i8整數轉成中文數字。
pub fn from_i8(variant: ChineseVariant, case: ChineseNumberCase, value: i8) -> String {
    let mut s = String::new();

    from_i8_mut(variant, case, value, &mut s);

    s
}

/// 將i8整數轉成中文數字。
pub fn from_i8_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    value: i8,
    buffer: &mut String,
) {
    if value < 0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);

        if value == i8::min_value() {
            from_u8_mut(variant, case, -(i16::from(value)) as u8, buffer)
        } else {
            from_u8_mut(variant, case, -value as u8, buffer)
        }
    } else {
        from_u8_mut(variant, case, value as u8, buffer)
    }
}

/// 將u8整數轉成中文數字。
pub fn from_u8(variant: ChineseVariant, case: ChineseNumberCase, value: u8) -> String {
    let mut s = String::new();

    from_u8_mut(variant, case, value, &mut s);

    s
}

/// 將u8整數轉成中文數字。
pub fn from_u8_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    value: u8,
    buffer: &mut String,
) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    if value >= 100 {
        digit_100(chinese_number_index, value as usize, buffer);
    } else if value >= 10 {
        digit_10(chinese_number_index, value as usize, buffer, false);
    } else {
        digit_1(chinese_number_index, value as usize, buffer);
    }
}

/// 將i16整數轉成中文數字。
pub fn from_i16(variant: ChineseVariant, case: ChineseNumberCase, value: i16) -> String {
    let mut s = String::new();

    from_i16_mut(variant, case, value, &mut s);

    s
}

/// 將i16整數轉成中文數字。
pub fn from_i16_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    value: i16,
    buffer: &mut String,
) {
    if value < 0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);

        if value == i16::min_value() {
            from_u16_mut(variant, case, -(i32::from(value)) as u16, buffer)
        } else {
            from_u16_mut(variant, case, -value as u16, buffer)
        }
    } else {
        from_u16_mut(variant, case, value as u16, buffer)
    }
}

/// 將u16整數轉成中文數字。
pub fn from_u16(variant: ChineseVariant, case: ChineseNumberCase, value: u16) -> String {
    let mut s = String::new();

    from_u16_mut(variant, case, value, &mut s);

    s
}

/// 將u16整數轉成中文數字。
pub fn from_u16_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    value: u16,
    buffer: &mut String,
) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    digit_10000_compat(chinese_number_index, value as usize, buffer, false);
}

/// 將i32整數轉成中文數字。
pub fn from_i32(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: i32,
) -> String {
    let mut s = String::new();

    from_i32_mut(variant, case, method, value, &mut s);

    s
}

/// 將i32整數轉成中文數字。
pub fn from_i32_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: i32,
    buffer: &mut String,
) {
    if value < 0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);

        if value == i32::min_value() {
            from_u32_mut(variant, case, method, -(i64::from(value)) as u32, buffer)
        } else {
            from_u32_mut(variant, case, method, -value as u32, buffer)
        }
    } else {
        from_u32_mut(variant, case, method, value as u32, buffer)
    }
}

/// 將u32整數轉成中文數字。
pub fn from_u32(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: u32,
) -> String {
    let mut s = String::new();

    from_u32_mut(variant, case, method, value, &mut s);

    s
}

/// 將u32整數轉成中文數字。
pub fn from_u32_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: u32,
    buffer: &mut String,
) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    match method {
        ChineseNumberCountMethod::Low => {
            digit_compat_low_u32(chinese_number_index, value, buffer);
        }
        ChineseNumberCountMethod::TenThousand
        | ChineseNumberCountMethod::Middle
        | ChineseNumberCountMethod::High => {
            digit_compat_ten_thousand_u32(chinese_number_index, value, buffer);
        }
    }
}

/// 將i64整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_i64(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: i64,
) -> String {
    let mut s = String::new();

    from_i64_mut(variant, case, method, value, &mut s);

    s
}

/// 將i64整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_i64_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: i64,
    buffer: &mut String,
) {
    if value < 0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);

        if value == i64::min_value() {
            from_u64_mut(variant, case, method, -(i128::from(value)) as u64, buffer)
        } else {
            from_u64_mut(variant, case, method, -value as u64, buffer)
        }
    } else {
        from_u64_mut(variant, case, method, value as u64, buffer)
    }
}

/// 將u64整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_u64(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: u64,
) -> String {
    let mut s = String::new();

    from_u64_mut(variant, case, method, value, &mut s);

    s
}

/// 將u64整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_u64_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: u64,
    buffer: &mut String,
) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    match method {
        ChineseNumberCountMethod::Low => {
            digit_compat_low_u64(chinese_number_index, value, buffer);
        }
        ChineseNumberCountMethod::TenThousand => {
            digit_compat_ten_thousand_u64(chinese_number_index, value, buffer);
        }
        ChineseNumberCountMethod::Middle => {
            digit_compat_middle_u64(chinese_number_index, value, buffer);
        }
        ChineseNumberCountMethod::High => {
            digit_compat_high_u128(chinese_number_index, u128::from(value), buffer);
        }
    }
}

/// 將i128整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_i128(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: i128,
) -> String {
    let mut s = String::new();

    from_i128_mut(variant, case, method, value, &mut s);

    s
}

/// 將i128整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_i128_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: i128,
    buffer: &mut String,
) {
    if value < 0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);

        if value == i128::min_value() {
            from_u128_mut(variant, case, method, -((value + 1) as i128) as u128 + 1, buffer)
        } else {
            from_u128_mut(variant, case, method, -value as u128, buffer)
        }
    } else {
        from_u128_mut(variant, case, method, value as u128, buffer)
    }
}

/// 將u128整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_u128(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: u128,
) -> String {
    let mut s = String::new();

    from_u128_mut(variant, case, method, value, &mut s);

    s
}

/// 將u128整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_u128_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: u128,
    buffer: &mut String,
) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    match method {
        ChineseNumberCountMethod::Low => {
            assert!(value < 10_000_000_000_000_000); // support to "極"
            digit_compat_low_u64(chinese_number_index, value as u64, buffer);
        }
        ChineseNumberCountMethod::TenThousand => {
            digit_compat_ten_thousand_u128(chinese_number_index, value as u128, buffer);
        }
        ChineseNumberCountMethod::Middle => {
            digit_compat_middle_u128(chinese_number_index, value as u128, buffer);
        }
        ChineseNumberCountMethod::High => {
            digit_compat_high_u128(chinese_number_index, value as u128, buffer);
        }
    }
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "8")]
pub fn from_isize(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: isize,
) -> String {
    from_i8(variant, case, value as i8)
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "8")]
pub fn from_isize_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: isize,
    buffer: &mut String,
) {
    from_i8_mut(variant, case, value as i8, buffer)
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "16")]
pub fn from_isize(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: isize,
) -> String {
    from_i16(variant, case, value as i16)
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "16")]
pub fn from_isize_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: isize,
    buffer: &mut String,
) {
    from_i16_mut(variant, case, value as i16, buffer)
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "32")]
pub fn from_isize(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: isize,
) -> String {
    from_i32(variant, case, method, value as i32)
}

/// 將isize整數轉成中文數字。
#[cfg(target_pointer_width = "32")]
pub fn from_isize_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: isize,
    buffer: &mut String,
) {
    from_i32_mut(variant, case, method, value as i32, buffer)
}

/// 將isize整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[cfg(target_pointer_width = "64")]
pub fn from_isize(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: isize,
) -> String {
    from_i64(variant, case, method, value as i64)
}

/// 將isize整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[cfg(target_pointer_width = "64")]
pub fn from_isize_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: isize,
    buffer: &mut String,
) {
    from_i64_mut(variant, case, method, value as i64, buffer)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "8")]
pub fn from_usize(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: usize,
) -> String {
    from_u8(variant, case, value as u8)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "8")]
pub fn from_usize_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: usize,
    buffer: &mut String,
) {
    from_u8_mut(variant, case, value as u8, buffer)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "16")]
pub fn from_usize(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: usize,
) -> String {
    from_u16(variant, case, value as u16)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "16")]
pub fn from_usize_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    _method: ChineseNumberCountMethod,
    value: usize,
    buffer: &mut String,
) {
    from_u16_mut(variant, case, value as u16, buffer)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "32")]
pub fn from_usize(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: usize,
) -> String {
    from_u32(variant, case, method, value as u32)
}

/// 將usize整數轉成中文數字。
#[cfg(target_pointer_width = "32")]
pub fn from_usize_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: usize,
    buffer: &mut String,
) {
    from_u32_mut(variant, case, method, value as u32, buffer)
}

/// 將usize整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[cfg(target_pointer_width = "64")]
pub fn from_usize(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: usize,
) -> String {
    from_u64(variant, case, method, value as u64)
}

/// 將usize整數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
#[cfg(target_pointer_width = "64")]
pub fn from_usize_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: usize,
    buffer: &mut String,
) {
    from_u64_mut(variant, case, method, value as u64, buffer)
}

/// 將f64浮點數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_f64(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: f64,
) -> String {
    let mut s = String::new();

    from_f64_mut(variant, case, method, value, &mut s);

    s
}

/// 將f64浮點數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_f64_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    mut value: f64,
    buffer: &mut String,
) {
    let chinese_number_index = get_chinese_number_index(variant, case);

    if value < 0.0 {
        buffer.push_str(CHINESE_NEGATIVE_SIGN[variant as usize]);
        value = -value;
    }

    match method {
        ChineseNumberCountMethod::Low => {
            fraction_compat_low(chinese_number_index, value, buffer);
        }
        ChineseNumberCountMethod::TenThousand => {
            fraction_compat_ten_thousand(chinese_number_index, value, buffer);
        }
        ChineseNumberCountMethod::Middle => {
            fraction_compat_middle(chinese_number_index, value, buffer);
        }
        ChineseNumberCountMethod::High => {
            fraction_compat_high(chinese_number_index, value, buffer);
        }
    }
}

/// 將f32浮點數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_f32(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: f32,
) -> String {
    from_f64(variant, case, method, f64::from(value))
}

/// 將f32浮點數轉成中文數字。如果使用 **「下數」** 來作為單位標準，數值不能大於或等於10000000000000000。
pub fn from_f32_mut(
    variant: ChineseVariant,
    case: ChineseNumberCase,
    method: ChineseNumberCountMethod,
    value: f32,
    buffer: &mut String,
) {
    from_f64_mut(variant, case, method, f64::from(value), buffer);
}

#[cfg(test)]
mod tests;

/// 讓Rust程式語言的所有基本數值型別擁有中文數字的轉換能力。
pub trait ChineseNumber {
    /// 轉成大寫數字，使用 **「上數」** 作為單位標準。
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String;
    /// 轉成大寫數字，使用 **「上數」** 作為單位標準。
    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成大寫數字，使用 **「中數」** 作為單位標準。
    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String;
    /// 轉成大寫數字，使用 **「中數」** 作為單位標準。
    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成大寫數字，使用 **「萬進」** 作為單位標準。
    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String;
    /// 轉成大寫數字，使用 **「萬進」** 作為單位標準。
    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成大寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    fn to_uppercase_low(&self, variant: ChineseVariant) -> String;
    /// 轉成大寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成小寫數字，使用 **「上數」** 作為單位標準。
    fn to_lowercase_high(&self, variant: ChineseVariant) -> String;
    /// 轉成小寫數字，使用 **「上數」** 作為單位標準。
    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成小寫數字，使用 **「中數」** 作為單位標準。
    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String;
    /// 轉成小寫數字，使用 **「中數」** 作為單位標準。
    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成小寫數字，使用 **「萬進」** 作為單位標準。
    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String;
    /// 轉成小寫數字，使用 **「萬進」** 作為單位標準。
    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String);
    /// 轉成小寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    fn to_lowercase_low(&self, variant: ChineseVariant) -> String;
    /// 轉成小寫數字，使用 **「下數」** 作為單位標準。數值不能大於或等於10000000000000000。
    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String);
}

impl ChineseNumber for i8 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_i8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }
}

impl ChineseNumber for u8 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_u8(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u8_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }
}

impl ChineseNumber for i16 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_i16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }
}

impl ChineseNumber for u16 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Upper, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Upper, *self, buffer)
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_u16(variant, ChineseNumberCase::Lower, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u16_mut(variant, ChineseNumberCase::Lower, *self, buffer)
    }
}

impl ChineseNumber for i32 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_i32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }
}

impl ChineseNumber for u32 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_u32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }
}

impl ChineseNumber for i64 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_i64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }
}

impl ChineseNumber for u64 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_u64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }
}

impl ChineseNumber for i128 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_i128(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_i128_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }
}

impl ChineseNumber for u128 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_u128(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_u128_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }
}

impl ChineseNumber for isize {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_isize(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_isize_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }
}

impl ChineseNumber for usize {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_usize(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_usize_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }
}

impl ChineseNumber for f64 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_f64(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f64_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }
}

impl ChineseNumber for f32 {
    fn to_uppercase_high(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::High, *self)
    }

    fn to_uppercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_uppercase_middle(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_uppercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_uppercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_uppercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_uppercase_low(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Upper, ChineseNumberCountMethod::Low, *self)
    }

    fn to_uppercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(
            variant,
            ChineseNumberCase::Upper,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }

    fn to_lowercase_high(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::High, *self)
    }

    fn to_lowercase_high_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::High,
            *self,
            buffer,
        )
    }

    fn to_lowercase_middle(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Middle, *self)
    }

    fn to_lowercase_middle_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Middle,
            *self,
            buffer,
        )
    }

    fn to_lowercase_ten_thousand(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::TenThousand, *self)
    }

    fn to_lowercase_ten_thousand_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::TenThousand,
            *self,
            buffer,
        )
    }

    fn to_lowercase_low(&self, variant: ChineseVariant) -> String {
        from_f32(variant, ChineseNumberCase::Lower, ChineseNumberCountMethod::Low, *self)
    }

    fn to_lowercase_low_mut(&self, variant: ChineseVariant, buffer: &mut String) {
        from_f32_mut(
            variant,
            ChineseNumberCase::Lower,
            ChineseNumberCountMethod::Low,
            *self,
            buffer,
        )
    }
}

/// 將中文數字轉成i8數值。
pub fn parse_chinese_number_to_i8<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i8, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => {
                        match chinese_digit_100_compat(
                            next_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i8::max_value() as u16 + 1 {
                                    Err(ChineseNumberParseError::Underflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 6,
                                    })
                                } else {
                                    Ok(-(number as i16) as i8)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                    None => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 1,
                        })
                    }
                }
            } else {
                match chinese_digit_100_compat(
                    first_char,
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                ) {
                    Ok(number) => {
                        if number > i8::max_value() as u16 {
                            Err(ChineseNumberParseError::Overflow)
                        } else if chars.next().is_some() {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: 5,
                            })
                        } else {
                            Ok(number as i8)
                        }
                    }
                    Err(err) => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err,
                        })
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成u8數值。
pub fn parse_chinese_number_to_u8<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u8, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match chinese_digit_100_compat(
                first_char,
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
            ) {
                Ok(number) => {
                    if number > u16::from(u8::max_value()) {
                        Err(ChineseNumberParseError::Overflow)
                    } else if chars.next().is_some() {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 5,
                        })
                    } else {
                        Ok(number as u8)
                    }
                }
                Err(err) => {
                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err,
                    })
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成i16數值。
pub fn parse_chinese_number_to_i16<S: AsRef<str>>(
    chinese_number: S,
) -> Result<i16, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => {
                        match chinese_digit_10000_ten_thousand_compat(
                            next_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i16::max_value() as u32 + 1 {
                                    Err(ChineseNumberParseError::Underflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 16,
                                    })
                                } else {
                                    Ok(-(number as i32) as i16)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                    None => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 1,
                        })
                    }
                }
            } else {
                match chinese_digit_10000_ten_thousand_compat(
                    first_char,
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                    chars.next(),
                ) {
                    Ok(number) => {
                        if number > i16::max_value() as u32 {
                            Err(ChineseNumberParseError::Overflow)
                        } else if chars.next().is_some() {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: 15,
                            })
                        } else {
                            Ok(number as i16)
                        }
                    }
                    Err(err) => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: err,
                        })
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成u16數值。
pub fn parse_chinese_number_to_u16<S: AsRef<str>>(
    chinese_number: S,
) -> Result<u16, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match chinese_digit_10000_ten_thousand_compat(
                first_char,
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
                chars.next(),
            ) {
                Ok(number) => {
                    if number > u32::from(u16::max_value()) {
                        Err(ChineseNumberParseError::Overflow)
                    } else if chars.next().is_some() {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 15,
                        })
                    } else {
                        Ok(number as u16)
                    }
                }
                Err(err) => {
                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                        char_index: err,
                    })
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成i32數值。
pub fn parse_chinese_number_to_i32<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<i32, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => {
                        match method {
                            ChineseNumberCountMethod::Low => {
                                match chinese_digit_1000000000_low_compat(
                                    next_char,
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                ) {
                                    Ok(number) => {
                                        if number > i32::max_value() as u64 + 1 {
                                            Err(ChineseNumberParseError::Underflow)
                                        } else if chars.next().is_some() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 20,
                                            })
                                        } else {
                                            Ok(-(number as i64) as i32)
                                        }
                                    }
                                    Err(err) => {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: err + 1,
                                        })
                                    }
                                }
                            }
                            ChineseNumberCountMethod::TenThousand
                            | ChineseNumberCountMethod::Middle
                            | ChineseNumberCountMethod::High => {
                                match chinese_digit_100000000_ten_thousand_compat(
                                    next_char,
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                ) {
                                    Ok(number) => {
                                        if number > i32::max_value() as u64 + 1 {
                                            Err(ChineseNumberParseError::Underflow)
                                        } else if chars.next().is_some() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 24,
                                            })
                                        } else {
                                            Ok(-(number as i64) as i32)
                                        }
                                    }
                                    Err(err) => {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: err + 1,
                                        })
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 1,
                        })
                    }
                }
            } else {
                match method {
                    ChineseNumberCountMethod::Low => {
                        match chinese_digit_1000000000_low_compat(
                            first_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i32::max_value() as u64 {
                                    Err(ChineseNumberParseError::Overflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 19,
                                    })
                                } else {
                                    Ok(number as i32)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                    ChineseNumberCountMethod::TenThousand
                    | ChineseNumberCountMethod::Middle
                    | ChineseNumberCountMethod::High => {
                        match chinese_digit_100000000_ten_thousand_compat(
                            first_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i32::max_value() as u64 {
                                    Err(ChineseNumberParseError::Overflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 23,
                                    })
                                } else {
                                    Ok(number as i32)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成u32數值。
pub fn parse_chinese_number_to_u32<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<u32, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match method {
                ChineseNumberCountMethod::Low => {
                    match chinese_digit_1000000000_low_compat(
                        first_char,
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                    ) {
                        Ok(number) => {
                            if number > u64::from(u32::max_value()) {
                                Err(ChineseNumberParseError::Overflow)
                            } else if chars.next().is_some() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 19,
                                })
                            } else {
                                Ok(number as u32)
                            }
                        }
                        Err(err) => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: err,
                            })
                        }
                    }
                }
                ChineseNumberCountMethod::TenThousand
                | ChineseNumberCountMethod::Middle
                | ChineseNumberCountMethod::High => {
                    match chinese_digit_100000000_ten_thousand_compat(
                        first_char,
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                    ) {
                        Ok(number) => {
                            if number > u64::from(u32::max_value()) {
                                Err(ChineseNumberParseError::Overflow)
                            } else if chars.next().is_some() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 23,
                                })
                            } else {
                                Ok(number as u32)
                            }
                        }
                        Err(err) => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: err,
                            })
                        }
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成i64數值。
pub fn parse_chinese_number_to_i64<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<i64, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            if CHINESE_NEGATIVE_SIGN_CHARS.contains(&first_char) {
                let next_char = chars.next();

                match next_char {
                    Some(next_char) => {
                        match method {
                            ChineseNumberCountMethod::Low => {
                                match chinese_digit_1000000000000000_low_compat(
                                    next_char,
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                ) {
                                    Ok(number) => {
                                        if chars.next().is_some() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 32,
                                            })
                                        } else {
                                            Ok(-(number as i64))
                                        }
                                    }
                                    Err(err) => {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: err + 1,
                                        })
                                    }
                                }
                            }
                            ChineseNumberCountMethod::TenThousand => {
                                match chinese_digit_10000000000000000_ten_thousand_compat(
                                    next_char,
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                ) {
                                    Ok(number) => {
                                        if number > i64::max_value() as u128 + 1 {
                                            Err(ChineseNumberParseError::Underflow)
                                        } else if chars.next().is_some() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 40,
                                            })
                                        } else {
                                            Ok(-(number as i128) as i64)
                                        }
                                    }
                                    Err(err) => {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: err + 1,
                                        })
                                    }
                                }
                            }
                            ChineseNumberCountMethod::Middle | ChineseNumberCountMethod::High => {
                                match chinese_digit_10000000000000000_middle_compat(
                                    next_char,
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                    chars.next(),
                                ) {
                                    Ok(number) => {
                                        if number > i64::max_value() as u128 + 1 {
                                            Err(ChineseNumberParseError::Underflow)
                                        } else if chars.next().is_some() {
                                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                                char_index: 48,
                                            })
                                        } else {
                                            Ok(-(number as i128) as i64)
                                        }
                                    }
                                    Err(err) => {
                                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                            char_index: err + 1,
                                        })
                                    }
                                }
                            }
                        }
                    }
                    None => {
                        Err(ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: 1,
                        })
                    }
                }
            } else {
                match method {
                    ChineseNumberCountMethod::Low => {
                        match chinese_digit_1000000000000000_low_compat(
                            first_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i64::max_value() as u64 {
                                    Err(ChineseNumberParseError::Overflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 31,
                                    })
                                } else {
                                    Ok(number as i64)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                    ChineseNumberCountMethod::TenThousand => {
                        match chinese_digit_10000000000000000_ten_thousand_compat(
                            first_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i64::max_value() as u128 {
                                    Err(ChineseNumberParseError::Overflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 39,
                                    })
                                } else {
                                    Ok(number as i64)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                    ChineseNumberCountMethod::Middle | ChineseNumberCountMethod::High => {
                        match chinese_digit_10000000000000000_middle_compat(
                            first_char,
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                            chars.next(),
                        ) {
                            Ok(number) => {
                                if number > i64::max_value() as u128 {
                                    Err(ChineseNumberParseError::Overflow)
                                } else if chars.next().is_some() {
                                    Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                        char_index: 47,
                                    })
                                } else {
                                    Ok(number as i64)
                                }
                            }
                            Err(err) => {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: err,
                                })
                            }
                        }
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成u64數值。
pub fn parse_chinese_number_to_u64<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<u64, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let mut chars = chinese_number.chars();

    let first_char = chars.next();

    match first_char {
        Some(first_char) => {
            match method {
                ChineseNumberCountMethod::Low => {
                    match chinese_digit_1000000000000000_low_compat(
                        first_char,
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                    ) {
                        Ok(number) => {
                            if number > u64::max_value() {
                                Err(ChineseNumberParseError::Overflow)
                            } else if chars.next().is_some() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 31,
                                })
                            } else {
                                Ok(number)
                            }
                        }
                        Err(err) => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: err,
                            })
                        }
                    }
                }
                ChineseNumberCountMethod::TenThousand => {
                    match chinese_digit_10000000000000000_ten_thousand_compat(
                        first_char,
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                    ) {
                        Ok(number) => {
                            if number > u128::from(u64::max_value()) {
                                Err(ChineseNumberParseError::Overflow)
                            } else if chars.next().is_some() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 39,
                                })
                            } else {
                                Ok(number as u64)
                            }
                        }
                        Err(err) => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: err,
                            })
                        }
                    }
                }
                ChineseNumberCountMethod::Middle | ChineseNumberCountMethod::High => {
                    match chinese_digit_10000000000000000_middle_compat(
                        first_char,
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                        chars.next(),
                    ) {
                        Ok(number) => {
                            if number > u128::from(u64::max_value()) {
                                Err(ChineseNumberParseError::Overflow)
                            } else if chars.next().is_some() {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: 47,
                                })
                            } else {
                                Ok(number as u64)
                            }
                        }
                        Err(err) => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: err,
                            })
                        }
                    }
                }
            }
        }
        None => Err(ChineseNumberParseError::ChineseNumberEmpty),
    }
}

/// 將中文數字轉成i128數值。
pub fn parse_chinese_number_to_i128<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    _chinese_number: S,
) -> Result<i128, ChineseNumberParseError> {
    unimplemented!()
}

/// 將中文數字轉成u128數值。
pub fn parse_chinese_number_to_u128<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    _chinese_number: S,
) -> Result<u128, ChineseNumberParseError> {
    unimplemented!()
}

/// 將中文數字轉成isize數值。
#[cfg(target_pointer_width = "8")]
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i8(chinese_number).map(|n| n as isize)
}

/// 將中文數字轉成usize數值。
#[cfg(target_pointer_width = "8")]
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u8(chinese_number).map(|n| n as usize)
}

/// 將中文數字轉成isize數值。
#[cfg(target_pointer_width = "16")]
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i16(chinese_number).map(|n| n as isize)
}

/// 將中文數字轉成usize數值。
#[cfg(target_pointer_width = "16")]
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(
    _method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u16(chinese_number).map(|n| n as usize)
}

/// 將中文數字轉成isize數值。
#[cfg(target_pointer_width = "32")]
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i32(method, chinese_number).map(|n| n as isize)
}

/// 將中文數字轉成usize數值。
#[cfg(target_pointer_width = "32")]
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u32(method, chinese_number).map(|n| n as usize)
}

/// 將中文數字轉成isize數值。
#[cfg(target_pointer_width = "64")]
pub fn parse_chinese_number_to_isize<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<isize, ChineseNumberParseError> {
    parse_chinese_number_to_i64(method, chinese_number).map(|n| n as isize)
}

/// 將中文數字轉成usize數值。
#[cfg(target_pointer_width = "64")]
pub fn parse_chinese_number_to_usize<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<usize, ChineseNumberParseError> {
    parse_chinese_number_to_u64(method, chinese_number).map(|n| n as usize)
}

/// 將中文數字轉成f64數值。
pub fn parse_chinese_number_to_f64<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<f64, ChineseNumberParseError> {
    let chinese_number = chinese_number.as_ref().replace(" ", "");

    let len = chinese_number.len();

    if len == 0 {
        return Err(ChineseNumberParseError::ChineseNumberEmpty);
    }

    let mut integer_index = len;

    for c in CHINESE_NUMBERS_FRACTION[0].iter() {
        if let Some(index) = chinese_number.find(c) {
            integer_index = index;
            break;
        }
    }

    if integer_index == len {
        for c in CHINESE_NUMBERS_FRACTION[1].iter() {
            if let Some(index) = chinese_number.find(c) {
                integer_index = index;
                break;
            }
        }
    }

    if integer_index == 0 {
        Err(ChineseNumberParseError::ChineseNumberIncorrect {
            char_index: 0,
        })
    } else if integer_index == len {
        parse_chinese_number_to_i64(method, chinese_number).map(|result| result as f64)
    } else {
        let mut integer_chars: Vec<char> = chinese_number[..integer_index].chars().collect();

        let integer_chars_len_dec = integer_chars.len() - 1;

        let last_char = integer_chars.remove(integer_chars_len_dec);

        let integer_part: String = integer_chars.iter().collect();

        let integer_number = parse_chinese_number_to_i64(method, integer_part)?;

        let fd1 = chinese_digit_1(last_char).map_err(|_| {
            ChineseNumberParseError::ChineseNumberIncorrect {
                char_index: integer_chars_len_dec,
            }
        })?;

        let mut fraction_chars = chinese_number[integer_index..].chars();

        let unit1 = fraction_chars.next().unwrap();

        let next_char = fraction_chars.next();

        if CHINESE_NUMBERS_FRACTION_CHARS[0].contains(&unit1) {
            match next_char {
                Some(next_char) => {
                    let fd2 = chinese_digit_1(next_char).map_err(|_| {
                        ChineseNumberParseError::ChineseNumberIncorrect {
                            char_index: integer_chars_len_dec + 2,
                        }
                    })?;

                    let unit2 = fraction_chars.next();

                    match unit2 {
                        Some(unit2) => {
                            if CHINESE_NUMBERS_FRACTION_CHARS[1].contains(&unit2) {
                                if integer_number >= 0 {
                                    Ok(integer_number as f64
                                        + f64::from(fd1) * 0.1
                                        + f64::from(fd2) * 0.01)
                                } else {
                                    Ok(integer_number as f64
                                        - f64::from(fd1) * 0.1
                                        - f64::from(fd2) * 0.01)
                                }
                            } else {
                                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                    char_index: integer_chars_len_dec + 3,
                                })
                            }
                        }
                        None => {
                            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                                char_index: integer_chars_len_dec + 3,
                            })
                        }
                    }
                }
                None => {
                    if integer_number >= 0 {
                        Ok(integer_number as f64 + f64::from(fd1) * 0.1)
                    } else {
                        Ok(integer_number as f64 - f64::from(fd1) * 0.1)
                    }
                }
            }
        } else if CHINESE_NUMBERS_FRACTION_CHARS[1].contains(&unit1) {
            if next_char.is_some() {
                Err(ChineseNumberParseError::ChineseNumberIncorrect {
                    char_index: integer_chars_len_dec + 2,
                })
            } else if integer_number >= 0 {
                Ok(integer_number as f64 + f64::from(fd1) * 0.01)
            } else {
                Ok(integer_number as f64 - f64::from(fd1) * 0.01)
            }
        } else {
            Err(ChineseNumberParseError::ChineseNumberIncorrect {
                char_index: integer_chars_len_dec + 1,
            })
        }
    }
}

/// 將中文數字轉成f32數值。
pub fn parse_chinese_number_to_f32<S: AsRef<str>>(
    method: ChineseNumberCountMethod,
    chinese_number: S,
) -> Result<f32, ChineseNumberParseError> {
    parse_chinese_number_to_f64(method, chinese_number).map(|result| result as f32)
}

/// 讓Rust程式語言的字串型別擁有中文數字的轉換能力。
pub trait ChineseNumberToNumber<T> {
    /// 將中文數字轉成基本型別之數值。
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<T, ChineseNumberParseError>;
}

impl<T: AsRef<str>> ChineseNumberToNumber<i8> for T {
    fn parse_chinese_number(
        &self,
        _method: ChineseNumberCountMethod,
    ) -> Result<i8, ChineseNumberParseError> {
        parse_chinese_number_to_i8(self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<u8> for T {
    fn parse_chinese_number(
        &self,
        _method: ChineseNumberCountMethod,
    ) -> Result<u8, ChineseNumberParseError> {
        parse_chinese_number_to_u8(self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<i16> for T {
    fn parse_chinese_number(
        &self,
        _method: ChineseNumberCountMethod,
    ) -> Result<i16, ChineseNumberParseError> {
        parse_chinese_number_to_i16(self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<u16> for T {
    fn parse_chinese_number(
        &self,
        _method: ChineseNumberCountMethod,
    ) -> Result<u16, ChineseNumberParseError> {
        parse_chinese_number_to_u16(self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<i32> for T {
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<i32, ChineseNumberParseError> {
        parse_chinese_number_to_i32(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<u32> for T {
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<u32, ChineseNumberParseError> {
        parse_chinese_number_to_u32(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<i64> for T {
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<i64, ChineseNumberParseError> {
        parse_chinese_number_to_i64(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<u64> for T {
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<u64, ChineseNumberParseError> {
        parse_chinese_number_to_u64(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<i128> for T {
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<i128, ChineseNumberParseError> {
        parse_chinese_number_to_i128(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<u128> for T {
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<u128, ChineseNumberParseError> {
        parse_chinese_number_to_u128(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<isize> for T {
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<isize, ChineseNumberParseError> {
        parse_chinese_number_to_isize(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<usize> for T {
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<usize, ChineseNumberParseError> {
        parse_chinese_number_to_usize(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<f64> for T {
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<f64, ChineseNumberParseError> {
        parse_chinese_number_to_f64(method, self)
    }
}

impl<T: AsRef<str>> ChineseNumberToNumber<f32> for T {
    fn parse_chinese_number(
        &self,
        method: ChineseNumberCountMethod,
    ) -> Result<f32, ChineseNumberParseError> {
        parse_chinese_number_to_f32(method, self)
    }
}
