// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


//! # __Sudoku-plus:__ An Expandable Sudoku Library
//! 
//! `Sudoku-plus` is a versatile Rust library designed for providing
//! various kinds of sudoku algorithms. It provides 2D plane sudoku algorithm,
//! 3D multiplane sudoku algorithm, and 3D cubic sudoku algorithm.
//! They generates sudoku problems and solves sudoku problems. They are designed
//! to be flexible and efficient, allowing users to easily create and solve
//! sudoku puzzles of varying sizes and complexities. The library is built with
//! a focus on performance and usability, making it suitable for both casual
//! puzzle enthusiasts and developers looking to integrate sudoku functionality
//! into their applications. `sudoku-plus` is a versatile Rust library designed
//! for generating and solving various Sudoku structures,
//! including __Plane__, __Multi-plane__, and __Cubic__ Sudoku.
//! 
//! # Roadmap for Version 1.0
//! 
//! The following features are planned for the sudoku-plus ecosystem.
//! - [X] __Completed:__ Implementation and documentation are
//!   at least __95%__ complete.
//! - [ ] __In Progress:__ Implementation or documentation is below __95%__,
//!   or work has not yet begun.
//! 
//! # 1. 2D Sudoku
//! 
//! - [ ] __PlaneSudoku:__ A generic 2D Sudoku with a (N^2 X N^2) grid.
//!   You can define the size by choosing the constant `N`. --
//!   [PlaneSudoku](https://docs.rs/sudoku-plus/latest/sudoku_plus/plane_sudoku/struct.PlaneSudoku.html.PlaneSudoku)
//! 
//! # 2. 3D Sudoku
//! 
//! - [ ] __Multi-plane Sudoku:__ A 3D Sudoku structure with dimensions of 
//!   (N^2 X N^2 X N^2). The size is determined by the constant `N`.  --
//!   [MultiplaneSudoku](https://docs.rs/sudoku-plus/latest/sudoku_plus/multiplane_sudoku/struct.MultiplaneSudoku.html.MultiplaneSudoku)
//! - [ ] __Cubic Sudoku:__ A 3D Sudoku structure with dimensions of (N^3 X N^3 X N^3). The size is determined by the constant `N`. --
//!   [CubicSudoku](https://docs.rs/sudoku-plus/latest/sudoku_plus/cubic_sudoku/struct.CubicSudoku.html.CubicSudoku)
//! 
//! # 3. Sudoku Elements
//! 
//! - [ ] __Sudoku Element:__ A generic Sudoku component designed for building
//!   complex applications, such as academic timetable generators powered by
//!   Sudoku algorithms. For the future use, this trait `SudokuElement` is
//!   defined for sudoku elements. It is supposed to be implemented by any data
//!   type that supports cryptocol::number::SmallUInt. It can be removed in the
//!   future if it is found not necessary. --
//!   [SudokuElement](https://docs.rs/sudoku-plus/latest/sudoku_plus/sudoku_element/struct.SudokuElement.html.SudokuElement).
//! 
//! # Versioning Policy
//! 
//! The project will reach Version 1.0.0 once all functional areas listed above
//! are fully implemented.
//! 
//! - __Pre-v1.0:__ Versions will range up to 0.3.x based on the progress of the
//!   listed functionalities.
//! - __Post-v1.0:__ New features and stable releases will follow standard
//!   semantic versioning beyond 1.0.0.
//! 
//! _Note: Version numbers like 0.2.0 indicate progress through the
//! functionality list, not necessarily a 20% completion of the entire codebase.
//! 
//! # Breaking Changes
//! 
//! For a detailed history of API changes and migration guides,
//! please refer to `BreakingChanges.md` for a detailed history of API changes.

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
