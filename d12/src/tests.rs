#[cfg(test)]
use super::*;

#[test]
fn test_count_arrangemets1() {
    let test_cases = [
        ("???.###", vec![1, 1, 3], 1),
        (".??..??...?##.", vec![1, 1, 3], 4),
        ("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6], 1),
        ("????.#...#...", vec![4, 1, 1], 1),
        ("????.######..#####.", vec![1, 6, 5], 4),
        ("?###????????", vec![3, 2, 1], 10),
    ];

    for (symbols, nums, expected) in test_cases {
        let mut symbols = symbols.to_owned().into_bytes();
        let actual = count_arrangemets(&mut symbols, &nums);
        let s = String::from_utf8(symbols).unwrap();
        assert_eq!(actual, expected, "symbols: {}, nums: {:?}", s, nums);
    }
}

#[test]
fn test_expand() {
    let test_cases = [(".#", vec![1], ".#?.#?.#?.#?.#", vec![1, 1, 1, 1, 1])];
    // let test_cases = [(".#", vec![1], ".#!.#!.#!.#!.#", vec![1, 1, 1, 1, 1])];

    for (symbols, nums, expected_symbols, expected_nums) in test_cases {
        let symbols = symbols.to_owned().into_bytes();
        let (symbols, nums) = expand(&symbols, &nums);
        let s = String::from_utf8(symbols.clone()).unwrap();
        assert_eq!(
            symbols,
            expected_symbols.as_bytes(),
            "symbols: {s}, expected_symbols: {expected_symbols}"
        );
        assert_eq!(nums, expected_nums, "nums: {:?}", nums);
    }
}

// #[ignore]
#[test]
fn test_count_arrangemets2() {
    let test_cases = [
        ("???.###", vec![1, 1, 3], 1),
        (".??..??...?##.", vec![1, 1, 3], 16384),
        ("?#?#?#?#?#?#?#?", vec![1, 3, 1, 6], 1),
        ("????.#...#...", vec![4, 1, 1], 16),
        ("????.######..#####.", vec![1, 6, 5], 2500),
        ("?###????????", vec![3, 2, 1], 506250),
    ];

    for (symbols, nums, expected) in test_cases {
        let symbols = symbols.to_owned().into_bytes();
        let (mut symbols, nums) = expand(&symbols, &nums);
        let actual = count_arrangemets(&mut symbols, &nums);
        let s = String::from_utf8(symbols).unwrap();
        assert_eq!(actual, expected, "symbols: {}, nums: {:?}", s, nums);
    }
}
