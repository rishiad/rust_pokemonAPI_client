use eframe::App;
use egui::{CentralPanel};

use crate::logic::{api::Data};


pub struct AppData {
    pub data : Result<Data,  Box<dyn std::error::Error>>
}

impl App for AppData {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
      
        CentralPanel::default().show(ctx, |ui| {
            
            match &self.data {
                Ok(n)  => ui.label(&n.name),
                Err(e) => panic!("Problem opening the file: {:?}", e)
            }

            
        });
    }
}