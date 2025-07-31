mod arroy;

use egui::Context;
use egui_commonmark::commonmark_str;
use serde::{Deserialize, Serialize};

use crate::{centered_scrollable, Blog};

macros::create_file!("database/index.html");

impl Blog {
    pub fn display_database_article(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("database_top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.database.selected, Pages::About, "About");
                ui.selectable_value(&mut self.database.selected, Pages::Cellulite, "Cellulite");
                ui.selectable_value(&mut self.database.selected, Pages::Arroy, "Arroy");
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            centered_scrollable(ui, |ui| {
                match self.database.selected {
                    Pages::About => {
                        commonmark_str!(ui, &mut self.md_cache, "content/database/index.md");
                    }
                    Pages::Cellulite => todo!(), // self.display_cellulite_section(ui),
                    Pages::Arroy => self.display_arroy_article(ctx),
                }
            });
        });
    }
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum Pages {
    #[default]
    About,
    Cellulite,
    Arroy,
}

impl Pages {
    pub fn as_url_part(&self) -> &'static str {
        match self {
            Self::About => "",
            Self::Cellulite => "/cellulite",
            Self::Arroy => "/arroy",
        }
    }

    pub fn from_url_part(part: &str) -> Option<Self> {
        match part {
            "about" => Some(Self::About),
            "cellulite" => Some(Self::Cellulite),
            "arroy" => Some(Self::Arroy),
            _ => None,
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Database {
    selected: Pages,
    arroy: arroy::Arroy,
}

impl Database {
    pub fn as_url_part(&self) -> String {
        let mut url = self.selected.as_url_part().to_string();
        let part = match self.selected {
            Pages::About => "".to_string(),
            Pages::Cellulite => todo!(),
            Pages::Arroy => self.arroy.as_url_part().to_string(),
        };
        url.push_str(&part);
        url
    }

    pub fn from_url_parts(mut parts: std::str::Split<char>) -> Self {
        let mut this = Self::default();
        this.selected = parts
            .next()
            .map(|part| Pages::from_url_part(part).unwrap())
            .unwrap_or_default();
        match this.selected {
            Pages::About => (),
            Pages::Cellulite => todo!(),
            Pages::Arroy => {
                this.arroy = arroy::Arroy::from_url_parts(parts);
            }
        }
        this
    }
}
