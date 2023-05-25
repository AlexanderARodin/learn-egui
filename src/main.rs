#![allow(non_snake_case)]
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    println!("MAIN has beed entered..");

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(200., 300.)),
        ..Default::default()
    };
    eframe::run_native(
        "some text 1",
        options,
        Box::new( |_cc| Box::<RaaDataModel>::default()),
   )
}

// ------------ //
// -- Models -- //
// ------------ //

struct RaaDataModel {
    txt:String,
}
impl Default for RaaDataModel {
    fn default() -> Self {
        Self {txt:"<empty>".to_owned(),}
    }
}

impl eframe::App for RaaDataModel {
    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {
        egui::CentralPanel::default().show( ctx, |ui| {
            ui.label("lbl - A");
            ui.label("lbl - B");
            ui.label("lbl - C");
            ui.horizontal( |ui| {
                ui.label("lbl - 1");
                ui.label("lbl - 2");
                ui.label("lbl - 3");
            });
            let aLbl = ui.label( format!("CONTAINER: [{}]", self.txt) );
            ui.text_edit_singleline(&mut self.txt)
                .labelled_by(aLbl.id);
        });
    }
}


