#[cfg(test)]
use super::*;

#[test]
fn test_1() -> Result<()> {
    let test_cases = [
        (
            concat!(
                "#.##..##.\n",
                "..#.##.#.\n",
                "##......#\n",
                "##......#\n",
                "..#.##.#.\n",
                "..##..##.\n",
                "#.#.##.#.\n",
            ),
            Reflection::Col(5),
        ),
        (
            concat!(
                "#...##..#\n",
                "#....#..#\n",
                "..##..###\n",
                "#####.##.\n",
                "#####.##.\n",
                "..##..###\n",
                "#....#..#\n",
            ),
            Reflection::Row(4),
        ),
    ];

    for (block, expected) in test_cases.iter() {
        let block = parse_line(block)?;
        let reflection = find_reflections(block.view(), Reflection::None);
        assert_eq!(reflection, *expected);
    }

    Ok(())
}

#[test]
fn test_2() -> Result<()> {
    let test_cases = [
        (
            concat!(
                "#.##..##.\n",
                "..#.##.#.\n",
                "##......#\n",
                "##......#\n",
                "..#.##.#.\n",
                "..##..##.\n",
                "#.#.##.#.\n",
            ),
            Reflection::Row(3),
        ),
        (
            concat!(
                "#...##..#\n",
                "#....#..#\n",
                "..##..###\n",
                "#####.##.\n",
                "#####.##.\n",
                "..##..###\n",
                "#....#..#\n",
            ),
            Reflection::Row(1),
        ),
    ];

    for (block, expected) in test_cases.iter() {
        let mut block = parse_line(block)?;
        let reflection = task2(block.view_mut());
        assert_eq!(reflection, *expected);
    }

    Ok(())
}
