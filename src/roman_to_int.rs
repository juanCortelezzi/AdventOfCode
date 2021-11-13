extern crate test;

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000

/// roman_to_int_func transforms roman numbers to decimals
///
/// This is achieved by mapping over the array and changing the characters for its corresponding
/// value. Then reversing it and folding it to check if a "roman character" is substracting or not
/// to the accumulated value.
///
/// ```
/// use testing::roman_to_int::roman_to_int_func;
///
/// let result = roman_to_int_func(String::from("MCMXXXII"));
/// assert_eq!(result, 1932);
/// ```
pub fn roman_to_int_func(s: String) -> i32 {
    s.to_uppercase()
        .chars()
        .map(|char| match char {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("not a valid roman number string"),
        })
        .rev()
        .fold((0, 0), |(acc, prev), curr| {
            if prev <= curr {
                (acc + curr, curr)
            } else {
                (acc - curr, curr)
            }
        })
        .0
}

pub fn roman_to_int(s: String) -> i32 {
    let chars: Vec<_> = s
        .to_uppercase()
        .chars()
        .map(|char| match char {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => panic!("not a valid roman number string"),
        })
        .rev()
        .collect();

    let mut acc = 0;
    let mut prev = 0;

    for x in chars {
        if prev <= x {
            acc += x;
        } else {
            acc -= x;
        }
        prev = x
    }

    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int(String::from("MCMXXXII")), 1932);
    }

    #[bench]
    fn bench_roman_to_int(b: &mut Bencher) {
        b.iter(|| roman_to_int(String::from("MCMXXXII")));
    }

    #[bench]
    fn bench_roman_to_int_func(b: &mut Bencher) {
        b.iter(|| roman_to_int_func(String::from("MCMXXXII")));
    }
}
