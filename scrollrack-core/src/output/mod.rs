use crate::card_query::CardsBySet;
use std::io::Write;
use std::{fs::File, path::Path};

pub mod format;
pub mod order;

fn gen_outfile_name(in_name: &str, ext: &str) -> String {
    format!(
        "{}-by-set.{}",
        Path::new(in_name).file_stem().unwrap().to_str().unwrap(),
        ext
    )
}

pub fn render_to_string<P: order::SetInfoOrder, F: format::OutputFormat>(
    cards_by_set: CardsBySet,
) -> anyhow::Result<String> {
    Ok(F::render(&P::sort(cards_by_set)))
}

pub fn render_to_file<P: order::SetInfoOrder, F: format::OutputFormat>(
    path: String,
    cards_by_set: CardsBySet,
) -> anyhow::Result<()> {
    let rendered = F::render(&P::sort(cards_by_set));
    let mut outfile = File::create(gen_outfile_name(&path, &F::get_file_extension()))?;
    Ok(outfile.write_all(rendered.as_bytes())?)
}
