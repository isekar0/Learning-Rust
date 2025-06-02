pub fn print_statements() {
    println!(
        "Max of i32 is {size_1}, max of i64 is {size_2}, and max of i128 is {size_3}",
        size_1 = i32::MAX,
        size_2 = i64::MAX,
        size_3 = i128::MAX
    );

    println!("Emojies :3 \u{1F600}")
}
