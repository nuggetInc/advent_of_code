mod year2023;
fn main() {
    let mut year = year2023::year2023();
    let result = year.run();

    println!("{result}");
}
