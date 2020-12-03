fn main() {
    let input = std::fs::read_to_string("input").expect("file 'input' not found");
    let numbers: Vec<u32> = input
        .lines()
        .map(|s| s.parse::<u32>().expect("not a number"))
        .collect();
    for n in &numbers {
        for n2 in &numbers {
            for n3 in &numbers {
                if n + n2 + n3 == 2020 {
                    eprintln!("{} + {} + {} = 2020", n, n2, n3);
                    println!("{}", n * n2 * n3);
                    return;
                }
            }
        }
    }
}
