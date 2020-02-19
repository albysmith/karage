// #![allow(clippy::non_ascii_literal)]

use gloo_timers::future::TimeoutFuture;
use seed::{prelude::*, *};

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
    table: String,
    index: usize,
    index_step: usize,
    toml_parsed: Toml,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    LoadToml,
    Forward,
    Back,
    KeyDown(web_sys::KeyboardEvent)
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
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
        }
    }
}

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
}
// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    div![
        input![ keyboard_ev("keydown", Msg::KeyDown)],
        div![
            button!["Load Data", ev(Ev::Click, |_| Msg::LoadToml)],
            button!["Forward", ev(Ev::Click, |_| Msg::Forward)],
            button!["Back", ev(Ev::Click, |_| Msg::Back)],
        ],
        div![Node::from_html(&model.table)] //
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view).build_and_start();
}
