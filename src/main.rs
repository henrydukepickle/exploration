mod game;
mod io;
mod read_kdl;
use io::io::get_data;
const PATH: &str = "data.kdl";

fn main() {
    get_data(String::from(PATH)).run();
}
