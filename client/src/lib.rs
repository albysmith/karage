<<<<<<< HEAD
#![allow(clippy::non_ascii_literal)]
=======
// #![allow(clippy::non_ascii_literal)]
>>>>>>> 7ebe872a30b409397f203e344e150bc2997e2e7a

use gloo_timers::future::TimeoutFuture;
use seed::{prelude::*, *};

<<<<<<< HEAD
=======
#[macro_use]
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

mod toml_parse;
use toml_parse::*;

// mod data_store;
// use data_store::*;
// mod build_table;
// use build_table::*;
>>>>>>> 7ebe872a30b409397f203e344e150bc2997e2e7a

// HOW THIS SHIT WORKS
//  - view function = render html on screen
//  - update function = change shit and re-render the page
//      - orders have a few options that you can use in the update function
//          .skip() tells it not to re-render the page this time
//          .send_msg(Msg::ENUMOPTION) tells it to send this other message type from the enum options
//          .perform_cmd(FUNCTIONNAME(PARAMS)) tells it to run this function
//          there are more in the link I sent you for the orders docs
<<<<<<< HEAD
//  - Model struct = info that you might want to render, and flags to help you decide how to render it 
//      - you might not actually need to save the table data in the model

=======
//  - Model struct = info that you might want to render, and flags to help you decide how to render it
//      - you might not actually need to save the table data in the model
>>>>>>> 7ebe872a30b409397f203e344e150bc2997e2e7a

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
<<<<<<< HEAD
    title: String,
    greet_clicked: bool,
    table: TableData,
}

#[derive(Default)]
struct TableData {
    headers: Vec<String>,
    entries: Vec<TableEntry>,
}

#[derive(Default)]
struct TableEntry {
    data: Vec<String>,
=======
    table: String,
    index: usize,
    index_step: usize,
    toml_parsed: Toml,
>>>>>>> 7ebe872a30b409397f203e344e150bc2997e2e7a
}

// ------ ------
//    Update
// ------ ------

enum Msg {
<<<<<<< HEAD
    Greet,
    WriteHello,
    WriteName(String),
    WriteExclamationMarks,
    WriteEmoticon(String),
=======
    LoadToml,
    Forward,
    Back,
    KeyDown(web_sys::KeyboardEvent)
>>>>>>> 7ebe872a30b409397f203e344e150bc2997e2e7a
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
<<<<<<< HEAD
        Msg::Greet => {
            model.greet_clicked = true;
            // this just adds something to the table headers vec so that something can display
            // see view function below; i'm not actually rendering a table into the html yet
            model.table.headers.push("World Header".into());
            orders
                .skip()
                .send_msg(Msg::WriteHello)
                .send_msg(Msg::WriteName("World".into()))
                .perform_cmd(write_exclamation_marks_after_delay())
                .perform_cmd(write_emoticon_after_delay("🙂".into()));
=======
        Msg::LoadToml => {
            model.index_step = 20;
            model.index = 0;
            let test_data = include_str!("test_data/print_universe.txt");
            let toml_parsed: Toml = toml::from_str(&test_data).expect("parsing toml");
            model.toml_parsed = toml_parsed;
            model.index += model.index_step;
            let table = draw_table(&model.toml_parsed, model.index);
            model.table.clear();
            model.table = table;
            log!("LoadToml")
        }
        Msg::Forward => {
            model.index += model.index_step;
            let table = draw_table(&model.toml_parsed, model.index);
            model.table.clear();
            model.table = table;
            log!("ForwardTable");
        }
        Msg::Back => {
            if model.index >= model.index_step {
                model.index -= model.index_step;
            }
            let table = draw_table(&model.toml_parsed, model.index);
            model.table.clear();
            model.table = table;
            log!("ForwardTable");
        }
        // Msg::Forward => {
        //     model.index += model.index_step;
        //     let table = draw_table(&model.toml_parsed, model.index);
        //     model.table.clear();
        //     model.table = table;
        //     log!("ForwardTable");
        // }
        Msg::KeyDown(_event) => {
            if model.index >= model.index_step {
                model.index += model.index_step;
            }
            let table = draw_table(&model.toml_parsed, model.index);
            model.table.clear();
            model.table = table;
            log!("ForwardTable");
>>>>>>> 7ebe872a30b409397f203e344e150bc2997e2e7a
        }
        Msg::WriteHello => model.title.push_str("Hello "),
        Msg::WriteName(name) => model.title.push_str(&name),
        Msg::WriteExclamationMarks => model.title.push_str("!!! "),
        Msg::WriteEmoticon(emoticon) => model.title.push_str(&emoticon),
    }
}

<<<<<<< HEAD
async fn write_exclamation_marks_after_delay() -> Msg {
    TimeoutFuture::new(1_000).await;
    Msg::WriteExclamationMarks
}

async fn write_emoticon_after_delay(emoticon: String) -> Msg {
    TimeoutFuture::new(2_000).await;
    Msg::WriteEmoticon(emoticon)
=======
fn draw_table(toml_parsed: &Toml, index_step: usize) -> String {
    if let Some(_ship) = &toml_parsed.Ships {
        toml_parsed
            .clone()
            .Ships
            .unwrap()
            .sort_by_key(|a| a.owner.clone().unwrap());
    }
    let mut rows = "".to_string();
    for (i, ship) in toml_parsed.Ships.as_ref().unwrap().iter().enumerate() {
        if i <= index_step {
            rows.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                &ship._id.as_ref().unwrap(),
                &ship.owner.as_ref().unwrap(),
                &ship.job.as_ref().unwrap()
            ))
        } else if i > index_step {
            break;
        }
    }
    let table = format!("<table id=\"table\"> <thead>  <tr><th>The table header</th></tr> </thead> <tbody> {} </tbody></table>",
                rows );
    table
>>>>>>> 7ebe872a30b409397f203e344e150bc2997e2e7a
}
// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    div![
<<<<<<< HEAD
        style![
            St::Display => "flex",
            St::JustifyContent => "center",
            St::AlignItems => "center",
            St::FontSize => vmin(5),
            St::FontFamily => "sans-serif",
            St::Height => vmin(50),
        ],
        div![
                style![
                    St::BackgroundColor => "lightgreen",
                    St::Padding => vmin(3),
                    St::BorderRadius => vmin(3),
                    St::Cursor => "pointer",
                    St::BoxShadow => [vmin(0), vmin(0.5), vmin(0.5), "green".into()].join(" "),
                ],
                // on click, send this message
                ev(Ev::Click, |_| Msg::Greet),
                "Greet!"
            ],
        if model.greet_clicked {
            // ENTER TABLE CREATION HERE
            h1![model.table.headers[0]]
        }
        else {
            h1!["No Data Here"]
        }
=======
        input![ keyboard_ev("keydown", Msg::KeyDown)],
        div![
            button!["Load Data", ev(Ev::Click, |_| Msg::LoadToml)],
            button!["Forward", ev(Ev::Click, |_| Msg::Forward)],
            button!["Back", ev(Ev::Click, |_| Msg::Back)],
        ],
        div![Node::from_html(&model.table)] //
>>>>>>> 7ebe872a30b409397f203e344e150bc2997e2e7a
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view).build_and_start();
}
