// Input: strs = ["flower","flow","flight"]
// Output: "fl"

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res = String::new();

    if strs.len() == 1 {
        return res;
    }

    let mut min = strs[0].len();

    for word in strs.iter().skip(1) {
        if word.len() < min {
            min = word.len();
        }
    }

    println!("{}", min);

    res
}
