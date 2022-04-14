pub mod verse;
pub mod stringinitials;
pub mod anime;

fn main() {
    //println!("{:?}", verse::getvinitials());
    let Anime = anime::init().unwrap();
    //println!("{:?}", anime::get(Anime));
    println!("{:?}", anime::get(Anime));
}
