use crate::MyApp;
use crate::ViewState;
use crate::database;
use eframe::egui;
use rfd::FileDialog;

impl MyApp {
    pub fn header(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Database", |ui| {
            if ui.button("Create New Database").clicked() {
                self.show_new_db_viewport = true;
                ui.close();
            }

            if ui.button("Open Database").clicked() {
                self.file_path = FileDialog::new()
                    .add_filter("database", &["enc"])
                    .set_directory("/")
                    .pick_file();
                self.pending_open = true;
                ui.close();
            }

            if ui.button("Quit").clicked() {
                ui.close();
            }
        });
        ui.menu_button("Entries", |ui| {
            if ui.button("New Entry").clicked() {
                self.show_new_db_viewport = true;
                ui.close();
            }
        });
    }

    pub fn show_new_db_viewport_ui(&mut self, ui: &mut egui::Ui) {
        if self.show_new_db_viewport {
            ui.ctx().show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Create new database")
                    .with_inner_size([720.0, 320.0]),
                |ui, class| {
                    egui::CentralPanel::default().show_inside(ui, |ui| {
                        if (self.new_db_page == 1) {
                            ui.label("Password");
                            ui.text_edit_singleline(&mut self.new_db_password);
                            ui.label("Confirm Password");
                            ui.text_edit_singleline(&mut self.new_db_confirm_password);
                            ui.horizontal(|ui| {
                                if ui.button("Go Back").clicked() {
                                    self.new_db_page -= 1;
                                }
                                if (ui.button("Done").clicked()
                                    && self.new_db_password == self.new_db_confirm_password)
                                {
                                    self.file_path = FileDialog::new()
                                        .set_file_name(&format!("{}.enc", self.new_db_name))
                                        .save_file();
                                    self.pending_create = true;
                                }
                            });
                        } else {
                            //Page 0
                            ui.heading("Database Information");
                            ui.label("Database Name ");
                            ui.text_edit_singleline(&mut self.new_db_name);
                            ui.horizontal(|ui| {
                                if (ui.button("Cancel").clicked()) {
                                    self.show_new_db_viewport = false;
                                }
                                if (ui.button("Continue").clicked()) {
                                    self.new_db_page += 1;
                                }
                            });
                        }

                        if ui.input(|i| i.viewport().close_requested()) {
                            self.show_new_db_viewport = false;
                        }
                    });
                },
            );
        }
    }
    pub fn reset_new_db_state(&mut self) {
        self.pending_create = false;
        self.file_path = None;

        //REMOVE VIEWPORT AND RESET STATE
        self.show_new_db_viewport = false;
        self.new_db_name = String::new();
        self.new_db_page = 0;
        self.new_db_password = String::new();
        self.new_db_confirm_password = String::new();
    }
    pub fn start_menu(&mut self, ui: &mut egui::Ui) {
        //draw_unselected_db();
        ui.horizontal(|ui| {
            ui.heading("Rustpass");
        });
        ui.horizontal(|ui| {
            if ui.button("Create New Database").clicked() {
                self.show_new_db_viewport = true;
            }
            if ui.button("Open Database").clicked() {
                //open_database
                self.file_path = FileDialog::new()
                    .add_filter("database", &["enc"])
                    .set_directory("/")
                    .pick_file();
                self.pending_open = true;
            }
        });
    }

    pub fn unlock_db(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Your password: ");
            ui.text_edit_singleline(&mut self.password);
        });
        if ui.button("Enter").clicked() {
            println!("{}", self.password);
            // take password, generate key, compare key to hash, if true store key,
            // else give error message
            match database::unlock_db(&self.password, &self.file_path) {
                Ok(key) => {
                    self.key = key;
                    self.db_selected = false;
                    self.unlocked_db = true;
                    self.viewstate = ViewState::DatabaseStartMenu;
                    println!("unlocked");
                }
                Err(e) => println!("{}", e),
            }
        }
    }

    pub fn unlocked_db(&mut self, ui: &mut egui::Ui) {
        if (self.unlocked_db_page == 0) {}
    }
    pub fn new_entry(&mut self, ui: &mut egui::Ui) {
        
    }
}
