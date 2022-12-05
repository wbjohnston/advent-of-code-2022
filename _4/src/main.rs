mod input;

type Range = (u64, u64);

fn parse_line(s: &'static str) -> (Range, Range) {
    let ranges: Vec<_> = s.split(",").map(parse_range).collect();

    (ranges[0], ranges[1])
}

fn parse_range(line: &'static str) -> (u64, u64) {
    let start = line
        .chars()
        .take_while(|&c| c != '-')
        .collect::<String>()
        .parse()
        .unwrap();

    // dbg!(start);

    let end = line
        .chars()
        .skip_while(|&c| c != '-')
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap();

    (start, end)
}

fn sort(a: Range, b: Range) -> (Range, Range) {
    if a.0 < b.0 {
        return (a, b);
    } else if a.0 > b.0 {
        return (b, a);
    } else {
        // ==
        if a.1 > b.1 {
            (a, b)
        } else {
            (b, a)
        }
    }
}

fn overlaps(a: Range, b: Range) -> bool {
    let (lower, upper) = sort(a, b);

    /*
    3 cases
    |-----|
        |-----------|

        |-----|
    |----------|

            |-----|
    |-----------|
    */

    lower.0 <= upper.0 && lower.1 >= upper.0
}

fn fully_contains(a: Range, b: Range) -> bool {
    let (lower, upper) = sort(a, b);

    println!("{:?} <= {:?}", lower, upper);

    lower.0 <= upper.0 && lower.1 >= upper.1
}

fn main() {
    let c = input::INPUT
        .split("\n")
        .map(parse_line)
        .filter(|&(a, b)| overlaps(a, b))
        .count();

    println!("{:?}", c);
}
