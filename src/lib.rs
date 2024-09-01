mod arroy;
mod cuisine;

use arroy::Arroy;
use eframe::CreationContext;
use egui::{Context, ScrollArea, Ui};
use serde::{Deserialize, Serialize};

#[derive(derivative::Derivative, Serialize, Deserialize, Clone, PartialEq)]
#[derivative(Default)]
pub struct Blog {
    main_article: Article,
    #[serde(skip)]
    arroy: Arroy,
    cuisine: cuisine::Cuisine,
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
pub enum Article {
    #[default]
    Main,
    Arroy,
    Cuisine,
}

impl eframe::App for Blog {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        #[allow(unused_variables)]
        let old = self.clone();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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

        #[cfg(target_arch = "wasm32")]
        if &old != self {
            use web_sys::wasm_bindgen::JsValue;

            if let Some(window) = web_sys::window() {
                if let Ok(history) = window.history() {
                    let url = window.location().href().unwrap();
                    let url = web_sys::Url::new(&url).unwrap();
                    let state = serde_json::to_string(&self).unwrap();
                    url.search_params().set("state", &state);
                    history
                        .push_state_with_url(
                            &JsValue::null(),
                            "",
                            url.to_string().as_string().as_deref(),
                        )
                        .unwrap();
                }
            }
        }
    }
}

impl Blog {
    pub fn new(_cc: &CreationContext) -> Self {
        #[allow(unused_mut)]
        let mut this = None;
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                let url = window.location().href().unwrap();
                let url = web_sys::Url::new(&url).unwrap();
                if let Some(state) = url.search_params().get("state") {
                    this = serde_json::from_str(&state).ok();
                }
            }
        }
        this.unwrap_or_default()
    }

    fn display_main_article(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            centered_scrollable(ui, |ui| {
                ui.heading("Hey");
                ui.label("This is my personal blog.");
                ui.label("I plan to use it to write complex stuff that does not fit in documentations and that I will forget otherwise.");
            });
        });
    }
}

pub(crate) fn centered_scrollable(ui: &mut Ui, f: impl FnOnce(&mut Ui)) {
    ScrollArea::vertical()
        .min_scrolled_width(ui.available_width())
        .show(ui, |ui| {
            let max_width = ui.available_width();
            // The text shouldn't be larger than the available size
            ui.set_max_width(max_width.min(1100.));
            f(ui);
        });
}
