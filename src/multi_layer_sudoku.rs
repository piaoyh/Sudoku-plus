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


/// A type alias for a 4x4x4 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 2`, resulting in a total grid size of 4x4x4 
/// (2^2 x 2^2 x 2^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_4X4X4 = MultiLayerSudoku<u8, 2>;

/// A type alias for a 9x9x9 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 3`, resulting in a total grid size of 9x9x9 
/// (3^2 x 3^2 x 3^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_9X9X9 = MultiLayerSudoku<u8, 3>;

/// A type alias for a 16x16x16 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 4`, resulting in a total grid size of 16x16x16 
/// (4^2 x 4^2 x 4^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_16X16X16 = MultiLayerSudoku<u8, 4>;

/// A type alias for a 25x25x25 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 5`, resulting in a total grid size of 25x25x25 
/// (5^2 x 5^2 x 5^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_25X25X25 = MultiLayerSudoku<u8, 5>;

/// A type alias for a 36x36x36 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 6`, resulting in a total grid size of 36x36x36 
/// (6^2 x 6^2 x 6^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_36X36X36 = MultiLayerSudoku<u8, 6>;

/// A type alias for a 49x49x49 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 7`, resulting in a total grid size of 49x49x49 
/// (7^2 x 7^2 x 7^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_49X49X49 = MultiLayerSudoku<u8, 7>;

/// A type alias for a 64x64x64 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 8`, resulting in a total grid size of 64x64x64 
/// (8^2 x 8^2 x 8^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_64X64X64 = MultiLayerSudoku<u8, 8>;

/// A type alias for an 81x81x81 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 9`, resulting in a total grid size of 81x81x81 
/// (9^2 x 9^2 x 9^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_81X81X81 = MultiLayerSudoku<u8, 9>;

/// A type alias for a 100x100x100 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 10`, resulting in a total grid size of 100x100x100 
/// (10^2 x 10^2 x 10^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_100X100X100 = MultiLayerSudoku<u8, 10>;

/// A type alias for a 121x121x121 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 11`, resulting in a total grid size of 121x121x121
/// (11^2 x 11^2 x 11^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_121X121X121 = MultiLayerSudoku<u8, 11>;

/// A type alias for a 144x144x144 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 12`, resulting in a total grid size of 144x144x144
/// (12^2 x 12^2 x 12^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_144X144X144 = MultiLayerSudoku<u8, 12>;

/// A type alias for a 169x169x169 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 13`, resulting in a total grid size of 169x169x169
/// (13^2 x 13^2 x 13^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_169X169X169 = MultiLayerSudoku<u8, 13>;

/// A type alias for a 196x196x196 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 14`, resulting in a total grid size of 196x196x196
/// (14^2 x 14^2 x 14^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_196X196X196 = MultiLayerSudoku<u8, 14>;

/// A type alias for a 225x225x225 `MultiLayerSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 15`, resulting in a total grid size of 225x225x225
/// (15^2 x 15^2 x 15^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_225X225X225 = MultiLayerSudoku<u8, 15>;

/// A type alias for a 256x256x256 `MultiLayerSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 16`, resulting in a total grid size of 256x256x256
/// (16^2 x 16^2 x 16^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_256X256X256 = MultiLayerSudoku<u16, 16>;

/// A type alias for a 289x289x289 `MultiLayerSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 17`, resulting in a total grid size of 289x289x289
/// (17^2 x 17^2 x 17^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_289X289X289 = MultiLayerSudoku<u16, 17>;

/// A type alias for a 324x324x324 `MultiLayerSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 18`, resulting in a total grid size of 324x324x324
/// (18^2 x 18^2 x 18^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_324X324X324 = MultiLayerSudoku<u16, 18>;

/// A type alias for a 361x361x361 `MultiLayerSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 19`, resulting in a total grid size of 361x361x361
/// (19^2 x 19^2 x 19^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_361X361X361 = MultiLayerSudoku<u16, 19>;

/// A type alias for a 400x400x400 `MultiLayerSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 20`, resulting in a total grid size of 400x400x400
/// (20^2 x 20^2 x 20^2).
#[allow(non_camel_case_types)]
pub type MultiLayerSudoku_400X400X400 = MultiLayerSudoku<u16, 20>;


/// # Introduction
/// Sudoku is the one of the famous puzzle games. The standard Sudoku that you
/// know is a 2D Sudoku puzzle. If you are interested in it, please refer to
/// [PlaneSudoku](../plane_sudoku/struct.PlaneSudoku.html#struct.PlaneSudoku).
/// This struct `MultiLayerSudoku` is designed for 3D Sudoku puzzles, which are
/// 3D Sudoku puzzles, this struct `MultiLayerSudoku` is designed to handle
/// 3D Sudoku puzzles, which are an extension of the traditional 2D Sudoku into
/// three dimensions. In a 3D Sudoku, the grid is structured as a cube
/// (multi-layered), and the rules of Sudoku apply across all three dimensions.
/// Each layer of the cube must contain all unique numbers, and the same applies
/// to rows and columns across layers. This struct provides functionality for
/// generating and solving 3D Sudoku puzzles of various sizes, allowing for a
/// more complex and engaging puzzle-solving experience compared to traditional
/// 2D Sudoku. Its size is a (N^2 X N^2 X N^2) grid.
/// You can define the size by choosing the generic constant `N`.
/// 
/// # Generic Constants
/// - `T`: is supposed to be any data type that supports
///   cryptocol::number::SmallUInt. At the momnet, `T` can be one of `u8`,
///   `u16`, `u32`, `u64`, `u128`, and `usize`.
///   In the future, this crate may provide some useful datatypes for `T`
///   that can substitute for cryptocol::number::SmallUInt.
///   The default type is `u8`.
/// - `N`: determines the size of sudoku board. `N` should be greater than `1`.
///   `N` indicates the size of a sub-square, which is `N` x `N`.
///   The sudoku cube is composed of `N` layers of `N` rows of `N` columns of
///   sub-squares. The size `N^2` should be within the representable range of
///   the underlying type `T` (`N^2 <= T::MAX`).
///   So, the size of the sudoku cube is N^2 x N x N x N x N
///   (= N^2 x N^2 x N^2). For example, if `N` = 2, the size of the sudoku cube
///   is 2 x 2 x 2. The default value of `N` = `2`.
/// 
/// # Rules for Multi-layer Sudoku (MultiLayerSudoku)
/// A `MultiLayerSudoku` is a 3-dimensional puzzle structured as a cube with 
/// dimensions of (N^2 x N^2 x N^2). It is composed of N^2 internal layers 
/// along each axis.
///
/// ## Core Structure
/// - **3D Grid**: A cubic volume containing (N^2)^3 total cells.
/// - **Planar Slices**: The cube can be viewed as a stack of N^2 slices 
///   along any of the three primary axes (X, Y, or Z).
///
/// ## Puzzle Constraints
/// To be a valid solution, every unit of length N^2 and every sub-grid 
/// within the slices must satisfy the following:
///
/// 1. **Linear Uniqueness (1D)**:
///    - Any line of N^2 cells parallel to the **X, Y, or Z axis** must 
///      contain all unique numbers from 1 to N^2.
///
/// 2. **Planar Sudoku Rules (2D Slices)**:
///    - Every 2D slice (orthogonal to any axis) must function as a 
///      valid 2D Sudoku. 
///    - This means for any fixed coordinate on one axis, the resulting 
///      N^2 x N^2 plane follows standard Sudoku rules.
///
/// 3. **Sub-square Constraints (N x N blocks)**:
///    - When looking through the cube from any face (X, Y, or Z direction), 
///      each N x N sub-square in every slice must contain all unique 
///      numbers from 1 to N^2.
///    - This applies to:
///      - **XY-planes** (all slices along the Z-axis)
///      - **YZ-planes** (all slices along the X-axis)
///      - **ZX-planes** (all slices along the Y-axis)
/// 
/// # Type Aliases
/// There are predefined type aliases for common Sudoku sizes,
/// such as `PlaneSudoku_9X9` for a standard 9x9 Sudoku.
/// You can use these type aliases to easily create Sudoku instances of the
/// desired size without having to specify the generic parameters each time.
/// The following type aliases are available:
/// - `MultiLayerSudoku_4X4X4` for a 4x4x4 Sudoku (N=2)
/// - `MultiLayerSudoku_9X9X9` for a 9x9x9 Sudoku (N=3)
/// - `MultiLayerSudoku_16X16X16` for a 16x16x16 Sudoku (N=4)
/// - `MultiLayerSudoku_25X25X25` for a 25x25x25 Sudoku (N=5)
/// - `MultiLayerSudoku_36X36X36` for a 36x36x36 Sudoku (N=6)
/// - `MultiLayerSudoku_49X49X49` for a 49x49x49 Sudoku (N=7)
/// - `MultiLayerSudoku_64X64X64` for a 64x64x64 Sudoku (N=8)
/// - `MultiLayerSudoku_81X81X81` for an 81x81x81 Sudoku (N=9)
/// - `MultiLayerSudoku_100X100X100` for a 100x100x100 Sudoku (N=10)
/// - `MultiLayerSudoku_121X121X121` for a 121x121x121 Sudoku (N=11)
/// - `MultiLayerSudoku_144X144X144` for a 144x144x144 Sudoku (N=12)
/// - `MultiLayerSudoku_169X169X169` for a 169x169x169 Sudoku (N=13)
/// - `MultiLayerSudoku_196X196X196` for a 196x196x196 Sudoku (N=14)
/// - `MultiLayerSudoku_225X225X225` for a 225x225x225 Sudoku (N=15)
/// - `MultiLayerSudoku_289X289X289` for a 289x289x289 Sudoku (N=16)
/// - `MultiLayerSudoku_324X324X324` for a 324x324x324 Sudoku (N=18)
/// - `MultiLayerSudoku_361X361X361` for a 361x361x361 Sudoku (N=19)
/// - `MultiLayerSudoku_400X400X400` for a 400x400x400 Sudoku (N=20)
/// 
/// You can use these type aliases to create Sudoku instances of the desired
/// size without having to specify the generic parameters each time.
/// For example, to create a standard 9x9 Sudoku,
/// you can simply use `PlaneSudoku_9X9::new()`.
pub struct MultiLayerSudoku<T: SmallUInt = u8, const N: usize = 2>
{
    sudoku: [[[[[[T; N]; N]; N]; N]; N]; N],
    candidate: [[[[[[T; N]; N]; N]; N]; N]; N],
    random: RandGen,
}

impl<T: SmallUInt, const N: usize> MultiLayerSudoku<T, N>
{
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
                sudoku: [[[[[[T::MIN; N]; N]; N]; N]; N]; N],
                candidate: [[[[[[T::MIN; N]; N]; N]; N]; N]; N],
                random: Random::new(),
            };
            me.create_candidates();
            Some(me)
        }
    }

    
    pub fn new_with(problem: [[[[[[T; N]; N]; N]; N]; N]; N]) -> Option<Self>
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
                candidate: [[[[[[T::MIN; N]; N]; N]; N]; N]; N],
                random: Random::new(),
            };
            me.create_candidates();
            Some(me)
        }
    }


    pub fn new_with_3d<const M: usize>(problem: [[[T; M]; M]; M]) -> Option<Self>
    {
        if N * N != M || !Self::assert()
            { return None; }

        let mut me = Self
        {
            sudoku: [[[[[[T::MIN; N]; N]; N]; N]; N]; N],
            candidate: [[[[[[T::MIN; N]; N]; N]; N]; N]; N],
            random: Random::new(),
        };
        
        for lay in 0..N
        {
            for la in 0..N
            {
                for row in 0..N
                {
                    for col in 0..N
                    {
                        for ro in 0..N
                        {
                            for co in 0..N
                            {
                                let (ll, rr, cc) = Self::index_into_3d_view(lay, la, row, col, ro, co);
                                me.sudoku[lay][la][row][col][ro][co] = problem[ll][rr][cc];
                            }
                        }
                    }
                }
            }
        }
        me.create_candidates();
        Some(me)
    }

    #[inline]
    pub fn get_from_3d_view(&self, lay: usize, row: usize, col: usize) -> T
    {
        self.sudoku[lay / N][lay % N][row / N][col / N][row % N][col % N]
    }

    pub fn generate(&mut self, n_holes: usize)
    {
        self.sudoku = [[[[[[T::MIN; N]; N]; N]; N]; N]; N];
        self.solve();
        let mut mask = [[[[[[T::ONE; N]; N]; N]; N]; N]; N];
        let (mut lay, mut la,   mut row, mut col, mut ro, mut co) = (0_usize, 0_usize, 0_usize, 0_usize, 0_usize, 0_usize);
        for _ in 0..(if n_holes <= N * N * N * N * N * N {n_holes} else {N * N * N * N * N * N})
        {
            mask[lay][la][row][col][ro][co] = T::MIN;
            (lay, la, row, col, ro, co) = Self::advance_big(lay, la, row, col, ro, co);
        }
        self.shuffle_big(&mut mask);

        for lay in 0..N
        {
            for la in 0..N
            {
                for row in 0..N
                {
                    for col in 0..N
                    {
                        for ro in 0..N
                        {
                            for co in 0..N
                            {
                                if mask[lay][la][row][col][ro][co] == T::MIN
                                    { self.sudoku[lay][la][row][col][ro][co] = T::MIN; }
                            }
                        }
                    }
                }
            }
        }
    }

    
    pub fn solve(&mut self) -> bool
    {
        self.solve_step_by_step( 0, 0, 0, 0, 0, 0)
    }

    fn solve_step_by_step(&mut self, mut lay: usize, mut la: usize, mut row: usize, mut col: usize, mut ro: usize, mut co: usize) -> bool
    {
        while self.sudoku[lay][la][row][col][ro][co] != T::MIN
        {
            (lay, la, row, col, ro, co) = Self::advance_big(lay, la, row, col, ro, co);
            if lay == N
                { return true; }
        }

        for r in 0..N
        {
            for c in 0..N
            {
                let point = self.candidate[lay][la][row][col][r][c];
                if self.check(lay, la, row, col, ro, co, point)
                {
                    self.sudoku[lay][la][row][col][ro][co] = point;
                    (lay, la, row, col, ro, co) = Self::advance_big(lay, la, row, col, ro, co);
                    if lay == N || self.solve_step_by_step(lay, la, row, col, ro, co)
                        { return true; }
                    (lay, la, row, col, ro, co) = Self::retreat_big(lay, la, row, col, ro, co);
                    self.sudoku[lay][la][row][col][ro][co] = T::MIN;
                }
            }
        }
        false
    }

    fn check(&self, lay: usize, la: usize, row: usize, col: usize, ro: usize, co: usize, point: T) -> bool
    {
        for r in 0..N
        {
            for c in 0..N
            {
                if ro == r && co == c
                    { continue; }
                if point == self.sudoku[lay][la][row][col][r][c]
                    { return false; } 
            }
        }

        for l in 0..N
        {
            if l == la
                { continue; }
            for rc in 0..N
            {
                if point == self.sudoku[lay][l][row][col][ro][rc]
                || point == self.sudoku[lay][l][row][col][rc][co]
                    { return false; } 
            }
        }

        for rrcc in 0..N
        {
            if rrcc == row || rrcc == col
                { continue; }

            for lrc in 0..N
            {
                if point == self.sudoku[lay][la][rrcc][col][lrc][co]
                || point == self.sudoku[lay][la][row][rrcc][ro][lrc]
                    { return false; }
            }
        }

        for ll in 0..N
        {
            for l in 0..N
            {
                if ll == lay && l == la
                    { continue; }
                if point == self.sudoku[ll][l][row][col][ro][co]
                    { return false; }
            }
        }
        true
    }

    fn create_candidates(&mut self)
    {
        for lay in 0..N
        {
            for la in 0..N
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
                                self.candidate[lay][la][row][col][ro][co] = candidate;
                            }
                        }
                        self.shuffle_candidates(lay, la, row, col);
                    }
                }
            }
        }
    }

    fn shuffle_candidates(&mut self, lay: usize, la: usize, row: usize, col: usize)
    {
        let (mut ro, mut co) = (N - 1, N - 1);
        while (ro, co) != (0, 0)
        {
            let r = self.random.random_under_uint_(ro + 1);
            let c = self.random.random_under_uint_(co + 1);

            let tmp = self.candidate[lay][la][row][col][ro][co];
            self.candidate[lay][la][row][col][ro][co] = self.candidate[lay][la][row][col][r][c];
            self.candidate[lay][la][row][col][r][c] = tmp;
            (ro, co) = Self::retreat_small(ro, co);
        }
    }

    fn shuffle_big(&mut self, elem: &mut [[[[[[T; N]; N]; N]; N]; N]; N])
    {
        let (mut lay, mut la,  mut row, mut col, mut ro, mut co) = (N - 1, N - 1, N - 1, N - 1, N - 1, N - 1);
        while (lay, la, row, col, ro, co) != (0, 0, 0, 0, 0, 0)
        {
            let ll = self.random.random_under_uint_(lay + 1);
            let l = self.random.random_under_uint_(la + 1);
            let rr = self.random.random_under_uint_(row + 1);
            let cc = self.random.random_under_uint_(col + 1);
            let r = self.random.random_under_uint_(ro + 1);
            let c = self.random.random_under_uint_(co + 1);

            let tmp = elem[lay][la][row][col][ro][co];
            elem[lay][la][row][col][ro][co] = elem[ll][l][rr][cc][r][c];
            elem[ll][l][rr][cc][r][c] = tmp;
            (lay, la, row, col, ro, co) = Self::retreat_big(lay, la, row, col, ro, co);
        }
    }

    fn carry(mut high: usize, mut low: usize) -> (usize, usize)
    {
        if low == N
        {
            low = 0;
            high += 1;
        }
        (high, low)
    }

    #[allow(unused)]
    fn advance_small(ro: usize, co: usize) -> (usize, usize)
    {
        Self::carry(ro % N, (co % N) + 1)
    }

    #[allow(unused)]
    fn advance_middle(mut row: usize, mut col: usize, mut ro: usize, mut co: usize) -> (usize, usize, usize, usize)
    {
        (ro, co) = Self::advance_small(ro % N, co % N);
        (col, ro) = Self::carry(col % N, ro);
        (row, col) = Self::carry(row % N, col);
        (row, col, ro, co)
    }

    fn advance_big(mut lay: usize, mut la: usize, mut row: usize, mut col: usize, mut ro: usize, mut co: usize) -> (usize, usize, usize, usize, usize, usize)
    {
        (ro, co) = Self::advance_small(ro % N, co % N);
        (col, ro) = Self::carry(col % N, ro);
        (row, col) = Self::carry(row % N, col);
        (la, row) = Self::carry(la % N, row);
        (lay, la) = Self::carry(lay % N, la);
        (lay, la, row, col, ro, co)
    }

    fn borrow(mut high: usize, mut low: usize) -> (usize, usize)
    {
        if low == N
        {
            low = N - 1;
            if high == 0
                { high = N; }
            else
                { high -= 1; }
        }
        (high, low)
    }

    fn retreat_small(ro: usize, mut co: usize) -> (usize, usize)
    {
        co = co % N;

        if co != 0
            { co -= 1; }
        else
            { co = N; }
        Self::borrow(ro % N, co)
    }

    #[allow(unused)]
    fn retreat_middle(mut row: usize, mut col: usize, mut ro: usize, mut co: usize) -> (usize, usize, usize, usize)
    {
        co = co % N;

        if co != 0
            { co -= 1; }
        else
            { co = N; }
        (ro, co) = Self::borrow(ro % N, co);
        (col, ro) = Self::borrow(col % N, ro);
        (row, col) = Self::borrow(row % N, col);
        
        (row, col, ro, co)
    }

    fn retreat_big(mut lay: usize, mut la: usize, mut row: usize, mut col: usize, mut ro: usize, mut co: usize) -> (usize, usize, usize, usize, usize, usize)
    {
        co = co % N;

        if co != 0
            { co -= 1; }
        else
            { co = N; }
        (ro, co) = Self::borrow(ro % N, co);
        (col, ro) = Self::borrow(col % N, ro);
        (row, col) = Self::borrow(row % N, col);
        (la, row) = Self::borrow(la % N, row);
        (lay, la) = Self::borrow(lay % N, la);
        (lay, la, row, col, ro, co)
    }

    #[inline]
    fn index_into_3d_view(lay: usize, la: usize, row: usize, col: usize, ro: usize, co: usize) -> (usize, usize, usize)
    {
        (lay * N + la, row * N + ro, col * N + co)
    }

    #[allow(unused)]
    #[inline]
    fn index_into_6d_view(lay: usize, row: usize, col: usize) -> (usize, usize, usize, usize, usize, usize)
    {
        (lay / N, lay % N, row / N, col / N, row % N, col % N)
    }

    #[inline]
    fn assert() -> bool
    {
        N > 1 && T::MAX.into_u128() >= (N as u128 * N as u128)
    }
}
