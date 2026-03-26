# Sudoku-plus: An Expandable Sudoku Library

`sudoku-plus` is a versatile Rust library designed for generating and solving various Sudoku structures, including __Plane__, __Multi-plane__, and __Cubic__ Sudoku.

## Roadmap for Version 1.0

The following features are planned for the `sudoku-plus` ecosystem.

- [X] **Completed:** Implementation and documentation are at least **95%** complete.
- [ ] __In Progress:__ Implementation or documentation is below __95%__, or work has not yet begun.


### 1. 2D Sudoku

- [ ] **PlaneSudoku:** A generic 2D Sudoku with a (N^2 X N^2) grid. You can define the size by choosing the constant `N`. --
      [PlaneSudoku](https://docs.rs/sudoku-plus/latest/sudoku_plus/plane_sudoku/struct.PlaneSudoku.html.PlaneSudoku)


### 2. 3D Sudoku

- [ ] **Multi-plane Sudoku:** A 3D Sudoku structure with dimensions of (N^2 X N^2 X N^2). The size is determined by the constant `N`.  --
      [MultiplaneSudoku](https://docs.rs/sudoku-plus/latest/sudoku_plus/multiplane_sudoku/struct.MultiplaneSudoku.html.MultiplaneSudoku)

- [ ] **Cubic Sudoku:** A 3D Sudoku structure with dimensions of (N^3 X N^3 X N^3). The size is determined by the constant `N`. --
      [CubicSudoku](https://docs.rs/sudoku-plus/latest/sudoku_plus/cubic_sudoku/struct.CubicSudoku.html.CubicSudoku)


### 3. Sudoku Elements

- [ ] **Sudoku Element:** A generic Sudoku component designed for building complex applications, such as academic timetable generators powered by Sudoku algorithms. --
      [SudokuElement](https://docs.rs/sudoku-plus/latest/sudoku_plus/sudoku_element/struct.SudokuElement.html.SudokuElement).


## Versioning Policy
The project will reach Version 1.0.0 once all functional areas listed above are fully implemented.

- __Pre-v1.0:__ Versions will range up to 0.3.x based on the progress of the listed functionalities.
- **Post-v1.0:** New features and stable releases will follow standard semantic versioning beyond 1.0.0.

_Note: Version numbers like 0.2.0 indicate progress through the functionality list, not necessarily a 20% completion of the entire codebase._

## Breaking Changes
For a detailed history of API changes and migration guides, please refer to `BreakingChanges.md` for a detailed history of API changes.

