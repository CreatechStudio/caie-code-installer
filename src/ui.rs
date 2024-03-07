use std::time::Duration;

use eframe::egui::{self, include_image, text::LayoutJob, Button, Color32, Image, Label, Layout, RichText, ScrollArea, TextFormat, Vec2};

#[cfg(target_os = "windows")]
use eframe::egui::{CollapsingHeader, Grid};

use egui_notify::Toasts;

use crate::{constants::LICENSE, funcs};

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

			let bt = Button::new(
				RichText::new("Install").heading()
			).min_size(Vec2::new(200.0, 30.0));
			if ui.add(bt).clicked() {
				if self.accept_license {
					funcs::install();
				} else {
					self.toasts.error("Accept License before Installation")
						.set_duration(Some(Duration::from_secs(2)));
				}
			}

			#[cfg(target_os = "windows")]
			CollapsingHeader::new("Advanced Configs")
				.id_source("configs")
				.show(ui, |ui| {
					Grid::new("grid")
					.num_columns(2)
					.show(ui, |ui| {
						ui.label("Installation Path");
						ui.label("/usr/local/sbin");
						ui.end_row();
					});
				});
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
