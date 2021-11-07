use testing::roman_to_int;
// Symbol       Value
// I             1
// V             5
// X             10 L             50
// C             100
// D             500
// M             1000

fn main() {
    let res = roman_to_int(String::from("MCMXXXII")); // 1932
    println!("{:#?}", res);
}
