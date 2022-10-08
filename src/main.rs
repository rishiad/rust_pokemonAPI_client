#![warn(clippy::all, rust_2018_idioms)]
mod ui;
mod logic;
use eframe::run_native;
use logic::make_req;
fn main() {
    // // Log to stdout (if you run with `RUST_LOG=debug`).
    // let data = logic::api::make_req();
    // println!("{:#?}",data);
    let native_options = eframe::NativeOptions::default();
    let data = make_req();
    // builds window 
    run_native("Pokedex", 
                native_options, 
                Box::new(|_cc| Box::new(ui::AppData {data}))
            )
}
