fn main() {
    let input = std::fs::read_to_string("input").expect("file 'input' not found");

    let mut count = 0;
    for line in input.lines() {
        let mut parts = line.split(&[' ', '-', ':'][..]);
        let min = parts.next().unwrap().parse::<usize>().unwrap();
        let max = parts.next().unwrap().parse::<usize>().unwrap();
        let l = parts.next().unwrap().chars().nth(0).unwrap();
        let passwd = parts.collect::<String>();
        let sum: usize = passwd
            .chars()
            .fold(0, |acc, c| if c == l { acc + 1 } else { acc });
        if min <= sum && sum <= max {
            count = count + 1;
        }
    }
    println!("part 1: {}", count);
    println!("{}", true ^ false);
    println!("{}", false ^ true);
    println!("{}", true ^ true);
    println!("{}", false ^ false);

    let mut count = 0;
    for line in input.lines() {
        let mut parts = line.split(&[' ', '-', ':'][..]);
        let min = parts.next().unwrap().parse::<usize>().unwrap();
        let max = parts.next().unwrap().parse::<usize>().unwrap();
        let l = parts.next().unwrap().chars().nth(0).unwrap();

        let passwd = parts.collect::<String>();
        let mut passwd_chars = passwd.chars();

        let first = passwd_chars.nth(min - 1).unwrap();
        let last = passwd_chars.nth(max - min - 1).unwrap();
        println!("{:?}[{}] = {}", passwd, min - 1, first);
        println!("{:?}[{}] = {}", passwd, max - 1, last);

        if (first == l) ^ (last == l) {
            count = count + 1;
        }
    }
    println!("part 2: {}", count);
}
