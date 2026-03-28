// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use cryptocol::number::SmallUInt;
use cryptocol::random::{ Random, RandGen };


/// A type alias for a 4x4 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 2`, resulting in a total grid size of 4x4 
/// (2^2 x 2^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_4X4 = PlaneSudoku<u8, 2>;

/// A type alias for a 9x9 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 3`, resulting in a total grid size of 9x9 
/// (3^2 x 3^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_9X9 = PlaneSudoku<u8, 3>;

/// A type alias for a 16x16 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 4`, resulting in a total grid size of 16x16 
/// (4^2 x 4^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_16X16 = PlaneSudoku<u8, 4>;

/// A type alias for a 25x25 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 5`, resulting in a total grid size of 25x25 
/// (5^2 x 5^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_25X25 = PlaneSudoku<u8, 5>;

/// A type alias for a 36x36 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 6`, resulting in a total grid size of 36x36 
/// (6^2 x 6^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_36X36 = PlaneSudoku<u8, 6>;

/// A type alias for a 49x49 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 7`, resulting in a total grid size of 49x49 
/// (7^2 x 7^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_49X49 = PlaneSudoku<u8, 7>;

/// A type alias for a 64x64 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 8`, resulting in a total grid size of 64x64 
/// (8^2 x 8^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_64X64 = PlaneSudoku<u8, 8>;

/// A type alias for an 81x81 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 9`, resulting in a total grid size of 81x81 
/// (9^2 x 9^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_81X81 = PlaneSudoku<u8, 9>;

/// A type alias for a 100x100 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 10`, resulting in a total grid size of 100x100 
/// (10^2 x 10^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_100X100 = PlaneSudoku<u8, 10>;

/// A type alias for a 121x121 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 11`, resulting in a total grid size of 121x121 
/// (11^2 x 11^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_121X121 = PlaneSudoku<u8, 11>;

/// A type alias for a 144x144 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 12`, resulting in a total grid size of 144x144 
/// (12^2 x 12^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_144X144 = PlaneSudoku<u8, 12>;

/// A type alias for a 169x169 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 13`, resulting in a total grid size of 169x169 
/// (13^2 x 13^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_169X169 = PlaneSudoku<u8, 13>;

/// A type alias for a 196x196 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 14`, resulting in a total grid size of 196x196 
/// (14^2 x 14^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_196X196 = PlaneSudoku<u8, 14>;

/// A type alias for a 225x225 `PlaneSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 15`, resulting in a total grid size of 225x225 
/// (15^2 x 15^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_225X225 = PlaneSudoku<u8, 15>;

/// A type alias for a 256x256 `PlaneSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 16`, resulting in a total grid size of 256x256 
/// (16^2 x 16^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_256X256 = PlaneSudoku<u16, 16>;

/// A type alias for a 289x289 `PlaneSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 17`, resulting in a total grid size of 289x289 
/// (17^2 x 17^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_289X289 = PlaneSudoku<u16, 17>;

/// A type alias for a 324x324 `PlaneSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 18`, resulting in a total grid size of 324x324 
/// (18^2 x 18^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_324X324 = PlaneSudoku<u16, 18>;

/// A type alias for a 361x361 `PlaneSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 19`, resulting in a total grid size of 361x361 
/// (19^2 x 19^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_361X361 = PlaneSudoku<u16, 19>;

/// A type alias for a 400x400 `PlaneSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 20`, resulting in a total grid size of 400x400 
/// (20^2 x 20^2).
#[allow(non_camel_case_types)]
pub type PlaneSudoku_400X400 = PlaneSudoku<u16, 20>;


/// # Introduction
/// Sudoku is the one of the famous puzzle games. This struct `PlaneSudoku` is 
/// a generic 2D Sudoku algorithm that generates and solves problems. Its size
/// is a (N^2 X N^2) grid. You can define the size by choosing the generic
/// constant `N`.
/// 
/// # Generic Constants
/// - `T`: is supposed to be any data type that supports
///   cryptocol::number::SmallUInt + Copy + Clone + Eq. At the momnet,
///   `T` can be one of `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`.
///   In the future, this crate may provide some useful datatypes for `T`
///   that can substitute for cryptocol::number::SmallUInt.
///   The default type is `u8`.
/// - `N`: determines the size of sudoku board. `N` should be greater than `1`.
///   `N` indicates the size of a sub-square, which is `N` x `N`.
///   The sudoku board is composed of `N` rows of `N` columns of sub-squares.
///   The size `N^2` should be within the representable range of the underlying
///   type `T` (`N^2 <= T::MAX`)
///   So, the size of the sudoku board is N^2 x N x N (= N^2 x N^2).
///   For example, if `N` = 3, the size of the sudoku board is 9 x 9.
///   The default value of `N` = `3`.
/// 
/// # Quick Start
/// To create a new `PlaneSudoku` instance, you can use the `new` method:
/// ```
/// use sudoku_plus::PlaneSudoku_9X9;
/// let sudoku = PlaneSudoku_9X9::new();
/// assert!(sudoku.is_some());
/// ```
/// 
/// You can also initialize a `PlaneSudoku` with a specific problem pattern
/// using the `new_with` method:
/// ```
/// use sudoku_plus::PlaneSudoku_9X9;
/// let problem: [[[[u8; 3]; 3]; 3]; 3] = [
///   [[[4, 0, 6], [3, 2, 7], [9, 0, 8]], 
///    [[9, 2, 3], [6, 1, 8], [4, 7, 5]], 
///    [[1, 7, 8], [9, 4, 5], [3, 6, 2]]], 
///  [[[2, 3, 1], [8, 0, 4], [5, 0, 7]], 
///    [[6, 8, 4], [5, 7, 9], [2, 3, 1]], 
///    [[7, 9, 5], [2, 3, 1], [6, 8, 4]]], 
///  [[[8, 1, 0], [4, 0, 6], [7, 0, 3]], 
///   [[5, 6, 2], [7, 8, 3], [1, 4, 9]], 
///   [[3, 0, 7], [1, 0, 2], [8, 0, 6]]]];
/// let sudoku = PlaneSudoku_9X9::new_with(problem);
/// assert!(sudoku.is_some());
/// ```
/// 
/// You can also initialize a `PlaneSudoku` with a 2D problem pattern
/// using the `new_with_2d` method:
/// ```
/// use sudoku_plus::PlaneSudoku_9X9;
/// let problem: [[u8; 9]; 9] = [
///   [4, 0, 6, 3, 2, 7, 9, 0, 8],
///   [9, 2, 3, 6, 1, 8, 4, 7, 5],
///   [1, 7, 8, 9, 4, 5, 3, 6, 2],
///   [2, 3, 1, 8, 0, 4, 5, 0, 7],
///   [6, 8, 4, 5, 7, 9, 2, 3, 1],
///   [7, 9, 5, 2, 3, 1, 6, 8, 4],
///   [8, 1, 0, 4, 0, 6, 7, 0, 3],
///   [5, 6, 2, 7, 8, 3, 1, 4, 9],
///   [3, 0, 7, 1, 0, 2, 8, 0, 6]];
/// let sudoku = PlaneSudoku_9X9::new_with_2d(problem);
/// assert!(sudoku.is_some());
/// ```
/// 
/// You can access the values in the 2D view of the Sudoku board
/// using the `get_from_2d_view` method:
/// ```
/// use sudoku_plus::PlaneSudoku_9X9;
/// let mut sudoku = PlaneSudoku_9X9::new().unwrap();
/// sudoku.generate(40);
/// for row in 0..9
/// {
///     for col in 0..9
///     {
///        let point = sudoku.get_from_2d_view(row, col);
///        if point == 0
///            { print!("_ "); }
///        else
///            { print!("{} ", point); }
///     }
///     println!();
/// }
/// ```
/// 
/// You can generate a Sudoku puzzle with a specific number of holes
/// (empty cells) using the `generate` method:
/// ```
/// use sudoku_plus::PlaneSudoku_9X9;
/// let mut sudoku = PlaneSudoku_9X9::new().unwrap();
/// sudoku.generate(40);
/// ```
/// This will create a Sudoku puzzle with 40 holes, which is generally
/// considered to be of medium difficulty. You can adjust the number of
/// holes to create puzzles of varying difficulty levels.
pub struct PlaneSudoku<T: SmallUInt = u8, const N: usize = 3>
{
    sudoku: [[[[T; N]; N]; N]; N],
    candidate: [[[[T; N]; N]; N]; N],
    random: RandGen,
}

impl<T: SmallUInt, const N: usize> PlaneSudoku<T, N>
{
    // pub fn new() -> Option<Self>
    /// Creates a new `PlaneSudoku` instance wrapped in `Some`.
    /// 
    /// # Returns
    /// - `Some(Self)` if:
    ///   - `N > 1` (minimum valid Sudoku sub-grid size); and
    ///   - The size `N^2` is within the representable range of the underlying
    ///     type `T` (`N^2 <= T::MAX`).
    /// - `None` if any of the above conditions are not met.
    /// 
    /// # Example 1 for Basic 9x9 Sudoku initialization (N=3)
    /// ```
    /// use sudoku_plus::PlaneSudoku_9X9;
    /// let sudoku = PlaneSudoku_9X9::new();
    /// assert!(sudoku.is_some());
    /// ```
    /// 
    /// # Example 2 for Using a larger grid size with u16 to avoid overflow
    /// ```
    /// use sudoku_plus::PlaneSudoku;
    /// let sudoku = PlaneSudoku::<u16, 16>::new();
    /// assert!(sudoku.is_some());
    /// ```
    /// 
    /// # Example 3 for Size constraints leading to None
    /// ```
    /// use sudoku_plus::PlaneSudoku;
    /// let sudoku = PlaneSudoku::<u8, 16>::new();
    /// assert!(sudoku.is_none());
    /// ```
    /// 
    /// # Example 4 for Handling potential None for invalid configurations
    /// ```
    /// use sudoku_plus::PlaneSudoku;
    /// if let Some(_sudoku) = PlaneSudoku::<u8, 3>::new()
    ///     { println!("Successfully created a 9 x 9 Sudoku!"); }
    /// ```
    pub fn new() -> Option<Self>
    {
        if !Self::assert()
        {
            None
        }
        else
        {
            let mut me = Self
            {
                sudoku: [[[[T::MIN; N]; N]; N]; N],
                candidate: [[[[T::MIN; N]; N]; N]; N],
                random: Random::new(),
            };
            me.create_candidates();
            Some(me)
        }
    }

    // pub fn new_with(problem: [[[[T; N]; N]; N]; N]) -> Option<Self>
    /// Creates a new `PlaneSudoku` instance wrapped in `Some`
    /// from a 4-dimensional problem array.
    /// 
    /// # Arguments
    /// * `problem` - A 4D array of type `T` with dimensions `N x N x N x N`, 
    ///   representing the initial state of the Sudoku puzzle.
    /// 
    /// # Returns
    /// - `Some(Self)` if:
    ///   - `N > 1` (minimum valid Sudoku sub-grid size); and
    ///   - The size `N^2` is within the representable range of the underlying
    ///     type `T` (`N^2 <= T::MAX`).
    /// - `None` if any of the above conditions are not met.
    /// 
    /// # Example 1 for Initializing with a specific puzzle pattern
    /// ```
    /// use sudoku_plus::PlaneSudoku_9X9;
    /// let problem: [[[[u8; 3]; 3]; 3]; 3] = [
    ///     [[[4, 0, 6], [3, 2, 7], [9, 0, 8]], 
    ///      [[9, 2, 3], [6, 1, 8], [4, 7, 5]], 
    ///      [[1, 7, 8], [9, 4, 5], [3, 6, 2]]], 
    ///     [[[2, 3, 1], [8, 0, 4], [5, 0, 7]], 
    ///      [[6, 8, 4], [5, 7, 9], [2, 3, 1]], 
    ///      [[7, 9, 5], [2, 3, 1], [6, 8, 4]]], 
    ///     [[[8, 1, 0], [4, 0, 6], [7, 0, 3]], 
    ///      [[5, 6, 2], [7, 8, 3], [1, 4, 9]], 
    ///      [[3, 0, 7], [1, 0, 2], [8, 0, 6]]]];
    /// let sudoku = PlaneSudoku_9X9::new_with(problem);
    /// assert!(sudoku.is_some());
    /// ```
    /// 
    /// # Example 2 for Error handling for invalid input data
    /// ```
    /// use sudoku_plus::PlaneSudoku_4X4;
    /// let problem: [[[[u8; 2]; 2]; 2]; 2] = [
    ///     [[[1, 0], [0, 2]],
    ///      [[0, 0], [0, 0]]],
    ///     [[[0, 0], [0, 0]],
    ///      [[0, 0], [3, 4]]]];
    /// let sudoku = PlaneSudoku_4X4::new_with(problem);
    /// match sudoku
    /// {
    ///     Some(_) => println!("Valid puzzle loaded"),
    ///     None => println!("Invalid puzzle dimensions"),
    /// }
    /// assert!(sudoku.is_some());
    /// ```
    pub fn new_with(problem: [[[[T; N]; N]; N]; N]) -> Option<Self>
    {
        if !Self::assert()
        {
            None
        }
        else
        {
            let mut me = Self
            {
                sudoku: problem,
                candidate: [[[[T::MIN; N]; N]; N]; N],
                random: Random::new(),
            };
            me.create_candidates();
            Some(me)
        }
    }

    // pub fn new_with_2d<const M: usize>(problem: [[T; M]; M]) -> Option<Self>
    /// Creates a new `PlaneSudoku` instance from a 2-dimensional problem array.
    /// 
    /// This method maps a flat 2D grid of size `M x M`
    /// into the internal 4D structure.
    /// 
    /// # Arguments
    /// * `problem` - A 2D array of type `T` with dimensions `M x M`.
    ///   Here, `M` must satisfy the condition `M == N * N`.
    /// 
    /// # Returns
    /// - `Some(Self)` if:
    ///   - `N > 1` (minimum valid Sudoku sub-grid size);
    ///   - `M` (where `M = N * N`) is within the representable range of 
    ///     type `T`; and
    ///   - The input array dimensions `M x M` are consistent
    ///     with the internal `N x N x N x N` structure.
    /// - `None` if any of the above conditions are not met.
    /// 
    /// # Example 1 for Initializing with a specific puzzle pattern
    /// ```
    /// use sudoku_plus::PlaneSudoku_9X9;
    /// let problem: [[u8; 9]; 9] = [
    ///     [4, 0, 6, 3, 2, 7, 9, 0, 8], 
    ///     [9, 2, 3, 6, 1, 8, 4, 7, 5], 
    ///     [1, 7, 8, 9, 4, 5, 3, 6, 2], 
    ///     [2, 3, 1, 8, 0, 4, 5, 0, 7], 
    ///     [6, 8, 4, 5, 7, 9, 2, 3, 1], 
    ///     [7, 9, 5, 2, 3, 1, 6, 8, 4], 
    ///     [8, 1, 0, 4, 0, 6, 7, 0, 3], 
    ///     [5, 6, 2, 7, 8, 3, 1, 4, 9], 
    ///     [3, 0, 7, 1, 0, 2, 8, 0, 6]];
    /// let sudoku = PlaneSudoku_9X9::new_with(problem);
    /// assert!(sudoku.is_some());
    /// ```
    /// 
    /// # Example 2 for Error handling for invalid input data
    /// ```
    /// use sudoku_plus::PlaneSudoku_4X4;
    /// let problem: [[u8; 4]; 4] = [
    ///     [1, 0, 0, 2],
    ///     [0, 0, 0, 0],
    ///     [0, 0, 0, 0],
    ///     [0, 0, 3, 4]];
    /// let sudoku = PlaneSudoku_4X4::new_with(problem);
    /// match sudoku
    /// {
    ///     Some(_) => println!("Valid puzzle loaded"),
    ///     None => println!("Invalid puzzle dimensions"),
    /// }
    /// assert!(sudoku.is_some());
    /// ```
    pub fn new_with_2d<const M: usize>(problem: [[T; M]; M]) -> Option<Self>
    {
        if N * N != M || !Self::assert()
            { return None; }

        let mut me = Self
        {
            sudoku: [[[[T::MIN; N]; N]; N]; N],
            candidate: [[[[T::MIN; N]; N]; N]; N],
            random: Random::new(),
        };
        
        for row in 0..N
        {
            for col in 0..N
            {
                for ro in 0..N
                {
                    for co in 0..N
                    {
                        let (r, c) = Self::index_into_2d_view(row, col, ro, co);
                        me.sudoku[row][col][ro][co] = problem[r][c];
                    }
                }
            }
        }
        me.create_candidates();
        Some(me)
    }

    // pub fn get_from_2d_view(&self, row: usize, col: usize) -> T
    /// Gets the value at the specified location
    /// in the 2D view of the Sudoku board.
    /// 
    /// # Arguments
    /// - `row`: The row index in the 2D view (0-based).
    /// - `col`: The column index in the 2D view (0-based).
    /// 
    /// # Returns
    /// - The value of type `T` at the specified location in the 2D view
    ///   on the Sudoku board.
    /// 
    /// # Example for Accessing values in the 2D view
    /// ```
    /// use sudoku_plus::PlaneSudoku_9X9;
    /// let mut sudoku = PlaneSudoku_9X9::new().unwrap();
    /// sudoku.generate(40);
    /// for row in 0..9
    /// {
    ///     for col in 0..9
    ///     {
    ///         let point = sudoku.get_from_2d_view(row, col);
    ///         if point == 0
    ///             { print!("_ "); }
    ///         else
    ///             { print!("{} ", point); }
    ///     }
    ///     println!();
    /// }
    /// ```
    #[inline]
    pub fn get_from_2d_view(&self, row: usize, col: usize) -> T
    {
        self.sudoku[row / N][col / N][row % N][col % N]
    }

    // pub fn generate(&mut self, n_holes: usize)
    /// Generates a Sudoku puzzle
    /// with the specified number of empty cells (`n_holes`).
    /// 
    /// # Arguments
    /// * `n_holes` - The number of blanks (holes) to be created in the
    ///   generated Sudoku board.
    /// 
    /// # Features
    /// * **Complete Solution**: If `n_holes` is `0`, the method returns a 
    ///   fully completed Sudoku board (a valid solution).
    /// * **Randomness**: Each call generates a unique puzzle based on the
    ///   internal RNG.
    /// * **Maximum Capacity**: If `n_holes` exceeds the total number of cells 
    ///   in the grid, it is treated as the maximum capacity. In this case, 
    ///   the method returns an entirely empty Sudoku board.
    /// 
    /// # Example 1 for Generating a puzzle with a specific difficulty (40 holes)
    /// ```
    /// use sudoku_plus::PlaneSudoku_9X9;
    /// let mut sudoku = PlaneSudoku_9X9::new().unwrap();
    /// sudoku.generate(40);
    /// for row in 0..9
    /// {
    ///     for col in 0..9
    ///     {
    ///         let point = sudoku.get_from_2d_view(row, col);
    ///         if point == 0
    ///             { print!("_ "); }
    ///         else
    ///             { print!("{} ", point); }
    ///     }
    ///     println!();
    /// }
    /// ```
    ///
    /// # Example 2 for Generating a fully solved board (0 holes)
    /// ```
    /// use sudoku_plus::PlaneSudoku_16X16;
    /// let mut sudoku = PlaneSudoku_16X16::new().unwrap();
    /// sudoku.generate(0);
    /// for row in 0..16
    /// {
    ///     for col in 0..16
    ///     {
    ///         let point = sudoku.get_from_2d_view(row, col);
    ///         if point == 0
    ///             { print!("__ "); }
    ///         else
    ///             { print!("{:02} ", point); }
    ///     }
    ///     println!();
    /// }
    /// ```
    ///
    /// # Example 3 for Handling oversized n_holes (results in an empty board)
    /// ```
    /// use sudoku_plus::PlaneSudoku_4X4;
    /// let mut sudoku = PlaneSudoku_4X4::new().unwrap();
    /// sudoku.generate(20); // 4x4 grid has 16 cells, so this will result in an empty board
    /// for row in 0..4
    /// {
    ///     for col in 0..4
    ///     {
    ///         let point = sudoku.get_from_2d_view(row, col);
    ///         if point == 0
    ///             { print!("_ "); }
    ///         else
    ///             { print!("{} ", point); }
    ///     }
    ///     println!();
    /// }
    /// ```
    pub fn generate(&mut self, n_holes: usize)
    {
        self.sudoku = [[[[T::MIN; N]; N]; N]; N];
        self.solve();
        let mut mask = [[[[T::ONE; N]; N]; N]; N];
        let (mut row, mut col, mut ro, mut co) = (0_usize, 0_usize, 0_usize, 0_usize);
        for _ in 0..(if n_holes <= N * N * N * N {n_holes} else {N * N * N * N})
        {
            mask[row][col][ro][co] = T::MIN;
            (row, col, ro, co) = Self::advance_big(row, col, ro, co);
        }
        self.shuffle_big(&mut mask);

        for row in 0..N
        {
            for col in 0..N
            {
                for ro in 0..N
                {
                    for co in 0..N
                    {
                        if mask[row][col][ro][co] == T::MIN
                            { self.sudoku[row][col][ro][co] = T::MIN; }
                    }
                }
            }
        }
    }

    // pub fn solve(&mut self) -> bool
    /// Solves the Sudoku puzzle using a backtracking algorithm.
    ///
    /// This method attempts to find a valid solution for the current board
    /// state. If a solution exists, the internal state of `PlaneSudoku` is
    /// updated with the completed grid.
    /// 
    /// # Returns
    /// - `true` if there is a solution and this method solves
    ///   the given problem successfully.
    /// - `false` if there is no solution.
    /// 
    /// # Example 1 for Solving a standard 9x9 Sudoku puzzle
    /// ```
    /// use sudoku_plus::PlaneSudoku_9X9;
    /// let mut sudoku = PlaneSudoku_9X9::new().unwrap();
    /// sudoku.generate(40); // Generate a puzzle with 40 holes
    /// let success = sudoku.solve();
    /// if success
    ///     { println!("Sudoku solved successfully!"); }
    /// else
    ///     { println!("No solution exists for this puzzle."); }
    /// assert!(success);
    /// ```
    /// 
    /// # Example 2 for Verifying a solution for a small 4x4 grid (N=2)
    /// ```
    /// use sudoku_plus::PlaneSudoku_4X4;
    /// let mut sudoku = PlaneSudoku_4X4::new().unwrap();
    /// sudoku.generate(7); // Generate a puzzle with 7 holes, which should be solvable
    /// assert!(sudoku.solve(), "4x4 puzzles generated this way should be solvable");
    /// ```
    /// 
    /// # Example 3 for Handling an unsolvable custom puzzle
    /// ```
    /// use sudoku_plus::PlaneSudoku_4X4;
    /// let mut problem: [[u8; 4]; 4] = [
    ///     // Set conflicting values in the same row/column/block
    ///     [1, 0, 0, 3],
    ///     [0, 0, 2, 0],
    ///     [0, 2, 0, 0],
    ///     [0, 0, 3, 4]];
    /// let mut sudoku = PlaneSudoku_4X4::new_with_2d(problem).unwrap();
    /// let success = sudoku.solve();   // Should return false for invalid puzzle states
    /// assert!(!success, "This puzzle should be unsolvable due to conflicting values");
    /// ```
    pub fn solve(&mut self) -> bool
    {
        self.solve_step_by_step( 0, 0, 0, 0)
    }

    fn solve_step_by_step(&mut self, mut row: usize, mut col: usize, mut ro: usize, mut co: usize) -> bool
    {
        while self.sudoku[row][col][ro][co] != T::MIN
        {
            (row, col, ro, co) = Self::advance_big(row, col, ro, co);
            if row == N
                { return true; }
        }

        for r in 0..N
        {
            for c in 0..N
            {
                let point = self.candidate[row][col][r][c];
                if self.check(row, col, ro, co, point)
                {
                    self.sudoku[row][col][ro][co] = point;
                    (row, col, ro, co) = Self::advance_big(row, col, ro, co);
                    if row == N || self.solve_step_by_step(row, col, ro, co)
                        { return true; }
                    (row, col, ro, co) = Self::retreat_big(row, col, ro, co);
                    self.sudoku[row][col][ro][co] = T::MIN;
                }
            }
        }
        false
    }

    // pub fn check(&self, row: usize, col: usize, ro: usize, co: usize, point: T) -> bool
    /// Checks whether `point` can be fit to the location (row, col, ro, co).
    /// 
    /// # Arguments
    /// - `row`: is vertical location of the sub-square in the sudoku board (0-based).
    /// - `col`: is horizontal location of the sub-square in the sudoku board (0-based).
    /// - `ro`: is vertical location of the blank in the sub-square (0-based).
    /// - `co`: is horizontal location of the blank in the sub-square (0-based).
    /// 
    /// # Returns
    /// - `true` if `point` can be fit to the location (row, col, ro, co).
    /// - `false` if `point` cannot be fit to the location (row, col, ro, co).
    fn check(&self, row: usize, col: usize, ro: usize, co: usize, point: T) -> bool
    {
        for r in 0..N
        {
            for c in 0..N
            {
                if ro == r && co == c
                    { continue; }
                if point == self.sudoku[row][col][r][c]
                    { return false; } 
            }
        }

        for cc in 0..N
        {
            if cc == col
                { continue; }
            for c in 0..N
            {
                if point == self.sudoku[row][cc][ro][c]
                    { return false; } 
            }
        }

        for rr in 0..N
        {
            if rr == row
                { continue; }
            for r in 0..N
            {
                if point == self.sudoku[rr][col][r][co]
                    { return false; } 
            }
        }
        true
    }

    fn create_candidates(&mut self)
    {
        for row in 0..N
        {
            for col in 0..N
            {
                let mut candidate = T::MIN;
                for ro in 0..N
                {
                    for co in 0..N
                    {
                        candidate += T::ONE;
                        self.candidate[row][col][ro][co] = candidate;
                    }
                }
                self.shuffle_small(row, col);
            }
        }
    }

    fn shuffle_small(&mut self, row: usize, col: usize)
    {
        let (mut ro, mut co) = (N - 1, N - 1);
        while (ro, co) != (0, 0)
        {
            let r = self.random.random_under_uint_(ro + 1);
            let c = self.random.random_under_uint_(co + 1);
            let tmp = self.candidate[row][col][ro][co];
            self.candidate[row][col][ro][co] = self.candidate[row][col][r][c];
            self.candidate[row][col][r][c] = tmp;
            (ro, co) = Self::retreat_small(ro, co);
        }
    }

    fn shuffle_big(&mut self, elem: &mut [[[[T; N]; N]; N]; N])
    {
        let (mut row, mut col, mut ro, mut co) = (N - 1, N - 1, N - 1, N - 1);
        while (row, col, ro, co) != (0, 0, 0, 0)
        {
            let rr = self.random.random_under_uint_(row + 1);
            let cc = self.random.random_under_uint_(col + 1);
            let r = self.random.random_under_uint_(ro + 1);
            let c = self.random.random_under_uint_(co + 1);
            let tmp = elem[row][col][ro][co];
            elem[row][col][ro][co] = elem[rr][cc][r][c];
            elem[rr][cc][r][c] = tmp;
            (row, col, ro, co) = Self::retreat_big(row, col, ro, co);
        }
    }

    #[allow(unused)]
    fn advance_small(mut ro: usize, mut co: usize) -> (usize, usize)
    {
        (ro, co) = (ro % N, co % N);
        co += 1;
        if co == N
        {
            co = 0;
            ro += 1;
        }
        (ro, co)
    }

    fn advance_big(mut row: usize, mut col: usize, mut ro: usize, mut co: usize) -> (usize, usize, usize, usize)
    {
        (row, col, ro, co) = (row % N, col % N, ro % N, co % N);
        co += 1;
        if co == N
        {
            co = 0;
            ro += 1;
        }
        if ro == N
        {
            ro = 0;
            col += 1;
        }
        if col == N
        {
            col = 0;
            row += 1;
        }
        (row, col, ro, co)
    }

    fn retreat_small(mut ro: usize, mut co: usize) -> (usize, usize)
    {
        (ro, co) = (ro % N, co % N);
        if co == 0
        {
            co = N - 1;
            if ro == 0
            {
                ro = N;
                co -= 1;
            }
            else
                { ro -= 1; }
        }
        else
            { co -= 1; }

        (ro, co)
    }

    fn retreat_big(mut row: usize, mut col: usize, mut ro: usize, mut co: usize) -> (usize, usize, usize, usize)
    {
        (row, col, ro, co) = (row % N, col % N, ro % N, co % N);
        if co == 0
        {
            co = N - 1;
            if ro == 0
            {
                ro = N - 1;
                if col == 0
                {
                    col = N;
                    if row == 0
                        { row = N; }
                    else
                        { row -= 1; }
                }
                col -= 1;
            }
            else
                { ro -= 1; }
        }
        else
            { co -= 1; }

        (row, col, ro, co)
    }

    #[inline]
    fn index_into_2d_view(row: usize, col: usize, ro: usize, co: usize) -> (usize, usize)
    {
        (row * N + ro, col * N + co)
    }

    #[allow(unused)]
    #[inline]
    fn index_into_4d_view(row: usize, col: usize) -> (usize, usize, usize, usize)
    {
        (row / N, col / N, row % N, col % N)
    }

    #[inline]
    fn assert() -> bool
    {
        N > 1 && T::MAX.into_u128() >= (N as u128 * N as u128)
    }
}