#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, App};
use std::{net::TcpStream, str::FromStr};
mod client;
use client::{read_response, write_request};

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(AppState::default())),
    )
}

struct AppState {
    name: String,
    addr: &'static str,
    connection: Option<TcpStream>,
    output_contents: String,
    streaming: bool,
    age: u32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            addr: Servers::FIRST,
            connection: if let Ok(con) = TcpStream::connect(Servers::FIRST) {
                Some(con)
            } else {
                None
            },
            output_contents: String::new(),
            streaming: false,
            age: 42,
        }
    }
}

#[derive(PartialEq)]
struct Servers {}

impl Servers {
    pub const FIRST: &str = "localhost:7878";
    pub const SECOND: &str = "localhost:8787";
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let Self {
                name,
                addr,
                connection,
                output_contents,
                streaming,
                age,
            } = self;
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.horizontal(|ui| {
                let first = ui.selectable_value(addr, Servers::FIRST, "Server 1");
                let second = ui.selectable_value(addr, Servers::SECOND, "Server 2");

                if first.changed() {
                    *addr = "localhost:7878";
                    *output_contents = String::from_str(addr).unwrap();
                    output_contents.push('\n');
                    match TcpStream::connect(*addr) {
                        Ok(stream) => *connection = Some(stream),
                        Err(_) => {
                            output_contents.push_str("\nFailed connecting. Maybe server offline?");
                            *connection = None
                        }
                    };
                }
                if second.changed() {
                    *addr = "localhost:8787";
                    *output_contents = String::from_str(addr).unwrap();
                    output_contents.push('\n');
                    match TcpStream::connect(*addr) {
                        Ok(stream) => *connection = Some(stream),
                        Err(_) => {
                            output_contents.push_str("\nFailed connecting. Maybe server offline?");
                            *connection = None
                        }
                    };
                }
            });
            ui.vertical_centered_justified(|ui| {
                egui::ScrollArea::new([false, true])
                    .always_show_scroll(true)
                    .max_height(220f32)
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        let text_edit = ui.add(
                            egui::widgets::text_edit::TextEdit::multiline(output_contents)
                                .interactive(false),
                        );
                        if text_edit.changed() {};
                    });
                if let Some(connection) = connection {
                    let button_single = ui.button("Request single");
                    if button_single.clicked() && !*streaming {
                        write_request(b"get_once", connection);
                        output_contents.push_str(&read_response(connection));
                    }
                    if ui.button("Request reactive").clicked() {
                        if !*streaming {
                            write_request(b"get_stream", connection);
                            *streaming = true;
                        } else {
                            *streaming = false;
                            *connection = TcpStream::connect(*addr).unwrap();
                        }
                    }
                    if *streaming {
                        output_contents.push_str(&read_response(connection));
                    }
                }
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
