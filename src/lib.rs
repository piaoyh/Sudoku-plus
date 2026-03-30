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
//! 3D multi-layer sudoku algorithm, and 3D cubic sudoku algorithm.
//! They generates sudoku problems and solves sudoku problems. They are designed
//! to be flexible and efficient, allowing users to easily create and solve
//! sudoku puzzles of varying sizes and complexities. The library is built with
//! a focus on performance and usability, making it suitable for both casual
//! puzzle enthusiasts and developers looking to integrate sudoku functionality
//! into their applications. `sudoku-plus` is a versatile Rust library designed
//! for generating and solving various Sudoku structures,
//! including __Plane__, __Multi-layer__, and __Cubic__ Sudoku.
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
//! - [X] __PlaneSudoku:__ A generic 2D Sudoku with a (N^2 X N^2) grid.
//!   You can define the size by choosing the constant `N`. --
//!   [PlaneSudoku](plane_sudoku/struct.PlaneSudoku.html#struct.PlaneSudoku)
//! 
//! # 2. 3D Sudoku
//! 
//! - [ ] __Multi-layer Sudoku:__ A 3D Sudoku structure with dimensions of 
//!   (N^2 X N^2 X N^2). The size is determined by the constant `N`.  --
//!   [MultiLayerSudoku](multi_layer_sudoku/struct.MultiLayerSudoku.html#struct.MultiLayerSudoku)
//! - [ ] __Cubic Sudoku:__ A 3D Sudoku structure with dimensions of (N^3 X N^3 X N^3). The size is determined by the constant `N`. --
//!   [CubicSudoku](cubic_sudoku/struct.CubicSudoku.html#struct.CubicSudoku)
//! 
//! # 3. Sudoku Elements
//! 
//! - [ ] __Sudoku Element:__ A generic Sudoku component designed for building
//!   complex applications, such as academic timetable generators powered by
//!   Sudoku algorithms. For the future use, this trait `SudokuElement` is
//!   defined for sudoku elements. It is supposed to be implemented by any data
//!   type that supports cryptocol::number::SmallUInt. It can be removed in the
//!   future if it is found not necessary. --
//!   [SudokuElement](sudoku_element/struct.SudokuElement.html#struct.SudokuElement).
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

/// The `plane_sudoku` module provides the struct `PlaneSudoku`
/// for 2D plane sudoku.
pub mod plane_sudoku;

/// The `multi_layer_Sudoku` module provides the struct `MultiLayerSudoku`
/// for 3D multi-layer sudoku.
pub mod multi_layer_sudoku;

/// The `cubic_sudoku` module provides the struct `CubicSudoku`
/// for 3D cubic sudoku.
pub mod cubic_sudoku;

/// The `sudoku_element` module provides the trait `SudokuElement`
/// for sudoku elements.
pub mod sudoku_element;

pub use plane_sudoku::PlaneSudoku;
pub use multi_layer_sudoku::MultiLayerSudoku;
pub use cubic_sudoku::CubicSudoku;
pub use sudoku_element::SudokuElement;

pub use plane_sudoku::PlaneSudoku_4x4;
pub use plane_sudoku::PlaneSudoku_9x9;
pub use plane_sudoku::PlaneSudoku_16x16;
pub use plane_sudoku::PlaneSudoku_25x25;
pub use plane_sudoku::PlaneSudoku_36x36;
pub use plane_sudoku::PlaneSudoku_49x49;
pub use plane_sudoku::PlaneSudoku_64x64;
pub use plane_sudoku::PlaneSudoku_81x81;
pub use plane_sudoku::PlaneSudoku_100x100;
pub use plane_sudoku::PlaneSudoku_121x121;
pub use plane_sudoku::PlaneSudoku_144x144;
pub use plane_sudoku::PlaneSudoku_169x169;
pub use plane_sudoku::PlaneSudoku_196x196;
pub use plane_sudoku::PlaneSudoku_225x225;
pub use plane_sudoku::PlaneSudoku_289x289;
pub use plane_sudoku::PlaneSudoku_324x324;
pub use plane_sudoku::PlaneSudoku_361x361;
pub use plane_sudoku::PlaneSudoku_400x400;

pub use multi_layer_sudoku::MultiLayerSudoku_4x4x4;
pub use multi_layer_sudoku::MultiLayerSudoku_9x9x9;
pub use multi_layer_sudoku::MultiLayerSudoku_16x16x16;
pub use multi_layer_sudoku::MultiLayerSudoku_25x25x25;
pub use multi_layer_sudoku::MultiLayerSudoku_36x36x36;
pub use multi_layer_sudoku::MultiLayerSudoku_49x49x49;
pub use multi_layer_sudoku::MultiLayerSudoku_64x64x64;
pub use multi_layer_sudoku::MultiLayerSudoku_81x81x81;
pub use multi_layer_sudoku::MultiLayerSudoku_100x100x100;
pub use multi_layer_sudoku::MultiLayerSudoku_121x121x121;
pub use multi_layer_sudoku::MultiLayerSudoku_144x144x144;
pub use multi_layer_sudoku::MultiLayerSudoku_169x169x169;
pub use multi_layer_sudoku::MultiLayerSudoku_196x196x196;
pub use multi_layer_sudoku::MultiLayerSudoku_225x225x225;
pub use multi_layer_sudoku::MultiLayerSudoku_289x289x289;
pub use multi_layer_sudoku::MultiLayerSudoku_324x324x324;
pub use multi_layer_sudoku::MultiLayerSudoku_361x361x361;
pub use multi_layer_sudoku::MultiLayerSudoku_400x400x400;

pub use cubic_sudoku::CubicSudoku_8x8x8;
pub use cubic_sudoku::CubicSudoku_27x27x27;
pub use cubic_sudoku::CubicSudoku_64x64x64;
pub use cubic_sudoku::CubicSudoku_125x125x125;
pub use cubic_sudoku::CubicSudoku_216x216x216;
pub use cubic_sudoku::CubicSudoku_343x343x343;
pub use cubic_sudoku::CubicSudoku_512x512x512;
pub use cubic_sudoku::CubicSudoku_729x729x729;
pub use cubic_sudoku::CubicSudoku_1000x1000x1000;
pub use cubic_sudoku::CubicSudoku_1331x1331x1331;
pub use cubic_sudoku::CubicSudoku_1728x1728x1728;
pub use cubic_sudoku::CubicSudoku_2197x2197x2197;
pub use cubic_sudoku::CubicSudoku_2744x2744x2744;
pub use cubic_sudoku::CubicSudoku_3375x3375x3375;
pub use cubic_sudoku::CubicSudoku_4096x4096x4096;
pub use cubic_sudoku::CubicSudoku_4913x4913x4913;
pub use cubic_sudoku::CubicSudoku_5832x5832x5832;
pub use cubic_sudoku::CubicSudoku_6859x6859x6859;
pub use cubic_sudoku::CubicSudoku_8000x8000x8000;
