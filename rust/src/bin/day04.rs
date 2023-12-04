fn main() {
    let input = include_str!("input/day04/input.txt");
    println!("part 1: {}", solve1(input));
    println!("part 2: {}", solve2(input));
}

fn solve1(lines: &str) -> u64 {
    todo!()
}

fn solve2(lines: &str) -> u64 {
    todo!()
}

#[test]
fn example01() {
    let example = include_str!("input/day04/example01.txt");
    assert_eq!(solve1(example), 4361);
}

#[test]
fn example02() {
    let example = include_str!("input/day04/example01.txt");
    assert_eq!(solve2(example), 467835);
}
