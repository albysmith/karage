#![allow(clippy::non_ascii_literal)]

use gloo_timers::future::TimeoutFuture;
use seed::{prelude::*, *};


// HOW THIS SHIT WORKS
//  - view function = render html on screen
//  - update function = change shit and re-render the page
//      - orders have a few options that you can use in the update function
//          .skip() tells it not to re-render the page this time
//          .send_msg(Msg::ENUMOPTION) tells it to send this other message type from the enum options
//          .perform_cmd(FUNCTIONNAME(PARAMS)) tells it to run this function
//          there are more in the link I sent you for the orders docs
//  - Model struct = info that you might want to render, and flags to help you decide how to render it 
//      - you might not actually need to save the table data in the model


// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
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
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Greet,
    WriteHello,
    WriteName(String),
    WriteExclamationMarks,
    WriteEmoticon(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
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
        }
        Msg::WriteHello => model.title.push_str("Hello "),
        Msg::WriteName(name) => model.title.push_str(&name),
        Msg::WriteExclamationMarks => model.title.push_str("!!! "),
        Msg::WriteEmoticon(emoticon) => model.title.push_str(&emoticon),
    }
}

async fn write_exclamation_marks_after_delay() -> Msg {
    TimeoutFuture::new(1_000).await;
    Msg::WriteExclamationMarks
}

async fn write_emoticon_after_delay(emoticon: String) -> Msg {
    TimeoutFuture::new(2_000).await;
    Msg::WriteEmoticon(emoticon)
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    div![
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
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view).build_and_start();
}
