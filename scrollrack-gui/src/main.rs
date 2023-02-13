// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

use std::sync::Arc;

use druid::widget::{Button, Flex, TextBox};
use druid::{commands, Data, Lens};
use druid::{
    AppDelegate, AppLauncher, DelegateCtx, Env, FileDialogOptions, FileSpec, Handled, Target,
    Widget, WidgetExt, WindowDesc,
};
use scrollrack_core::card_query;
use scrollrack_core::output;
use scrollrack_core::parse;
use scrollrack_core::rules::prefilter;

struct Delegate;

#[derive(Clone, Data, Lens)]
struct AppData {
    input_str: Arc<String>,
    output_str: Arc<String>,
}

fn build_app() -> impl Widget<AppData> {
    let txt = FileSpec::new("Text file", &["txt"]);
    let other = FileSpec::new("Deck file", &["dck", "deck"]);
    // The options can also be generated at runtime,
    // so to show that off we create a String for the default save name.
    let default_save_name = String::from("MyFile.txt");
    let save_dialog_options = FileDialogOptions::new()
        .allowed_types(vec![txt, other])
        .default_type(txt)
        .default_name(default_save_name)
        .name_label("Target")
        .title("Output file name")
        .button_text("Export");
    let open_dialog_options = save_dialog_options
        .clone()
        .default_name("MySavedFile.txt")
        .name_label("Source")
        .title("Input file name")
        .button_text("Import");

    let save = Button::new("Save as...").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_SAVE_PANEL.with(save_dialog_options.clone()))
    });
    let open = Button::new("Import...").on_click(move |ctx, _, _| {
        ctx.submit_command(druid::commands::SHOW_OPEN_PANEL.with(open_dialog_options.clone()))
    });

    let sort = Button::new("Sort").on_click(move |_ctx, data: &mut AppData, _| {
        let card_infos = parse::parse_card_infos(data.input_str.lines());
        let cards_by_set = card_query::CardQuery::build()
            .with_prefilter(prefilter::PrefilterRule::NoPromo)
            .with_prefilter(prefilter::PrefilterRule::NoGiftBox)
            .with_prefilter(prefilter::PrefilterRule::IsPaper)
            .cards(card_infos)
            .done()
            .run();

        let out_string = output::output_string::<output::SortByName>(cards_by_set);
        data.output_str = out_string.into();
    });

    Flex::column()
        .with_flex_child(
            Flex::row()
                .main_axis_alignment(druid::widget::MainAxisAlignment::SpaceEvenly)
                .must_fill_main_axis(true)
                .with_flex_child(
                    TextBox::multiline()
                        .lens(AppData::input_str)
                        .expand_height()
                        .expand_width(),
                    0.5,
                )
                .with_default_spacer()
                .with_flex_child(
                    TextBox::multiline()
                        .lens(AppData::output_str)
                        .expand_height()
                        .expand_width(),
                    0.5,
                ),
            0.9,
        )
        .with_default_spacer()
        .with_flex_child(
            Flex::row()
                .with_child(open)
                .with_default_spacer()
                .with_child(sort)
                .with_default_spacer()
                .with_child(save),
            0.1,
        )
        .padding(8.0)

    // This method asks druid to draw colored rectangles around our widgets,
    // so we can visually inspect their layout rectangles.
    // col.debug_paint_layout()
}

pub fn main() {
    let window = WindowDesc::new(build_app).title("Scrollrack GUI");

    AppLauncher::with_window(window)
        .delegate(Delegate)
        .launch(AppData {
            input_str: "".to_string().into(),
            output_str: "".to_string().into(),
        })
        .expect("launch failed");
}

impl AppDelegate<AppData> for Delegate {
    fn event(
        &mut self,
        _ctx: &mut DelegateCtx,
        _window_id: druid::WindowId,
        event: druid::Event,
        _data: &mut AppData,
        _env: &Env,
    ) -> Option<druid::Event> {
        Some(event)
    }

    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &druid::Command,
        data: &mut AppData,
        _env: &Env,
    ) -> Handled {
        if let Some(file_info) = cmd.get(commands::SAVE_FILE_AS) {
            if let Err(e) = std::fs::write(file_info.path(), &data.output_str[..]) {
                println!("Error writing file: {}", e);
            }
            return Handled::Yes;
        }
        if let Some(file_info) = cmd.get(commands::OPEN_FILE) {
            match std::fs::read_to_string(file_info.path()) {
                Ok(s) => {
                    data.input_str = s.into();
                }
                Err(e) => {
                    println!("Error opening file: {}", e);
                }
            }
            return Handled::Yes;
        }
        Handled::No
    }

    fn window_added(
        &mut self,
        _id: druid::WindowId,
        _data: &mut AppData,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
    }

    fn window_removed(
        &mut self,
        _id: druid::WindowId,
        _data: &mut AppData,
        _env: &Env,
        _ctx: &mut DelegateCtx,
    ) {
    }
}
