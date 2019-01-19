fn display(board: &Vec<Vec<i32>>) {
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 0 {
                print!("{}", ".");
            } else {
                print!("{}", "*")
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn conway(grid: Vec<Vec<i32>>, m: usize, n: usize) -> Vec<Vec<i32>> {
    let mut future = vec![vec![0; n]; m];
    for l in 1..m - 1 {
        for m in 1..n - 1 {
            let mut alive_neighbours = 0;
            for i in -1i32..=1i32 {
                for j in -1i32..=1i32 {
                    alive_neighbours += grid[(l as i32 + i) as usize][(m as i32 + j) as usize];
                }
            }
            alive_neighbours -= grid[l][m];
            if (grid[l][m] == 1) && (alive_neighbours < 2) {
                future[l][m] = 0;
            } else if (grid[l][m] == 1) && (alive_neighbours > 3) {
                future[l][m] = 0;
            } else if (grid[l][m] == 0) && (alive_neighbours == 3) {
                future[l][m] = 1;
            } else {
                future[l][m] = grid[l][m];
            }
        }
    }
    println!("Next Generation");
    display(&future);
    future
}


fn main() {
    let m = 10usize;
    let n = 10usize;
    let generations = 10;
    let mut grid = vec![vec![0; n]; m];
    grid[1][3] = 1;
    grid[1][4] = 1;
    grid[2][4] = 1;
    grid[5][3] = 1;
    grid[5][4] = 1;
    grid[6][2] = 1;
    grid[6][3] = 1;
    grid[7][5] = 1;
    grid[8][4] = 1;
    println!("Original Generation");
    display(&grid);
    for _ in 0..generations {
        grid = conway(grid, m, n);
    }
}