fn get_input() -> Vec<String> {
    include_str!("./inputs/day4.real")
        .trim()
        .split("\n\n")
        .map(|lines| lines.trim().replace('\n', " "))
        .collect()
}
pub fn solve_one() {
    let input = get_input();

    let mut valid_passport_count = 0;
    let attributes = vec![
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /* , "cid" */
    ];

    'outer: for passport_string in input {
        let passport_attrs: Vec<_> = passport_string
            .split(' ')
            .flat_map(|attr_field| attr_field.split_once(':'))
            .map(|attr_field| attr_field.0)
            .collect();

        for attr in attributes.iter() {
            if !passport_attrs.contains(attr) {
                continue 'outer;
            }
        }
        valid_passport_count += 1;
    }

    println!("{:?}", valid_passport_count);
}
pub fn solve_two() {}
