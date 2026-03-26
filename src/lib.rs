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
//! It provides 2D plane sudoku algorithm, 3-dimensional multiplane
//! sudoku algorithm, and 3D cubic sudoku algorithm. They generates
//! sudoku problems and solves sudoku problems.

/// The `plane_sudoku` module provides the struct `PlaneSudoku`
/// for 2D plane sudoku.
pub mod plane_sudoku;

/// The `multiplane_sudoku` module provides the struct `MultiplaneSudoku`
/// for 3D multi-plane sudoku.
pub mod multiplane_sudoku;

/// The `cubic_sudoku` module provides the struct `CubicSudoku`
/// for 3D cubic sudoku.
pub mod cubic_sudoku;

/// The `sudoku_element` module provides the trait `SudokuElement`
/// for sudoku elements.
pub mod sudoku_element;

pub use plane_sudoku::PlaneSudoku;
pub use multiplane_sudoku::MultiplaneSudoku;
pub use cubic_sudoku::CubicSudoku;
pub use sudoku_element::SudokuElement;
pub use sudoku_element::TraitsSudokuElement;
