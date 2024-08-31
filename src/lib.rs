mod arroy;
mod cuisine;

use arroy::Arroy;
use eframe::CreationContext;
use egui::{Context, Margin, Ui};

#[derive(derivative::Derivative)]
#[derivative(Default)]
pub struct Blog {
    main_article: Article,
    #[derivative(Default(value = "850.0"))]
    text_width: f32,
    #[derivative(Default(value = "false"))]
    text_width_selected: bool,
    arroy: Arroy,
    cuisine: cuisine::Cuisine,
}

#[derive(Default, PartialEq, Eq)]
pub enum Article {
    #[default]
    Main,
    Arroy,
    Cuisine,
}

impl eframe::App for Blog {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            let max_width = ui.available_width();
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                ui.selectable_value(
                    &mut self.main_article,
                    Article::Main,
                    "Tamo's personal blog",
                );
                ui.selectable_value(&mut self.main_article, Article::Arroy, "Arroy");
                ui.selectable_value(&mut self.main_article, Article::Cuisine, "Cuisine");

                ui.separator();
                self.text_width_selected = ui
                    .add(
                        egui::Slider::new(&mut self.text_width, 0.0..=max_width).text("Text width"),
                    )
                    .dragged();
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.add(egui::github_link_file!(
                    "https://github.com/irevoire/blog",
                    "Source code."
                ));
            });
        });

        match self.main_article {
            Article::Main => self.display_main_article(ctx),
            Article::Arroy => self.display_arroy_article(ctx),
            Article::Cuisine => self.display_cuisine_article(ctx),
        }
    }
}

impl Blog {
    pub fn new(_cc: &CreationContext) -> Self {
        Self::default()
    }

    fn display_text_content(&mut self, ui: &mut Ui, mut f: impl FnMut(&mut Self, &mut Ui)) {
        let width = ui.available_width();
        let mut remove = (width - self.text_width) / 2.0;
        if remove.is_sign_negative() {
            remove = 0.0;
        }
        ui.label(format!("width: {width}, remove: {remove}"));

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.set_min_height(ui.available_height());

            let mut frame = egui::Frame::none().outer_margin(Margin::symmetric(remove, 0.0));
            if self.text_width_selected {
                frame = frame.fill(egui::Color32::RED.gamma_multiply(0.15));
            }
            frame.show(ui, |ui| {
                ui.set_min_width(ui.available_width());
                ui.vertical_centered_justified(|ui| f(self, ui));
                // f(ui);
            });
        });
    }

    fn display_main_article(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.display_text_content(ui, |_, ui| {
                ui.heading("Hey");
                ui.label("This is my personal blog.");
                ui.label("I plan to use it to write complex stuff that does not fit in documentations and that I will forget otherwise.");
            });
        });
    }
}
