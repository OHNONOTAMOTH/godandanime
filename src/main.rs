pub mod verse;
pub mod stringinitials;

fn main() {
    println!("{:?}", stringinitials::parse(verse::get()));
}
