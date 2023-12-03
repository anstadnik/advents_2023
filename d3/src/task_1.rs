use regex::Regex;

pub(crate) fn task_1(s: &str) -> usize {
    let (n, m) = (s.lines().count(), s.lines().next().unwrap().len());
    let mut is_good_coordinate = vec![vec![false; m]; n];
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if !c.is_ascii_digit() && c != '.' {
                for y in [y - 1, y, y + 1] {
                    for x in [x - 1, x, x + 1] {
                        if let Some(v) = is_good_coordinate.get_mut(y).and_then(|v| v.get_mut(x)) {
                            *v = true;
                        }
                    }
                }
            }
        }
    }
    let regex = Regex::new(r"(\d+)").unwrap();
    s.lines()
        .zip(&is_good_coordinate)
        .flat_map(|(line, is_gc_line)| {
            regex
                .find_iter(line)
                .filter(|m| is_gc_line[m.start()..m.end()].iter().any(|&b| b))
                .map(move |m| m.as_str().parse::<usize>().unwrap())
        })
        .sum::<usize>()
}

