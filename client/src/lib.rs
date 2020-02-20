// #![allow(clippy::non_ascii_literal)]

use gloo_timers::future::TimeoutFuture;
use seed::{prelude::*, *};
// use web_sys::MouseScrollEvent;
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

/*
Menu -> Table | Blog -> Menu
    - refresh -> Menu


*/

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    table: String,
    index: usize,
    old_index: usize,
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
    KeyDown(web_sys::KeyboardEvent),
    Scroll,
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::LoadToml => {
            model.index_step = 200;
            model.index = 0;
            let test_data = include_str!("test_data/print_universe.txt");
            let toml_parsed: Toml = toml::from_str(&test_data).expect("parsing toml");
            model.toml_parsed = toml_parsed;
            increment_index(model, true);
            let table = draw_table(&model.toml_parsed, model.index, model.old_index);
            model.table.clear();
            model.table = table;
            log!("LoadToml")
        }
        Msg::Forward => {
            increment_index(model, true);
            let table = draw_table(&model.toml_parsed, model.index, model.old_index);
            model.table.clear();
            model.table = table;
            log!("ForwardTable");
        }
        Msg::Back => {
            increment_index(model, false);
            let table = draw_table(&model.toml_parsed, model.index, model.old_index);
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
            increment_index(model, false);
            let table = draw_table(&model.toml_parsed, model.index, model.old_index);
            model.table.clear();
            model.table = table;
            log!("ForwardTable");
        }
        Msg::Scroll => {
            increment_index(model, true);
            let table = draw_table(&model.toml_parsed, model.index, model.old_index);
            model.table.clear();
            model.table = table;
            log!("Scroll");
        }
    }
}

fn increment_index(model: &mut Model, add_or_subtract: bool) {
    model.old_index = model.index;
    if add_or_subtract {
        model.index += model.index_step
    } else if model.index >= model.index_step {
        model.index -= model.index_step
    }
}

fn draw_table(toml_parsed: &Toml, index: usize, old_index: usize) -> String {
    if let Some(_ship) = &toml_parsed.Ships {
        toml_parsed
            .clone()
            .Ships
            .unwrap()
            .sort_by_key(|a| a.owner.clone().unwrap());
    }
    let mut rows = "".to_string();
    for (i, ship) in toml_parsed.Ships.as_ref().unwrap().iter().enumerate() {
        if i >= old_index && i <= index {
            rows.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                &ship._id.as_ref().unwrap(),
                &ship.owner.as_ref().unwrap(),
                &ship.job.as_ref().unwrap()
            ))
        } else if i > index {
            break;
        }
    }
    let table = format!("<table id=\"table\" class=\"littletables\"> <thead>  <tr><th>The table header</th></tr> </thead> <tbody> {} </tbody></table>",
                rows );
    table
}
// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    div![
        input![keyboard_ev("keydown", Msg::KeyDown)],
        div![
            button!["Load Data", ev(Ev::Click, |_| Msg::LoadToml)],
            button!["Forward", ev(Ev::Click, |_| Msg::Forward)],
            button!["Back", ev(Ev::Click, |_| Msg::Back)],
        ],
        div![
            ev(Ev::Scroll, |_| Msg::Scroll),
            Node::from_html(&model.table)
        ] //
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::builder(update, view).build_and_start();
}
