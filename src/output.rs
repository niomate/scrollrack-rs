use std::fs::File;
use std::path::Path;

fn gen_outfile_name(in_name: &str) -> String {
    format!(
        "{}-by-set.txt",
        Path::new(&in_name).file_stem().unwrap().to_str().unwrap()
    )
}

pub fn gen_outfile_from_infile(infile_path: &str) -> File {
    File::create(gen_outfile_name(infile_path)).expect("Could not open a new file")
}
