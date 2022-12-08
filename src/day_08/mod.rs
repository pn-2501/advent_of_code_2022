use std::ops::Range;
use advent_of_code::utils::inputs::get_file;

static HEIGHT: usize = 99;
static WIDTH: usize = 99;


pub fn day_08() {
    let trees = get_input();

    let solution_a = part_a(&trees);
    println!("\t- Solution A is : {}", solution_a);

    let solution_b = part_b(&trees);
    println!("\t- Solution B is : {}", solution_b);
}


fn get_input() -> Vec<Vec<u8>> {
    get_file("./src/day_08/input.txt")
        .lines()
        .map(
            |l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
        )
        .collect()
}


fn part_a(trees: &[Vec<u8>]) -> usize {
    (0..HEIGHT)
        .map(
            |y| (0..WIDTH).map(move |x| is_visible(trees, y, x))
        )
        .flatten()
        .filter(|is_visible| *is_visible)
        .count()
}


fn is_visible(trees: &[Vec<u8>], y: usize, x: usize) -> bool {
    let north_range = 0..y;
    let south_range = (y + 1)..HEIGHT;
    let east_range = 0..x;
    let west_range = (x + 1)..WIDTH;

    is_visible_iter(trees, trees[y][x], north_range, x..x + 1)
        || is_visible_iter(trees, trees[y][x], south_range, x..x + 1)
        || is_visible_iter(trees, trees[y][x], y..y + 1, east_range)
        || is_visible_iter(trees, trees[y][x], y..y + 1, west_range)
}


fn is_visible_iter(trees: &[Vec<u8>], val: u8, range_y: Range<usize>, range_x: Range<usize>) -> bool {
    range_y
        .map(
            |y| range_x.clone().map(move |x| trees[y][x])
        )
        .flatten()
        .all(|t| val > t)
}


fn part_b(trees: &[Vec<u8>]) -> usize {
    (0..HEIGHT)
        .map(
            |y| (0..WIDTH).map(move |x| get_scenic_score(trees, y, x))
        )
        .flatten()
        .max()
        .unwrap()
}

fn get_scenic_score(trees: &[Vec<u8>], y: usize, x: usize) -> usize {
    let north_range = (0..y).rev();
    let south_range = (y + 1)..HEIGHT;
    let east_range = (0..x).rev();
    let west_range = (x + 1)..WIDTH;

    get_score(trees, trees[y][x], north_range, x..x + 1) *
        get_score(trees, trees[y][x], south_range, x..x + 1) *
        get_score(trees, trees[y][x], y..y + 1, east_range) *
        get_score(trees, trees[y][x], y..y + 1, west_range)
}


fn get_score<X, Y>(trees: &[Vec<u8>], value: u8, range_y: Y, range_x: X) -> usize
    where
        X: Iterator<Item=usize> + Clone,
        Y: Iterator<Item=usize> + Clone,
{
    let mut count = 0;
    for y in range_y {
        for x in range_x.clone() {
            count += 1;
            if value <= trees[y][x] {
                return count;
            }
        }
    }
    count
}
