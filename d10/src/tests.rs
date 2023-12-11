use crate::count_tiles_inside;
use crate::find_start;
use crate::get_path;
use anyhow::Result;

#[test]
fn test_easy() -> Result<()> {
    let field = concat!(
        "...........\n",
        ".S-------7.\n",
        ".|F-----7|.\n",
        ".||.....||.\n",
        ".||.....||.\n",
        ".|L-7.F-J|.\n",
        ".|..|.|..|.\n",
        ".L--J.L--J.\n",
        "...........\n"
    );
    let field: Vec<Vec<char>> = field.lines().map(|l| l.chars().collect()).collect();
    let start = find_start(&field)?;
    let path = get_path(&field, start)?;
    assert_eq!(count_tiles_inside(&field, &path), 4);
    Ok(())
}

#[test]
fn test_hard() -> Result<()> {
    let field = concat!(
        ".F----7F7F7F7F-7....\n",
        ".|F--7||||||||FJ....\n",
        ".||.FJ||||||||L7....\n",
        "FJL7L7LJLJ||LJ.L-7..\n",
        "L--J.L7...LJS7F-7L7.\n",
        "....F-J..F7FJ|L7L7L7\n",
        "....L7.F7||L7|.L7L7|\n",
        ".....|FJLJ|FJ|F7|.LJ\n",
        "....FJL-7.||.||||...\n",
        "....L---J.LJ.LJLJ...\n",
    );
    let field: Vec<Vec<char>> = field.lines().map(|l| l.chars().collect()).collect();
    let start = find_start(&field)?;
    let path = get_path(&field, start)?;
    assert_eq!(count_tiles_inside(&field, &path), 8);
    Ok(())
}
