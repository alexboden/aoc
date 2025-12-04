use std::io::{self, Read};

fn is_repeat(i: i64) -> bool {
    let s = i.to_string();
	for j in 1..s.len() {
		if s.len() % j != 0 {
			continue
		}
		if s[0..j].repeat(s.len() / j) == s {
			return true
		}
	}
	return false
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let ranges: Vec<(i64, i64)> = buf
        .trim()
        .split(',')
        .filter_map(|chunk| {
            let mut parts = chunk.splitn(2, '-');
            match (parts.next(), parts.next()) {
                (Some(start), Some(end)) => Some((start.parse().ok()?, end.parse().ok()?)),
                _ => None,
            }
        })
        .collect();

	let mut res = 0;

    for (start, end) in &ranges {
        for i in *start..*end + 1 {
            if is_repeat(i) {
                res += i;
            }
        }

    }

	println!("{}", res);

    Ok(())
}
