use crate::card_query::CardsBySet;
use std::io::Write;
use std::{fs::File, path::Path};

use headless_chrome::{protocol::cdp::Page, Browser};

pub mod format;
pub mod order;

fn gen_outfile_name(in_name: &str, ext: &str) -> String {
    format!(
        "{}-by-set.{}",
        Path::new(in_name).file_stem().unwrap().to_str().unwrap(),
        ext
    )
}

pub fn render_to_string<P: order::SetInfoOrder>(
    cards_by_set: CardsBySet,
    formatter: Box<dyn format::OutputFormat>,
    order: P,
) -> anyhow::Result<String> {
    let rendered = formatter.render(&order.sort(cards_by_set));
    Ok(rendered)
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

pub fn render_pdf() -> anyhow::Result<()> {
    println!("open deafult brqwoser");
    let browser = Browser::default()?;
    println!("browser");
    let tab = browser.new_tab()?;
    println!("tab");
    let html = r##"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Hello World</title>
    </head>
    <body>
        <h1>Hello World!</h1>
    </body>
    </html>
    "##;
    println!("html");

    tab.navigate_to(format!("data:text/html;charset=utf-8,{}", html).as_str())?;

    println!("navigate");
    let pdf = tab.print_to_pdf(None)?;
    println!("print");
    Ok(std::fs::write("test.pdf", &pdf)?)
    // let mut outfile = File::create("test.pdf")?;
    // Ok(outfile.write_all(&pdf)?)
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
