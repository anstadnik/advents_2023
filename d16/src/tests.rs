#[cfg(test)]
use super::*;

#[test]
fn test_task_1() {
    let s = concat!(
        ".|...\\....\n",
        "|.-.\\.....\n",
        ".....|-...\n",
        "........|.\n",
        "..........\n",
        ".........\\\n",
        "..../.\\\\..\n",
        ".-.-/..|..\n",
        ".|....-|.\\\n",
        "..//.|....\n",
    );
    let field = gen_field(s);

    assert_eq!(task_1(&field, (0, 0), Direction::Right), 46);
}

#[test]
fn test_task_2() {
    let s = concat!(
        ".|...\\....\n",
        "|.-.\\.....\n",
        ".....|-...\n",
        "........|.\n",
        "..........\n",
        ".........\\\n",
        "..../.\\\\..\n",
        ".-.-/..|..\n",
        ".|....-|.\\\n",
        "..//.|....\n",
    );
    let field = gen_field(s);

    assert_eq!(task_2(&field), 51);
}
