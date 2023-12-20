
#[cfg(test)]
use super::*;

#[test]
fn test_1() {
    let s = concat!(
        "R 6 (#70c710)\n",
        "D 5 (#0dc571)\n",
        "L 2 (#5713f0)\n",
        "D 2 (#d2c081)\n",
        "R 2 (#59c680)\n",
        "D 2 (#411b91)\n",
        "L 5 (#8ceee2)\n",
        "U 2 (#caa173)\n",
        "L 1 (#1b58a2)\n",
        "U 2 (#caa171)\n",
        "R 2 (#7807d2)\n",
        "U 3 (#a77fa3)\n",
        "L 2 (#015232)\n",
        "U 2 (#7a21e3)\n",
    );
    let f = parse::parse_input1(s).unwrap();
    assert_eq!(task(&f).unwrap(), 62);
}

// #[ignore]
#[test]
fn test_2() {
    let s = concat!(
        "R 6 (#70c710)\n",
        "D 5 (#0dc571)\n",
        "L 2 (#5713f0)\n",
        "D 2 (#d2c081)\n",
        "R 2 (#59c680)\n",
        "D 2 (#411b91)\n",
        "L 5 (#8ceee2)\n",
        "U 2 (#caa173)\n",
        "L 1 (#1b58a2)\n",
        "U 2 (#caa171)\n",
        "R 2 (#7807d2)\n",
        "U 3 (#a77fa3)\n",
        "L 2 (#015232)\n",
        "U 2 (#7a21e3)\n",
    );
    let f = parse::parse_input2(s).unwrap();
    let f_ = vec![
        (Right, 461937),
        (Down, 56407),
        (Right, 356671),
        (Down, 863240),
        (Right, 367720),
        (Down, 266681),
        (Left, 577262),
        (Up, 829975),
        (Left, 112010),
        (Down, 829975),
        (Left, 491645),
        (Up, 686074),
        (Left, 5411),
        (Up, 500254),
    ];
    assert_eq!(f, f_);
    let ret = task(&f).unwrap();
    assert_eq!(ret, 952408144115);
}
