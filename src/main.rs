mod archivers;

use archivers::*;

fn main() {
    let letterboxd_archiver = archivers::letterboxd::LetterboxdArchiver {};
    println!("{}", letterboxd_archiver.get_name());

    letterboxd_archiver.get_data();
}
