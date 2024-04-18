use scrollrack_core::{
    card_query::CardQuery, output::write_to_file, parse,
    scryfall_card_wrapper::ScryfallCardWrapper, setinfo::SetInfo,
};
use tera::{Context, Tera};
use wkhtmltopdf::{Size, Orientation, PdfApplication};

// lazy_static! {
//     pub static ref TEMPLATES: Tera = {
//         let mut tera = match Tera::new("html_template.html") {
//             Ok(t) => t,
//             Err(e) => {
//                 println!("Parsing error(s): {}", e);
//                 ::std::process::exit(1);
//             }
//         };
//         tera.autoescape_on(vec![".html", ".sql"]);
//         tera.register_filter("do_nothing", do_nothing_filter);
//         tera
//     };
// }
//
#[derive(serde::Serialize)]
struct Entry {
    set_name: String,
    set_code: String,
    cards: Vec<ScryfallCardWrapper>,
}

impl From<(&SetInfo, &Vec<ScryfallCardWrapper>)> for Entry {
    fn from(value: (&SetInfo, &Vec<ScryfallCardWrapper>)) -> Self {
        Entry {
            set_name: value.0.set_name().to_string(),
            set_code: value.1[0].set_code().to_string(),
            cards: value.1.to_owned(),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = "docs/affinity.txt";
    let lines = parse::read_lines(path)?;

    let cards_by_set = CardQuery::default()
        .run(parse::parse_card_infos(lines))
        .await;

    let tera = match Tera::new("templates/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let entries: Vec<Entry> = cards_by_set
        .iter()
        .map(|(key, val)| Into::into((key, val)))
        .collect();

    let mut context = Context::new();
    context.insert("title", "Cards sorted by set");
    context.insert("sets", &entries);

    let render = tera.render("html_template.html", &context)?;

    let mut pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app
        .builder()
        .orientation(Orientation::Portrait)
        .margin(Size::Inches(2))
        .title("Awesome Foo")
        .build_from_html(render)
        .expect("failed to build pdf");

    pdfout.save("foo.pdf").expect("failed to save foo.pdf");
    println!("generated PDF saved as: foo.pdf");
    Ok(())
}
