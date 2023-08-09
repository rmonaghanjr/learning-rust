pub fn part1(path: &str) -> u32 {
    let input = std::fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(|bag| {
            bag.lines().map(|n| n.parse::<u32>().unwrap()).sum()
        })
        .max()
        .unwrap();

    return input;
}

pub fn part2(path: &str) -> u32 {
    let mut input: Vec<u32> = std::fs::read_to_string(path)
        .unwrap()
        .split("\n\n")
        .map(|bag| {
            bag.lines().map(|n| n.parse::<u32>().unwrap()).sum()
        })
        .collect();
    input.sort_by(|a, b| b.cmp(a));
    input.truncate(3);

    return input.iter().sum();
}

