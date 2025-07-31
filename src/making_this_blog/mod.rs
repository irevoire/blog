use egui::{Context, ScrollArea};
use egui_commonmark::{commonmark, commonmark_str};
use serde::{Deserialize, Serialize};

use crate::{centered_scrollable, Blog};

macros::create_file!(
    "making-this-blog/index.html",
    "making-this-blog/making-the-url-works.html",
    "making-this-blog/formatting-test.html",
);

impl Blog {
    pub fn display_making_this_blog_article(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_making_this_blog_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.making_this_blog.selected,
                    Pages::About,
                    "About",
                );
                ui.selectable_value(
                    &mut self.making_this_blog.selected,
                    Pages::MakingTheUrlWorks,
                    "Making the url works",
                );
                ui.selectable_value(
                    &mut self.making_this_blog.selected,
                    Pages::FormattingTest,
                    "Formatting test",
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().show(ui, |ui| match self.making_this_blog.selected {
                Pages::About => centered_scrollable(ui, |ui| {
                    let _response = commonmark_str!(
                        ui,
                        &mut self.md_cache,
                        "content/making-this-blog/index.md"
                    );
                }),
                Pages::MakingTheUrlWorks => centered_scrollable(ui, |ui| {
                    let _response = commonmark_str!(
                        ui,
                        &mut self.md_cache,
                        "content/making-this-blog/making-the-url-works.md"
                    );
                }),
                Pages::FormattingTest => centered_scrollable(ui, |ui| {
                    let _response = commonmark!(
                        ui,
                        &mut self.md_cache,
                        "Can I display a simple string? Yes!

# Can I display a title? Yes!!
                        
Lol"
                    );
                }),
            })
        });
    }
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct MakingThisBlog {
    selected: Pages,
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
enum Pages {
    #[default]
    About,
    MakingTheUrlWorks,
    FormattingTest,
}

impl Pages {
    fn from_url_part(part: &str) -> Option<Self> {
        match part {
            "" | "index.html" => Some(Self::default()),
            "making-the-url-works.html" => Some(Self::MakingTheUrlWorks),
            "formatting-test.html" => Some(Self::FormattingTest),
            _ => None,
        }
    }
}

impl MakingThisBlog {
    pub fn as_url_part(&self) -> &'static str {
        match self.selected {
            Pages::About => "/index.html",
            Pages::MakingTheUrlWorks => "/making-the-url-works.html",
            Pages::FormattingTest => "/formatting-test.html",
        }
    }

    pub fn from_url_parts(mut parts: std::str::Split<char>) -> Self {
        let selected = parts
            .next()
            .map(|part| Pages::from_url_part(part).unwrap())
            .unwrap_or_default();
        let mut this = Self::default();
        this.selected = selected;
        this
    }
}
