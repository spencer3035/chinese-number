Chinese Number
====================

[![Build Status](https://travis-ci.org/magiclen/chinese-number.svg?branch=master)](https://travis-ci.org/magiclen/chinese-number)
[![Build status](https://ci.appveyor.com/api/projects/status/nofhu6rdlsqi2xdk/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/chinese-number/branch/master)

Convert primitive numbers to Chinese numbers, or parse Chinese numbers to primitive numbers.

This crate can convert Rust's primitive number data types to Chinese numbers as strings. For instance, **123** can be converted into **一百二十三** or **壹佰貳拾參**. It supports both Traditional Chinese and Simple Chinese.

## Example

```rust
extern crate chinese_number;

use chinese_number::{ChineseNumber, ChineseVariant};

assert_eq!("壹佰貳拾參", 123i8.to_uppercase_ten_thousand(ChineseVariant::Traditional));
assert_eq!("壹佰贰拾参", 123i8.to_uppercase_ten_thousand(ChineseVariant::Simple));

assert_eq!("一百二十三", 123i8.to_lowercase_ten_thousand(ChineseVariant::Traditional));

assert_eq!("十二萬三千四百五十六億七千八百九十萬一千二百三十四兆五千六百七十八萬九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_high(ChineseVariant::Traditional));
assert_eq!("十二萬三千四百五十六京七千八百九十萬一千二百三十四兆五千六百七十八萬九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_middle(ChineseVariant::Traditional));
assert_eq!("十二穰三千四百五十六秭七千八百九十垓一千二百三十四京五千六百七十八兆九千零一十二億三千四百五十六萬七千八百九十", 123456789012345678901234567890i128.to_lowercase_ten_thousand(ChineseVariant::Traditional));
assert_eq!("一极二载三正四涧五沟六穰七秭八垓九京零一亿二万三千四百五十六", 1234567890123456i64.to_lowercase_low(ChineseVariant::Simple));

assert_eq!("一角二分", 0.12f64.to_lowercase_ten_thousand(ChineseVariant::Traditional));
```

## Todo

1. Parsing Chinese numbers to primitive numbers.

## Crates.io

https://crates.io/crates/chinese-number

## Documentation

https://docs.rs/chinese-number

## License

[MIT](LICENSE)