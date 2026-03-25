// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::cmp::PartialEq;
use cryptocol::number::SmallUInt;

pub trait SudokuElement<Rhs = Self> : PartialEq<Rhs> + Copy + Clone
where Rhs: ?Sized
{
    fn new() -> Self;
    fn get_id(&self) -> u128;
    fn one() -> Self;
}


macro_rules! SudokuElement_for_int_impl
{
    ($($f:ty), *) => {
        $(
            impl SudokuElement for $f
            {
                fn new() -> Self
                {
                    0 as $f
                }

                fn get_id(&self) -> u128
                {
                   *self as u128
                }

                fn one() -> Self
                {
                    1 as $f
                }
            }
        )*
    };
}


SudokuElement_for_int_impl! { u8, u16, u32, u64, u128, usize }
// SudokuElement_for_int_impl! { i8, i16, i32, i64, i128, isize }
