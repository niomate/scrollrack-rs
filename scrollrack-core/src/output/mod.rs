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

pub fn render_to_file<P: order::SetInfoOrder>(
    path: String,
    cards_by_set: CardsBySet,
    formatter: Box<dyn format::OutputFormat>,
    order: P,
) -> anyhow::Result<()> {
    let rendered = formatter.render(&order.sort(cards_by_set));
    let mut outfile = File::create(gen_outfile_name(&path, &formatter.get_file_extension()))?;
    Ok(outfile.write_all(rendered.as_bytes())?)
}

// pub fn render<P: order::SetInfoOrder>(
//     cards_by_set: CardsBySet,
//     formatter: Box<dyn format::OutputFormat>,
//     order: P,
// ) -> &[u8] {
//     formatter.render(&order.sort(cards_by_set))
// }

// pub fn write_to_file(data: &[], path: &str) -> anyhow::Result<()> {
//     let mut outfile = File::create(path)?;
//     Ok(outfile.write_all(data)?)
// }
