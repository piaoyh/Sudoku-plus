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
///   In the future, this crate will provide some useful datatypes for `T`
///   that supports cryptocol::number::SmallUInt + Copy + Clone + Eq.
///   The default type is `u8`.
/// - `N`: determines the size of sudoku board.
///   `N` indicates the size of a sub-square, which is `N` X `N`.
///   The sudoku board is composed of `N` rows of `N` columns of sub-squares.
///   So, the size of the sudoku board is N^2 X N X N (= N^2 X N^2).
///   For example, if `N` = 3, the size of the sudoku board is 9 X 9.
///   The default value of `N` = `3`.
pub struct PlaneSudoku<T: SmallUInt = u8, const N: usize = 3>
{
    sudoku: [[[[T; N]; N]; N]; N],
    random: RandGen,
}

impl<T: SmallUInt, const N: usize> PlaneSudoku<T, N>
{
    // pub fn new() -> Option<Self>
    /// Creates a new `PlaneSudoku` instance wrapped in `Some`.
    /// 
    /// # Returns
    /// - `Some(Self)` if the size `N^2` is within the representable range
    ///   of the underlying type `T` (`N^2 <= T::MAX`).
    /// - `None` if `N^2` exceeds the maximum value of `T`.
    pub fn new() -> Option<Self>
    {
        if !Self::assert()
            { None }
        else
        {
            Some(
                Self
                {
                    sudoku: [[[[T::MIN; N]; N]; N]; N],
                    random: Random::new(),
                }
            )
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
    /// - `Some(Self)` if the size `N^2` is within the representable range 
    ///   of the underlying type `T` (`N^2 <= T::MAX`).
    /// - `None` if `N^2` exceeds the maximum value of `T`.
    pub fn new_with(problem: [[[[T; N]; N]; N]; N]) -> Option<Self>
    {
        if !Self::assert()
            { None }
        else
        {
            Some(
                Self
                {
                    sudoku: problem,
                    random: Random::new(),
                }
            )
        }
    }

    // pub fn new_with_<const M: usize>(problem: [[T; M]; M]) -> Option<Self>
    /// Creates a new `PlaneSudoku` instance from a 2-dimensional problem array.
    /// 
    /// # Arguments
    /// * `problem` - A 2D array of type `T` with dimensions `M x M`.
    ///   Here, `M` must satisfy the condition `M == N * N`.
    /// 
    /// # Returns
    /// - `Some(Self)` if the dimensions of the input array are valid
    ///   and consistent with the constraint `M == N * N`
    ///   and the size `M` is within the representable range
    ///   of the underlying type `T` (`M <= T::MAX`).
    /// - `None` if the dimensions do not satisfy the required Sudoku 
    ///   grid constraints or `M` exceeds the maximum value of `T`.
    pub fn new_with_<const M: usize>(problem: [[T; M]; M]) -> Option<Self>
    {
        if N * N != M || !Self::assert()
            { return None; }

        let mut me = Self
        {
            sudoku: [[[[T::MIN; N]; N]; N]; N],
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
                        let (r, c) = Self::index_into_2D_view(row, col, ro, co);
                        me.sudoku[row][col][ro][co] = problem[r][c];
                    }
                }
            }
        }
        Some(me)
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
    pub fn generate(&mut self, n_holes: usize)
    {
        self.sudoku = [[[[T::zero(); N]; N]; N]; N];
        self.solve();
        let mut mask = [[[[T::ONE; N]; N]; N]; N];
        let (mut row, mut col, mut ro, mut co) = (0_usize, 0_usize, 0_usize, 0_usize);
        for _ in 0..(if n_holes <= N * N {n_holes} else {N * N})
        {
            mask[row][col][ro][co] = T::MIN;
            (row, col, ro, co) = Self::advance(row, col, ro, co);
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

    // pub fn check(&self, row: usize, col: usize, ro: usize, co: usize, point: T) -> bool
    /// Checks whether `point` can be fit to the location (row, col, ro, co).
    /// 
    /// # Arguments
    /// - `row`: is vertical location of the sub-square in the sudoku board.
    /// - `col`: is horizontal location of the sub-square in the sudoku board.
    /// - `ro`: is vertical location of the blank in the sub-square.
    /// - `co`: is horizontal location of the blank in the sub-square.
    /// 
    /// # Returns
    /// - `true` if `point` can be fit to the location (row, col, ro, co).
    /// - `false` if `point` cannot be fit to the location (row, col, ro, co).
    pub fn check(&self, row: usize, col: usize, ro: usize, co: usize, point: T) -> bool
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

        let rr = row;
        for cc in 0..N
        {
            if cc == col
                { continue; }
            for c in 0..N
            {
                if point == self.sudoku[rr][cc][ro][c]
                    { return false; } 
            }
        }

        let cc = col;
        for rr in 0..N
        {
            if rr == row
                { continue; }
            for r in 0..N
            {
                if point == self.sudoku[rr][cc][r][co]
                    { return false; } 
            }
        }
        true
    }

    // pub fn solve(&mut self) -> bool
    /// Solves the given sudoku problem.
    /// 
    /// # Returns
    /// - `true` if there is a solution and this method solves
    ///   the given problem successfully.
    /// - `false` if there is no solution.
    pub fn solve(&mut self) -> bool
    {
        let mut elem = self.create_random_stack();
        self.solve_step_by_step(&mut elem, 0, 0, 0, 0)
    }

    fn solve_step_by_step(&mut self, elem: &mut [[T; N]; N], row: usize, col: usize, ro: usize, co: usize) -> bool
    {
        let mut point = elem[ro][co];
        return if self.check(row, col, ro, co, point)
        {
            self.fill(elem, row, col, ro, co, point)
        }
        else
        {
            let left = Self::count_left_elements(ro, co);
            for _ in 1..left
            {
                Self::rotate(elem, ro, co);
                point = elem[ro][co];
                if self.check(row, col, ro, co, point)
                    { return self.solve_step_by_step(elem, row, col, ro, co); }
            }
            false
        }
    }

    fn fill(&mut self, elem: &mut [[T; N]; N], mut row: usize, mut col: usize, mut ro: usize, mut co: usize, point: T) -> bool
    {
        self.sudoku[row][col][ro][co] = point;
        (row, col, ro, co) = Self::advance(row, col, ro, co);
        if row == 0
        {
            true
        }
        else
        {
            if ro == 0 && co == 0
            {
                let mut elem = self.create_random_stack();
                self.solve_step_by_step(&mut elem, row, col, ro, co)
            }
            else
            {
                self.solve_step_by_step(elem, row, col, ro, co)
            }
        }
    }

    fn create_random_stack(&mut self) -> [[T; N]; N]
    {
        let mut elem = [[T::MIN; N]; N];
        let mut n = T::one();
        for r in 0..N
        {
            for c in 0..N
            {
                elem[r][c] = n;
                n += T::ONE;
            }
        }
        self.shuffle_small(&mut elem);
        elem
    }

    fn shuffle_small(&mut self, elem: &mut [[T; N]; N])
    {
        let mut list = Vec::with_capacity(N * N);
        let mut i = 0_usize;
        for r in 0..N
        {
            for c in 0..N
            {
                list[i] = elem[r][c];
                i += 1;
            }
        }
        self.random.shuffle(&mut list);
        i = 0_usize;
        for r in 0..N
        {
            for c in 0..N
            {
                elem[r][c] = list[i];
                i += 1;
            }
        }
    }

    fn shuffle_big(&mut self, elem: &mut [[[[T; N]; N]; N]; N])
    {
        let mut list = Vec::with_capacity(N * N * N * N);
        let mut i = 0_usize;
        for row in 0..N
        {
            for col in 0..N
            {
                for ro in 0..N
                {
                    for co in 0..N
                    {
                        list[i] = elem[row][col][ro][co];
                        i += 1;
                    }
                }
            }
        }
        self.random.shuffle(&mut list);
        i = 0_usize;
        for row in 0..N
        {
            for col in 0..N
            {
                for ro in 0..N
                {
                    for co in 0..N
                    {
                        elem[row][col][ro][co] = list[i];
                        i += 1;
                    }
                }
            }
        }
    }

    fn advance(mut row: usize, mut col: usize, mut ro: usize, mut co: usize) -> (usize, usize, usize, usize)
    {
        co += 1;
        if co == N
        {
            ro += 1;
            co = 0;
        }
        if ro == N
        {
            col += 1;
            ro = 0;
        }
        if col == N
        {
            row += 1;
            col = 0;
        }
        if row == N
        {
            row = 0;
        }
        (row, col, ro, co)
    }

    fn rotate(elem: &mut [[T; N]; N], ro: usize, co: usize)
    {
        let tmp = elem[ro][co];
        for cc in co..N-1
            { elem[ro][cc] = elem[ro][cc+1]; }
        if ro != N - 1
            { elem[ro][N-1] = elem[ro+1][0]; }
        for rr in ro+1..N-1
        {
            for cc in 0..N-1
                { elem[rr][cc] = elem[rr][cc+1]; }
            elem[rr][N-1] = elem[rr+1][0];
        }
        for cc in 0..N-1
            { elem[N-1][cc] = elem[N-1][cc+1]; }
        if ro != N - 1
            { elem[N-1][N-1] = tmp; }
    }

    #[inline]
    fn count_left_elements(ro: usize, co: usize) -> usize
    {
        N - co + (N - 1 - ro) * N
    }

    // fn get_from_2D(&self, row: usize, col: usize)
    fn get_from_2D_view(&self, row: usize, col: usize) -> T
    {
        self.sudoku[row / N][col / N][row % N][col % N]
    }

    #[inline]
    fn index_into_2D_view(row: usize, col: usize, ro: usize, co: usize) -> (usize, usize)
    {
        (row * N + ro, col * N + co)
    }
    
    #[inline]
    fn index_into_4D_view(row: usize, col: usize) -> (usize, usize, usize, usize)
    {
        (row / N, col / N, row % N, col % N)
    }

    #[inline]
    fn assert() -> bool
    {
        T::MAX.into_u128() >= (N as u128 * N as u128)
    }
}