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


/// A type alias for a 8x8x8 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 2`, resulting in a total grid size of 8x8x8 
/// (2^3 x 2^3 x 2^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_8x8x8 = CubicSudoku<u8, 2>;

/// A type alias for a 27x27x27 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 3`, resulting in a total grid size of 27x27x27 
/// (3^3 x 3^3 x 3^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_27x27x27 = CubicSudoku<u8, 3>;

/// A type alias for a 64x64x64 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 4`, resulting in a total grid size of 64x64x64 
/// (4^3 x 4^3 x 4^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_64x64x64 = CubicSudoku<u8, 4>;

/// A type alias for a 125x125x125 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 5`, resulting in a total grid size of 125x125x125 
/// (5^3 x 5^3 x 5^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_125x125x125 = CubicSudoku<u8, 5>;

/// A type alias for a 216x216x216 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 6`, resulting in a total grid size of 216x216x216 
/// (6^3 x 6^3 x 6^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_216x216x216 = CubicSudoku<u8, 6>;

/// A type alias for a 343x343x343 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 7`, resulting in a total grid size of 343x343x343 
/// (7^3 x 7^3 x 7^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_343x343x343 = CubicSudoku<u16, 7>;

/// A type alias for a 512x512x512 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 8`, resulting in a total grid size of 512x512x512
/// (8^3 x 8^3 x 8^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_512x512x512 = CubicSudoku<u16, 8>;

/// A type alias for a 729x729x729 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 9`, resulting in a total grid size of 729x729x729 
/// (9^3 x 9^3 x 9^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_729x729x729 = CubicSudoku<u16, 9>;

/// A type alias for a 1000x1000x1000 `CubicSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 10`, resulting in a total grid size of 1000x1000x1000
/// (10^3 x 10^3 x 10^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_1000x1000x1000 = CubicSudoku<u16, 10>;

/// A type alias for a 1331x1331x1331 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 11`, resulting in a total grid size of 1331x1331x1331
/// (11^3 x 11^3 x 11^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_1331x1331x1331 = CubicSudoku<u16, 11>;

/// A type alias for a 1728x1728x1728 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 12`, resulting in a total grid size of 1728x1728x1728
/// (12^3 x 12^3 x 12^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_1728x1728x1728 = CubicSudoku<u16, 12>;

/// A type alias for a 2197x2197x2197 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 13`, resulting in a total grid size of 2197x2197x2197
/// (13^3 x 13^3 x 13^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_2197x2197x2197 = CubicSudoku<u16, 13>;

/// A type alias for a 2744x2744x2744 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 14`, resulting in a total grid size of 2744x2744x2744
/// (14^3 x 14^3 x 14^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_2744x2744x2744 = CubicSudoku<u16, 14>;

/// A type alias for a 3375x3375x3375 `CubicSudoku` using `u8` as the underlying type.
/// 
/// This configuration uses `N = 15`, resulting in a total grid size of 3375x3375x3375
/// (15^3 x 15^3 x 15^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_3375x3375x3375 = CubicSudoku<u16, 15>;

/// A type alias for a 4096x4096x4096 `CubicSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 16`, resulting in a total grid size of 4096x4096x4096
/// (16^3 x 16^3 x 16^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_4096x4096x4096 = CubicSudoku<u16, 16>;

/// A type alias for a 4913x4913x4913 `CubicSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 17`, resulting in a total grid size of 4913x4913x4913
/// (17^3 x 17^3 x 17^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_4913x4913x4913 = CubicSudoku<u16, 17>;

/// A type alias for a 5832x5832x5832 `CubicSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 18`, resulting in a total grid size of 5832x5832x5832
/// (18^3 x 18^3 x 18^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_5832x5832x5832 = CubicSudoku<u16, 18>;

/// A type alias for a 6859x6859x6859 `CubicSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 19`, resulting in a total grid size of 6859x6859x6859
/// (19^3 x 19^3 x 19^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_6859x6859x6859 = CubicSudoku<u16, 19>;

/// A type alias for a 8000x8000x8000 `CubicSudoku` using `u16` as the underlying type.
/// 
/// This configuration uses `N = 20`, resulting in a total grid size of 8000x8000x8000
/// (20^3 x 20^3 x 20^3).
#[allow(non_camel_case_types)]
pub type CubicSudoku_8000x8000x8000 = CubicSudoku<u16, 20>;


/// # Introduction
/// Sudoku is one of the world's most popular puzzle games. While standard
/// Sudoku is a 2D puzzle
/// (see [PlaneSudoku](../plane_sudoku/struct.PlaneSudoku.html)), 
/// `CubicSudoku` extends this concept into three dimensions.
///
/// `CubicSudoku` is designed for 3D puzzles structured as a cube where rules
/// apply across all three axes (X, Y, and Z). 
///
/// ### CubicSudoku vs MultiLayerSudoku
/// While both are 3D extensions, they differ in their sub-unit constraints:
/// - **MultiLayerSudoku**: Uses `N x N` sub-squares within each 2D layer.
/// - **CubicSudoku**: Uses `N x N x N` **sub-cubes**. Every group of `N` layers
///   shares these sub-cubes, and each sub-cube must contain all unique numbers
///   from 1 to N^3.
///
/// This provides a more complex and engaging experience by requiring 
/// volumetric thinking to solve.
///
/// # Generic Constants
/// - `T`: The underlying data type. It must support
///   `cryptocol::number::SmallUInt`.
///   Currently, `u8`, `u16`, `u32`, `u64`, `u128`, and `usize` are supported.
///   The default is `u8`.
/// - `N`: Determines the dimensions. `N` must be greater than 1.
///   - Each sub-cube is of size `N x N x N`.
///   - The total grid size is `N^3 x N^3 x N^3`.
///   - The value `N^3` must be representable within the range of type `T`
///     (`N^3 <= T::MAX`).
///   - For example, if `N = 2`, the cube is an 8x8x8 grid composed of
///     2x2x2 sub-cubes. Default is `N = 2`.
///
/// # Rules for Cubic Sudoku
/// A `CubicSudoku` is a 3D puzzle with dimensions of N^3 x N^3 x N^3.
///
/// ## Core Structure
/// - **3D Grid**: A cubic volume containing (N^3)^3 total cells.
/// - **Planar Slices**: The cube consists of N^3 slices along each primary
///   axis (X, Y, and Z).
/// - **Sub-cubes**: The total volume is divided into N^6 smaller
///   **N x N x N sub-cubes**, each containing N^3 cells.
///
/// ## Puzzle Constraints
/// For a valid solution, every unit of length N^3 and every sub-cube must
/// satisfy the following:
///
/// 1. **Linear Uniqueness (1D)**:
///    - Any straight line of N^3 cells parallel to the **X, Y, or Z axis**
///      must contain all unique numbers from 1 to N^3.
///
/// 2. **Volumetric Uniqueness (3D Sub-cubes)**:
///    - Each **N x N x N sub-cube** must contain all unique numbers from 1 to N^3.
///    - These sub-cubes are shared across `N` consecutive layers in each direction.
///      When viewed from any axis (X, Y, or Z), the sub-cube constraint 
///      must be satisfied within every group of `N` slices.
/// 
/// # Type Aliases
/// There are predefined type aliases for common Sudoku sizes,
/// such as `CubicSudoku_8X8X8` for a standard 8x8x8 Sudoku.
/// You can use these type aliases to easily create Sudoku instances of the
/// desired size without having to specify the generic parameters each time.
/// The following type aliases are available:
/// - `CubicSudoku_8X8X8` for a 8x8x8 Sudoku (N=2)
/// - `CubicSudoku_9X9X9` for a 27x27x27 Sudoku (N=3)
/// - `CubicSudoku_16X16X16` for a 16x16x16 Sudoku (N=4)
/// - `CubicSudoku_25X25X25` for a 25x25x25 Sudoku (N=5)
/// - `CubicSudoku_36X36X36` for a 36x36x36 Sudoku (N=6)
/// - `CubicSudoku_49X49X49` for a 49x49x49 Sudoku (N=7)
/// - `CubicSudoku_64X64X64` for a 64x64x64 Sudoku (N=8)
/// - `CubicSudoku_81X81X81` for an 81x81x81 Sudoku (N=9)
/// - `CubicSudoku_100X100X100` for a 100x100x100 Sudoku (N=10)
/// - `CubicSudoku_121X121X121` for a 121x121x121 Sudoku (N=11)
/// - `CubicSudoku_144X144X144` for a 144x144x144 Sudoku (N=12)
/// - `CubicSudoku_169X169X169` for a 169x169x169 Sudoku (N=13)
/// - `CubicSudoku_196X196X196` for a 196x196x196 Sudoku (N=14)
/// - `CubicSudoku_225X225X225` for a 225x225x225 Sudoku (N=15)
/// - `CubicSudoku_289X289X289` for a 289x289x289 Sudoku (N=16)
/// - `CubicSudoku_324X324X324` for a 324x324x324 Sudoku (N=18)
/// - `CubicSudoku_361X361X361` for a 361x361x361 Sudoku (N=19)
/// - `CubicSudoku_400X400X400` for a 400x400x400 Sudoku (N=20)
/// 
/// You can use these type aliases to create Sudoku instances of the desired
/// size without having to specify the generic parameters each time.
/// For example, to create a standard 9x9 Sudoku,
/// you can simply use `PlaneSudoku_9X9::new()`.
pub struct CubicSudoku<T: SmallUInt = u8, const N: usize = 2>
{
    sudoku: Vec<Vec<Vec<[[[T; N]; N]; N]>>>,
    candidate: Vec<Vec<Vec<[[[T; N]; N]; N]>>>,
    random: RandGen,
}

impl<T: SmallUInt, const N: usize> CubicSudoku<T, N>
{
    pub fn new() -> Option<Self>
    {
        if !Self::assert()
        {
            None
        }
        else
        {
            let subcubes = [[[T::MIN; N]; N]; N];
            let mut row = Vec::<[[[T; N]; N]; N]>::with_capacity(N * N);
            row.fill(subcubes);

            let mut layer = Vec::<Vec::<[[[T; N]; N]; N]>>::with_capacity(N * N);
            layer.fill(row);

            let mut sudoku = Vec::<Vec::<Vec::<[[[T; N]; N]; N]>>>::with_capacity(N * N);
            sudoku.fill(layer);

            let mut candidate = Vec::<Vec::<Vec::<[[[T; N]; N]; N]>>>::with_capacity(N * N);
            candidate.fill(layer);

            let mut me = Self
            {
                sudoku,
                candidate,
                random: Random::new(),
            };
            me.create_candidates();
            Some(me)
        }
    }

    
    pub fn new_with(problem: Vec::<Vec::<Vec::<[[[T; N]; N]; N]>>>) -> Option<Self>
    {
        if !Self::assert()
        {
            None
        }
        else
        {
            let subcubes = [[[T::MIN; N]; N]; N];
            let mut row = Vec::<[[[T; N]; N]; N]>::with_capacity(N * N);
            row.fill(subcubes);

            let mut layer = Vec::<Vec::<[[[T; N]; N]; N]>>::with_capacity(N * N);
            layer.fill(row);

            let mut candidate = Vec::<Vec::<Vec::<[[[T; N]; N]; N]>>>::with_capacity(N * N);
            candidate.fill(layer);

            let mut me = Self
            {
                sudoku: problem,
                candidate,
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

        let mut me = Self::new()?;
        
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
        for l in 0..N
        {
            for r in 0..N
            {
                for c in 0..N
                {
                    if la == l && ro == r && co == c
                        { continue; }
                    if point == self.sudoku[lay][l][row][col][r][c]
                        { return false; } 
                }
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
        for lay in 0..(N * N)
        {
            for row in 0..(N * N)
            {
                for col in 0..(N * N)
                {
                    let mut candidate = T::MIN;
                    for la in 0..N
                    {
                        for ro in 0..N
                        {
                            for co in 0..N
                            {
                                candidate += T::ONE;
                                self.candidate[lay][row][col][la][ro][co] = candidate;
                            }
                        }
                    }
                    self.shuffle_candidates(lay, row, col);
                }
            }
        }
    }

    fn shuffle_candidates(&mut self, lay: usize, row: usize, col: usize)
    {
        let (mut la, mut ro, mut co) = (N - 1, N - 1, N - 1);
        while (la, ro, co) != (0, 0, 0)
        {
            let l = self.random.random_under_uint_(la + 1);
            let r = self.random.random_under_uint_(ro + 1);
            let c = self.random.random_under_uint_(co + 1);

            let tmp = self.candidate[lay][row][col][la][ro][co];
            self.candidate[lay][row][col][la][ro][co] = self.candidate[lay][row][col][la][r][c];
            self.candidate[lay][row][col][la][r][c] = tmp;
            (la, ro, co) = Self::retreat_small(la, ro, co);
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

    fn retreat_small(mut la: usize, mut ro: usize, mut co: usize) -> (usize, usize, usize)
    {
        co = co % N;

        if co != 0
            { co -= 1; }
        else
            { co = N; }
        (ro, co) = Self::borrow(ro % N, co);
        (la, ro) = Self::borrow(la % N, ro);
        (la, ro, co)
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
