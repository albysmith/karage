use actix::prelude::*;
use actix_files::{Files, NamedFile};
use actix_multipart::{Multipart, MultipartError};
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpRequest, HttpServer};
use env_logger::init;

use std::fmt::Write;
use std::time;
use tokio_timer;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_envlogger;
extern crate slog_scope;
extern crate slog_stdlog;
extern crate slog_term;
#[macro_use]
extern crate log;

use slog::Drain;
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    let system = System::new("gundamu");

    // SETTINGS FOR ENV LOGGER
    // std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_LOG", "info");

    // SETTING UP LOG FILE NAME (INCOMPLETE - ADD DATE/TIME)
    // let time =
    // let log_path = &format!("log/output.log");
    let log_path = "log/output.log";
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(log_path)
        .unwrap();

    // create logger
    let decorator = slog_term::PlainSyncDecorator::new(file);
    let drain = slog_async::Async::default(slog_envlogger::new(
        slog_term::FullFormat::new(decorator).build().fuse(),
    ));
    let logger = slog::Logger::root(drain.fuse(), o!());

    // slog_stdlog uses the logger from slog_scope, so set a logger there
    let _guard = slog_scope::set_global_logger(logger);

    // register slog_stdlog as the log handler with the log crate
    slog_stdlog::init().unwrap();

    // create and run actix server
    HttpServer::new(move || {
        App::new()
            // default logger settings
            // .wrap(Logger::default())
            // this is how you set custom logs (see actix middleware for % notation)
            .wrap(Logger::new(
                "server request: %a %t \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T",
            ))
            .service(Files::new("/public", "./client/public"))
            .service(Files::new("/pkg", "./client/pkg"))
            .default_service(web::get().to(|req: HttpRequest| {
                println!("app start {:?}", req);
                NamedFile::open("./client/index.html")
            }))
    })
    .bind("127.0.0.1:8000")?
    .run()?;

    system.run()
}

// UNUSED DUE TO UNKNOWN SCOPE ERROR - adding this to main function solved issues
// fn logs() {
//     let log_path = "log/output.log";
//     let file = OpenOptions::new()
//         .create(true)
//         .write(true)
//         .truncate(true)
//         .open(log_path)
//         .unwrap();

//     // create logger
//     let decorator = slog_term::PlainSyncDecorator::new(file);
//     let drain = slog_term::FullFormat::new(decorator).build().fuse();
//     let logger = slog::Logger::root(drain, o!());

//     // slog_stdlog uses the logger from slog_scope, so set a logger there
//     let _guard = slog_scope::set_global_logger(logger);

//     // register slog_stdlog as the log handler with the log crate
//     slog_stdlog::init().unwrap();

//     info!("global file logger");
// }
