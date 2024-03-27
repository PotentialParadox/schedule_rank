mod rank;

fn main() {
    let mut residents = rank::read_csv().unwrap();
    rank::optimize(&mut residents);
    rank::assign_tracks(&mut residents);
    rank::print_residents(&mut residents);
}
 