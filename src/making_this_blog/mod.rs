use egui::{Context, ScrollArea};
use egui_commonmark::commonmark;
use serde::{Deserialize, Serialize};

use crate::{centered_scrollable, Blog};

macros::create_file!(
    "making-this-blog/index.html",
    "making-this-blog/formatting-test.html"
);

impl Blog {
    pub fn display_making_this_blog_article(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_making_this_blog_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.selectable_value(
                    &mut self.making_this_blog.selected,
                    Pages::FormattingTest,
                    "Formatting test",
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::both().show(ui, |ui| match self.making_this_blog.selected {
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
    FormattingTest,
}

impl MakingThisBlog {
    pub fn as_url_part(&self) -> &'static str {
        match self.selected {
            Pages::FormattingTest => "/formatting-test.html",
        }
    }

    pub fn from_url_parts(_path: std::str::Split<char>) -> Self {
        let mut this = Self::default();
        this.selected = Pages::FormattingTest;
        this
    }
}
