#![cfg(test)]

use super::*;

#[test]
fn test1() -> Result<()> {
    let s = concat!(
        "O....#....\n",
        "O.OO#....#\n",
        ".....##...\n",
        "OO.#O....O\n",
        ".O.....O#.\n",
        "O.#..O.#.#\n",
        "..O..#O..O\n",
        ".......O..\n",
        "#....###..\n",
        "#OO..#....\n",
    );
    let block = parse_block(s)?;
    let ans1 = task1(block);

    assert_eq!(ans1, 136);

    Ok(())
}

#[test]
fn test2() -> Result<()> {
    let s = concat!(
        "O....#....\n",
        "O.OO#....#\n",
        ".....##...\n",
        "OO.#O....O\n",
        ".O.....O#.\n",
        "O.#..O.#.#\n",
        "..O..#O..O\n",
        ".......O..\n",
        "#....###..\n",
        "#OO..#....\n",
    );

    let block = parse_block(s)?;
    let ans2 = task2(block, 100000);
    // let ans2 = task2(block, 3);

    assert_eq!(ans2, 64);

    Ok(())
}

#[test]
fn test_rot90() {
    let mut arr = Array2::from_shape_vec((3, 3), "123456789".chars().collect()).unwrap();
    rot90(&mut arr);
    assert_eq!(
        arr,
        Array2::from_shape_vec((3, 3), "741852963".chars().collect()).unwrap()
    );
}
