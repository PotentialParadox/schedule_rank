mod rank;

fn main() {
    let results = rank::read_csv().unwrap();
    println!("{:?}", results);
    println!("Hello, world!");
}
