use std::thread;

use eframe::egui::{self, include_image, text::LayoutJob, Button, Color32, Image, Label, Layout, RichText, ScrollArea, TextFormat, Vec2};

use egui_notify::Toasts;

use crate::{constants::{get_install_result, set_install_result, LICENSE}, funcs};

pub struct Installer {
	accept_license: bool,
	toasts: Toasts,
}

impl Default for Installer {
	fn default() -> Self {
		Self {
			accept_license: false,
			toasts: Toasts::default(),
		}
	}
}

impl Installer {
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		egui_extras::install_image_loaders(&cc.egui_ctx);
		Self::default()
	}
}

impl eframe::App for Installer {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		self.toasts.show(ctx);
		if let Some(result) = get_install_result() {
			if result.success() {
				self.toasts.success("Install successfully");
			} else {
				if let Some(code) = result.code() {
					self.toasts.error(format!("Failed to install with code {}", code));
				} else {
					self.toasts.error("Failed to install");
				}
			}
			set_install_result(None);
		}
		egui::SidePanel::left("left").resizable(false).show(ctx, |ui| {
			ui.add_space(5.0);
			ui.add(Image::new(include_image!("./assets/cpc.png")).rounding(5.0));
			ui.add_space(10.0);
			ui.add(
				Label::new(
					RichText::new("CAIE Code (cpc) Installer").heading()
				).selectable(false)
			);

			ui.separator();

			let install_bt_size = Vec2::new(200.0, 30.0);

			let bt = Button::new(
				RichText::new("Install").heading()
			).min_size(install_bt_size);
			if ui.add(bt).clicked() {
				if self.accept_license {
					if funcs::check_dependencies() {
						self.toasts.info("Begin to install");
						thread::spawn(|| {
							funcs::install();
						});
					} else {
						self.toasts.error("Failed to check dependencies");
					}
				} else {
					self.toasts.error("Accept license before installation");
				}
			}

			ui.add_space(5.0);

			let custom_bt = Button::new(
				RichText::new("Custom Install").heading()
			).min_size(install_bt_size);
			if ui.add(custom_bt).clicked() {
				self.toasts.info("This function is not available now");
			}
		});
		egui::CentralPanel::default().show(ctx, |ui| {
			ui.checkbox(&mut self.accept_license, "Accept License");

			ui.separator();

			ScrollArea::vertical().show(ui, |ui| {
				ui.with_layout(
					Layout::top_down(egui::Align::Center).with_cross_justify(true),
					|ui| {
						let mut job = LayoutJob::default();
						job.append(
							LICENSE,
							0.0,
							TextFormat {
								color: Color32::from_rgb(0, 0, 0),
								font_id: egui::FontId::monospace(12.0),
								..Default::default()
							}
						);
						ui.add(Label::new(job).selectable(false));
					}
				);
			});
		});
	}
}
