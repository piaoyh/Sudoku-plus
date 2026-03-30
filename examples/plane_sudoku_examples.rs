
fn main()
{
    constructors();
    generaters_and_get_from_2d_view();
    solve();
    // advance();
}


fn constructors()
{
    new_1();
    new_2();
    new_3();
    new_4();
    new_with_1();
    new_with_2();
    new_with_2d_1();
    new_with_2d_2();
}

fn new_1()
{
    println!("Testing new() for Basic 9x9 Sudoku initialization (N=3):");
    use sudoku_plus::PlaneSudoku_9x9;
    let sudoku = PlaneSudoku_9x9::new();
    assert!(sudoku.is_some());
    println!("----------------------------")
}

fn new_2()
{
    println!("Testing new() for Using a larger grid size with u16 to avoid overflow:");
    use sudoku_plus::PlaneSudoku;
    let sudoku = PlaneSudoku::<u16, 16>::new();
    assert!(sudoku.is_some());
    println!("----------------------------");
}

fn new_3()
{
    println!("Testing new() for Size constraints leading to None:");
    use sudoku_plus::PlaneSudoku;
    let sudoku = PlaneSudoku::<u8, 16>::new();
    assert!(sudoku.is_none());
    println!("----------------------------");
}

fn new_4()
{
    println!("Testing new() for Handling potential None for invalid configurations:");
    use sudoku_plus::PlaneSudoku;
    if let Some(_sudoku) = PlaneSudoku::<u8, 3>::new()
        { println!("Successfully created a 9 x 9 Sudoku!"); }
    println!("----------------------------");
}

fn new_with_1()
{
    println!("Testing new_with() for Initializing with a specific puzzle pattern");
    use sudoku_plus::PlaneSudoku_9x9;
    let problem: [[[[u8; 3]; 3]; 3]; 3] = [
        [[[4, 0, 6], [3, 2, 7], [9, 0, 8]], 
         [[9, 2, 3], [6, 1, 8], [4, 7, 5]], 
         [[1, 7, 8], [9, 4, 5], [3, 6, 2]]], 
        [[[2, 3, 1], [8, 0, 4], [5, 0, 7]], 
         [[6, 8, 4], [5, 7, 9], [2, 3, 1]], 
         [[7, 9, 5], [2, 3, 1], [6, 8, 4]]], 
        [[[8, 1, 0], [4, 0, 6], [7, 0, 3]], 
         [[5, 6, 2], [7, 8, 3], [1, 4, 9]], 
         [[3, 0, 7], [1, 0, 2], [8, 0, 6]]]];
    let sudoku = PlaneSudoku_9x9::new_with(problem);
    assert!(sudoku.is_some());
    println!("----------------------------");
}

fn new_with_2()
{
    println!("Testing new_with() for Error handling for invalid input data");
    use sudoku_plus::PlaneSudoku_4x4;
    let problem: [[[[u8; 2]; 2]; 2]; 2] = [
        [[[1, 0], [0, 2]],
         [[0, 0], [0, 0]]],
        [[[0, 0], [0, 0]],
         [[0, 0], [3, 4]]]];
    let sudoku = PlaneSudoku_4x4::new_with(problem);
    match sudoku
    {
        Some(_) => println!("Valid puzzle loaded"),
        None => println!("Invalid puzzle dimensions"),
    }
    assert!(sudoku.is_some());
    println!("----------------------------");
}

fn new_with_2d_1()
{
    println!("Testing new_with_2d() for Initializing with a specific puzzle pattern");
    use sudoku_plus::PlaneSudoku_9x9;
    let problem: [[u8; 9]; 9] = [
        [4, 0, 6, 3, 2, 7, 9, 0, 8], 
        [9, 2, 3, 6, 1, 8, 4, 7, 5], 
        [1, 7, 8, 9, 4, 5, 3, 6, 2], 
        [2, 3, 1, 8, 0, 4, 5, 0, 7], 
        [6, 8, 4, 5, 7, 9, 2, 3, 1], 
        [7, 9, 5, 2, 3, 1, 6, 8, 4], 
        [8, 1, 0, 4, 0, 6, 7, 0, 3],
        [5, 6, 2, 7, 8, 3, 1, 4, 9], 
        [3, 0, 7, 1, 0, 2, 8, 0, 6]];
    let sudoku = PlaneSudoku_9x9::new_with_2d(problem);
    assert!(sudoku.is_some());
    println!("----------------------------");
}

fn new_with_2d_2()
{
    println!("Testing new_with_2d() for Error handling for invalid input data");
    use sudoku_plus::PlaneSudoku_4x4;
    let problem: [[u8; 4]; 4] = [
        [1, 0, 0, 2],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 3, 4]];
    let sudoku = PlaneSudoku_4x4::new_with_2d(problem);
    match sudoku
    {
        Some(_) => println!("Valid puzzle loaded"),
        None => println!("Invalid puzzle dimensions"),
    }
    assert!(sudoku.is_some());
    println!("----------------------------");
}


fn generaters_and_get_from_2d_view()
{
    generate_1();
    generate_2();
    generate_3();
}

fn generate_1()
{
    println!("Testing generate_1() for Generating a puzzle with a specific difficulty (40 holes)");
    use sudoku_plus::PlaneSudoku_9x9;
    let mut sudoku = PlaneSudoku_9x9::new().unwrap();
    sudoku.generate(40);
    for row in 0..9
    {
        for col in 0..9
        {
            let point = sudoku.get_from_2d_view(row, col);
            if point == 0
                { print!("_ "); }
            else
                { print!("{} ", point); }
        }
        println!();
    }
    println!("----------------------------");
}

fn generate_2()
{
    println!("Testing generate_2() for Generating a fully solved board (0 holes)");
    use sudoku_plus::PlaneSudoku_16x16;
    let mut sudoku = PlaneSudoku_16x16::new().unwrap();
    sudoku.generate(0);
    for row in 0..16
    {
        for col in 0..16
        {
            let point = sudoku.get_from_2d_view(row, col);
            if point == 0
                { print!("__ "); }
            else
                { print!("{:02} ", point); }
        }
        println!();
    }
    println!("----------------------------");
}

fn generate_3()
{
    println!("Testing generate_3() for Handling oversized n_holes (results in an empty board)");
    use sudoku_plus::PlaneSudoku_4x4;
    let mut sudoku = PlaneSudoku_4x4::new().unwrap();
    sudoku.generate(20); // 4x4 grid has 16 cells, so this will result in an empty board
    for row in 0..4
    {
        for col in 0..4
        {
            let point = sudoku.get_from_2d_view(row, col);
            if point == 0
                { print!("_ "); }
            else
                { print!("{} ", point); }
        }
        println!();
    }
    println!("----------------------------");
}


fn solve()
{
    solve_1();
    solve_2();
    solve_3();
}

fn solve_1()
{
    println!("Testing solve() for Solving a standard 9x9 Sudoku puzzle");
    use sudoku_plus::PlaneSudoku_9x9;
    let mut sudoku = PlaneSudoku_9x9::new().unwrap();
    sudoku.generate(40); // Generate a puzzle with 40 holes
    let success = sudoku.solve();
    if success
        { println!("Sudoku solved successfully!"); }
    else
        { println!("No solution exists for this puzzle."); }
    assert!(success);
    println!("----------------------------");
}

fn solve_2()
{
    println!("Testing solve() for Verifying a solution for a small 4x4 grid (N=2)");
    use sudoku_plus::PlaneSudoku_4x4;
    let mut sudoku = PlaneSudoku_4x4::new().unwrap();
    sudoku.generate(7); // Generate a puzzle with 7 holes, which should be solvable
    assert!(sudoku.solve(), "4x4 puzzles generated this way should be solvable");
    println!("----------------------------");
}

fn solve_3()
{
    println!("Testing solve() for Handling an unsolvable custom puzzle");
    use sudoku_plus::PlaneSudoku_4x4;
    let problem: [[u8; 4]; 4] = [
        // Set conflicting values in the same row/column/block
        [1, 0, 0, 3],
        [0, 0, 2, 0],
        [0, 2, 0, 0],
        [0, 0, 3, 4]];
    let mut sudoku = PlaneSudoku_4x4::new_with_2d(problem).unwrap();
    let success = sudoku.solve();   // Should return false for invalid puzzle states
    assert!(!success, "This puzzle should be unsolvable due to conflicting values");
    println!("----------------------------");
}

/*
fn advance()
{
    use sudoku_plus::PlaneSudoku;

    println!("Advance small:");
    let (mut row, mut col, mut ro, mut co) = (0, 0, 0, 0);
    while ro < 3
    {
        println!("({}, {})", ro, co);
        (ro, co) = PlaneSudoku::<u8, 3>::advance_small(ro, co);
    }

    println!("Advance big:");
    (row, col, ro, co) = (0, 0, 0, 0);
    while row < 3
    {
        println!("({}, {}, {}, {})", row, col, ro, co);
        (row, col, ro, co) = PlaneSudoku::<u8, 3>::advance_big(row, col, ro, co);
    }
    
    println!("Retreat small:");
    (row, col, ro, co) = (2, 2, 2, 2);
    while ro < 3
    {
        println!("({}, {})", ro, co);
        (ro, co) = PlaneSudoku::<u8, 3>::retreat_small(ro, co);
    }

    println!("Retreat big:");
    (row, col, ro, co) = (2, 2, 2, 2);
    while row < 3
    {
        println!("({}, {}, {}, {})", row, col, ro, co);
        (row, col, ro, co) = PlaneSudoku::<u8, 3>::retreat_big(row, col, ro, co);
    }
}
*/