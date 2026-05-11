#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![expect(rustdoc::missing_crate_level_docs)]

use chacha20poly1305::Key;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use eframe::egui;
use egui_file_dialog::FileDialog;
use std::path::PathBuf;

mod database;
mod gui_handler;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 480.0]),
        ..Default::default()
    };

    // let cipher = ChaCha20Poly1305::new(&key);
    // let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; create new for every cipher text
    // let ciphertext = cipher
    //     .encrypt(&nonce, b"plaintext message".as_ref())
    //     .expect("Encryption failed");

    // let plaintext = cipher
    //     .decrypt(&nonce, ciphertext.as_ref())
    //     .expect("Decryption failed");

    // println!(
    //     "Decrypted plaintext: {}",
    //     String::from_utf8_lossy(&plaintext)
    // );
    // assert_eq!(&plaintext, b"plaintext message");
    eframe::run_native(
        "Rustpass",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

#[derive(Default)]
struct MyApp {
    show_new_db_viewport: bool,
    db_selected: bool,
    open_db_selected: bool,
    new_db_name: String,
    new_db_page: i32,
    new_db_password: String,
    new_db_confirm_password: String,
    password: String,
    key: Key,
    file_dialog: FileDialog,
    file_path: Option<PathBuf>,
    pending_create: bool,
    pending_open: bool,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            show_new_db_viewport: false,
            db_selected: false,
            open_db_selected: false,
            new_db_name: String::new(),
            new_db_page: 0,
            new_db_password: String::new(),
            new_db_confirm_password: String::new(),
            password: String::new(),
            key: Key::from([0u8; 32]),
            file_dialog: FileDialog::new(),
            file_path: None,
            pending_create: false,
            pending_open: false,
        }
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.start_menu(ui);

            self.show_new_db_viewport_ui(ui); //automatically handles itself based on state

            if let Some(path) = self.file_dialog.update(ui).picked() {
                self.file_path = Some(path.to_path_buf());
            }
            if self.pending_create {
                if let Some(path) = &self.file_path {
                    self.key = database::create_db(&self.new_db_name, &self.new_db_password, path);
                    self.reset_new_db_state();
                    self.file_path = None;
                }
            } else if self.pending_open {
                if let Some(path) = &self.file_path {
                    self.db_selected = true;
                    self.pending_open = false;
                }
            }
        });
    }
}
