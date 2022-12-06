use std::collections::HashSet;

mod input;

fn get_chars_prefix(s: &'static str) -> Option<usize> {
    'outer: for i in 0..s.len() {
        let mut seen = HashSet::new();
        for j in (i)..(s.len().min(i + 14)) {
            let curr = s.chars().nth(j).unwrap();
            println!("  {}", curr);
            if seen.contains(&curr) {
                continue 'outer;
            }

            seen.insert(curr);
        }
        return Some(i);
    }

    None
}

fn main() {
    let input = input::INPUT;

    dbg!(get_chars_prefix(input));
}
