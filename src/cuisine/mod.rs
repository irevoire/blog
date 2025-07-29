use egui::{Context, ScrollArea};
use serde::{Deserialize, Serialize};

mod pate_brisee;
mod tarte_au_citron;

use crate::{centered_scrollable, Blog};

use self::pate_brisee::PateBrisee;
use self::tarte_au_citron::TarteAuCitron;

macros::create_file!(cuisine / index.html);
macros::create_file!(cuisine / pate - brisee.html);
macros::create_file!(cuisine / tarte - au - citron.html);

impl Blog {
    pub fn display_cuisine_article(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_cuisine_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.selectable_value(&mut self.cuisine.selected, Plats::About, "À propos");
                ui.selectable_value(&mut self.cuisine.selected, Plats::PateBrisee, "Pâte brisée");
                ui.selectable_value(
                    &mut self.cuisine.selected,
                    Plats::TarteAuCitron,
                    "Tarte au citron meringuée",
                );
            });
        });
        egui::TopBottomPanel::bottom("bottom_cuisine_panel").show(ctx, |ui| {
            match self.cuisine.selected {
                Plats::About => (),
                Plats::PateBrisee => self.cuisine.pate_brisee.link_to_original(ui),
                Plats::TarteAuCitron => self.cuisine.tarte_au_citron.link_to_original(ui),
            }
        });

        egui::CentralPanel::default().show(ctx, |ui|
            ScrollArea::both().show(ui, |ui| {
                match self.cuisine.selected {
                    Plats::About => centered_scrollable(ui, |ui| {
                            ui.heading("Hey, this section is just a list of recipe's I like and am bored to find again on the internet.");
                            ui.heading("It's going to be in french sorry bye.");
                        }),
                    Plats::PateBrisee => self.cuisine.pate_brisee.display(ui),
                    Plats::TarteAuCitron => self.cuisine.tarte_au_citron.display(ui),
                }
            })
        );
    }
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Cuisine {
    selected: Plats,

    #[serde(skip)]
    pate_brisee: PateBrisee,
    #[serde(skip)]
    tarte_au_citron: TarteAuCitron,
}

impl Cuisine {
    pub fn as_url_part(&self) -> &'static str {
        match self.selected {
            Plats::About => "",
            Plats::PateBrisee => "/pate-brisee.html",
            Plats::TarteAuCitron => "/tarte-au-citron.html",
        }
    }

    pub fn from_url_parts(mut parts: std::str::Split<char>) -> Self {
        let selected = parts
            .next()
            .map_or(Plats::About, |part| Plats::from_url_part(part).unwrap());
        let mut this = Self::default();
        this.selected = selected;
        this
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Plats {
    #[default]
    About,
    PateBrisee,
    TarteAuCitron,
}

impl Plats {
    fn from_url_part(part: &str) -> Option<Self> {
        match part {
            "" | "index.html" => Some(Plats::About),
            "pate-brisee.html" => Some(Plats::PateBrisee),
            "tarte-au-citron.html" => Some(Plats::TarteAuCitron),
            _ => None,
        }
    }
}
