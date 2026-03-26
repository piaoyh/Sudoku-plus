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
    /// Creates a new object of `PlaneSudoku` wrapped by `Some`.
    /// 
    /// # Returns
    /// A new object of `PlaneSudoku` wrapped by `Some` if the size is valid.
    /// `None` if the size is invalid.
    pub fn new() -> Option<Self>
    {
        if T::MAX.into_u128() < (N as u128 * N as u128)
            { None }
        else
        {
            Some(
                Self
                {
                    sudoku: [[[[T::zero(); N]; N]; N]; N],
                    random: Random::new(),
                }
            )
        }
    }

    // pub fn new_with(problem: [[[[T; N]; N]; N]; N]) -> Option<Self>
    /// Creates a new object of `PlaneSudoku` wrapped by `Some`
    /// out of problem array.
    /// 
    /// # Arguments
    /// - `problem`: is a 4-dimensional array of `T`
    ///   with the size `N` X `N` X `N` X `N`.
    /// 
    /// # Returns
    /// A new object of `PlaneSudoku` wrapped by `Some` if the size is valid.
    /// `None` if the size is invalid.
    pub fn new_with(problem: [[[[T; N]; N]; N]; N]) -> Option<Self>
    {
        if T::MAX.into_u128() < (N as u128 * N as u128)
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
    /// Creates a new object of `PlaneSudoku` wrapped by `Some`
    /// out of problem array.
    /// 
    /// # Arguments
    /// - `problem`: is a 4-dimensional array of `T`
    ///   with the size `M` X `M`, where `M` == `N^2`.
    /// 
    /// # Returns
    /// A new object of `PlaneSudoku` wrapped by `Some` if `M` == `N^2`.
    /// `None` if `M` != `N^2`.
    pub fn new_with_<const M: usize>(problem: [[T; M]; M]) -> Option<Self>
    {
        if N * N != M || T::MAX.into_u128() < (M as u128)
            { return None; }

        let mut me = Self
        {
            sudoku: [[[[T::zero(); N]; N]; N]; N],
            random: Random::new(),
        };
        
        for row in 0..N
        {
            for col in 0..N
            {
                for ro in 0..N
                {
                    for co in 0..N
                        { me.sudoku[row][col][ro][co] = problem[row * N + ro][col * N + co]; }
                }
            }
        }
        Some(me)
    }

    // pub fn generate(&mut self, n_holes: usize)
    /// Generates a sudoku problem with `n_holes` holes (blanks).
    /// 
    /// # Arguments
    /// - `n_holes`: is the number of holes (blanks) in the generated sudoku
    ///   board.
    /// 
    /// # Features
    /// - If `n_holes` is `zero`, the generated sudoku board is
    ///   a complete solution.
    pub fn generate(&mut self, n_holes: usize)
    {
        self.sudoku = [[[[T::zero(); N]; N]; N]; N];
        self.solve();
        for _ in 0..n_holes
        {
            let row = self.random.random_under_uint_(N);
            let col = self.random.random_under_uint_(N);
            let ro = self.random.random_under_uint_(N);
            let co = self.random.random_under_uint_(N);
            self.sudoku[row][col][ro][co] = T::MIN;
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
            let left = self.count_left_elements(ro, co);
            for _ in 1..left
            {
                self.rotate(elem, ro, co);
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
        (row, col, ro, co) = self.advance(row, col, ro, co);
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
        self.shuffle(&mut elem);
        elem
    }

    fn shuffle(&mut self, elem: &mut [[T; N]; N])
    {
        for i in 0..N
            { self.random.shuffle(&mut elem[i]); }
        self.random.shuffle(elem);
    }

    fn advance(&self, mut row: usize, mut col: usize, mut ro: usize, mut co: usize) -> (usize, usize, usize, usize)
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

    fn rotate(&self, elem: &mut [[T; N]; N], ro: usize, co: usize)
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
    fn count_left_elements(&self, ro: usize, co: usize) -> usize
    {
        N - co + (N - 1 - ro) * N
    }
}