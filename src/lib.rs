// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


//! # Sudoku-plus
//!
//! `Sudoku-plus` is a library for providing various kinds of sudoku algorithms.
//! It provides 2-dimensional plain sudoku algorithm, 3-dimensional multiplain
//! sudoku algorithm, and 3-dimentional cubic sudoku algorithm. They generates
//! sudoku problems and solves sudoku problems.
//! 
/// The `plain_sudoku` module provides the struct `PlainSudoku`
/// for 2-dimensional plain sudoku.
pub mod plain_sudoku;

/// The `multiplain_sudoku` module provides the struct `MultiplainSudoku`
/// for 3-dimensional multiplain sudoku.
pub mod multiplain_sudoku;

/// The `cubic_sudoku` module provides the struct `CubicSudoku`
/// for 3-dimensional cubic sudoku.
pub mod cubic_sudoku;

/// The `sudoku_element` module provides the trait `SudokuElement`
/// for sudoku elements.
pub mod sudoku_element;

pub use plain_sudoku::PlainSudoku;
pub use multiplain_sudoku::MultiplainSudoku;
pub use cubic_sudoku::CubicSudoku;
pub use sudoku_element::SudokuElement;
