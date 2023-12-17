#![cfg(test)]
use super::*;

impl Step {
    pub fn new_add(label: &str, foc_l: u32) -> Self {
        Self::Add {
            label: label.to_string(),
            foc_l,
        }
    }
    pub fn new_remove(label: &str) -> Self {
        Self::Remove {
            label: label.to_string(),
        }
    }
}

#[test]
fn test_hash() {
    let s = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(task1(s), 1320);
}

#[test]
fn test_parse_step() -> Result<()> {
    let s = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let ans = vec![
        Step::new_add("rn", 1),
        Step::new_remove("cm"),
        Step::new_add("qp", 3),
        Step::new_add("cm", 2),
        Step::new_remove("qp"),
        Step::new_add("pc", 4),
        Step::new_add("ot", 9),
        Step::new_add("ab", 5),
        Step::new_remove("pc"),
        Step::new_add("pc", 6),
        Step::new_add("ot", 7),
    ];
    let s_ = s
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<Step>>>()?;
    assert_eq!(s_, ans);
    Ok(())
}

#[test]
fn test_task2() {
    let s = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(task2(s), 145);
}
