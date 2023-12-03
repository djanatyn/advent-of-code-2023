use std::io;

fn main() -> io::Result<()> {
    let input = include_str!("input/day02/input.txt");
    println!("part 1: {}", solve1(input));
    Ok(())
}

fn solve1(lines: &str) -> u64 {
    todo!();
}

#[test]
fn example01() {
    let example = include_str!("input/day01/example01.txt");
    assert_eq!(solve1(example), todo!());
}
