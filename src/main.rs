use testing::longest_common_prefix;

fn main() {
    let res = longest_common_prefix::longest_common_prefix(vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ]);
    println!("{:#?}", res);
}
