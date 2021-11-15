// Input: strs = ["flower","flow","flight"]
// Output: "fl"

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

extern crate test;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // Its ok to unwrap because strs will never be empty
    let min = strs.iter().map(|x| x.len()).min().unwrap();

    // go through the length of the largest word in strs
    for i in (1..=min).rev() {
        // set the prefix by doing string pointer indexing magic
        let prefix = &strs[0][0..i];
        // iterate over the strings in strs to check if they comply to the prefix
        if strs.iter().all(|x| x.find(prefix) == Some(0)) {
            return prefix.to_owned();
        }
    }

    String::new()
}

pub fn longest_common_prefix2(strs: Vec<String>) -> String {
    let min = strs.iter().map(|x| x.len()).min().unwrap();
    for i in (1..min + 1).rev() {
        let prefix = &strs[0][0..i];
        if strs.iter().all(|x| x.find(prefix) == Some(0)) {
            return prefix.to_owned();
        }
    }
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_data() -> Vec<String> {
        vec![
            String::from("flower"),
            String::from("flow"),
            String::from("floight"),
        ]
    }

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(longest_common_prefix(get_data()), "flo");
    }

    #[bench]
    fn bench_longest_common_prefix(b: &mut Bencher) {
        b.iter(|| longest_common_prefix(get_data()));
    }

    #[bench]
    fn bench_longest_common_prefix2(b: &mut Bencher) {
        b.iter(|| longest_common_prefix2(get_data()));
    }
}
