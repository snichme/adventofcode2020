fn run_slope(lines: std::str::Lines, right: usize, down: usize) -> usize {
    let mut line_no = 0;
    let mut x = 0;
    let mut hit = 0;
    for line in lines {
        if line_no > 0 && line_no % down != 0 {
            line_no += 1;
            continue;
        }
        let point = &line[x..(x + 1)];
        eprintln!("#{} {}[{}] = {}", line_no, line, x, point);
        if line_no > 0 && point == "#" {
            hit += 1;
        }
        x += right;
        if x > 30 {
            x -= 31;
        }
        line_no += 1;
    }
    hit
}

fn main() {
    let input = std::fs::read_to_string("input").expect("file 'input' not found");
    let lines = input.lines();
    let mut res = 1;

    let hits = run_slope(lines.clone(), 1, 1);
    res *= hits;
    println!("Right 1, down 1: {}", hits);

    let hits = run_slope(lines.clone(), 3, 1);
    res *= hits;
    println!("Right 3, down 1: {}", hits);

    let hits = run_slope(lines.clone(), 5, 1);
    res *= hits;
    println!("Right 5, down 1: {}", hits);

    let hits = run_slope(lines.clone(), 7, 1);
    res *= hits;
    println!("Right 7, down 1: {}", hits);

    let hits = run_slope(lines, 1, 2);
    res *= hits;
    println!("Right 1, down 2: {}", hits);

    println!("{}", res);
}
