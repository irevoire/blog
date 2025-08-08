mod cuisine;
mod database;
mod macros;
mod making_this_blog;

use eframe::CreationContext;
use egui::{Context, ScrollArea, Ui};
use egui_commonmark::CommonMarkCache;
use serde::{Deserialize, Serialize};

#[derive(derivative::Derivative, Serialize, Deserialize)]
#[derivative(Default)]
pub struct Blog {
    #[serde(skip, default)]
    md_cache: CommonMarkCache,
    #[serde(skip, default)]
    last_url: Option<String>,

    main_article: Article,
    database: database::Database,
    making_this_blog: making_this_blog::MakingThisBlog,
    cuisine: cuisine::Cuisine,
}

#[derive(Default, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Article {
    #[default]
    Main,
    Database,
    MakingThisBlog,
    Cuisine,
}

impl Article {
    pub const fn as_url_part(&self) -> &'static str {
        match self {
            Article::Main => "",
            Article::Database => "/database",
            Article::MakingThisBlog => "/making-this-blog",
            Article::Cuisine => "/cuisine",
        }
    }

    pub fn from_url_part(part: &str) -> Option<Self> {
        log::debug!("Parsing url part: {part}");
        match part {
            "" | "index.html" => Some(Article::Main),
            "database" => Some(Article::Database),
            "making-this-blog" => Some(Article::MakingThisBlog),
            "cuisine" => Some(Article::Cuisine),
            _ => None,
        }
    }
}

impl eframe::App for Blog {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let old = self.as_url();
        let real_url = get_url().unwrap().pathname();
        let old = if real_url
            .trim_end_matches('/')
            .trim_end_matches("/index.html")
            != old
        {
            log::debug!("Last url `{old}` is different from the current one `{real_url}`, the prev button was probably pressed, reloading from it");
            *self = Self::from_url(&real_url);
            self.as_url()
        } else {
            old
        };

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                egui::global_theme_preference_switch(ui);
                ui.separator();
                ui.selectable_value(
                    &mut self.main_article,
                    Article::Main,
                    "Tamo's personal blog",
                );
                ui.selectable_value(&mut self.main_article, Article::Database, "Database");
                ui.selectable_value(
                    &mut self.main_article,
                    Article::MakingThisBlog,
                    "Making this blog",
                );
                ui.selectable_value(&mut self.main_article, Article::Cuisine, "Cuisine");
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.add(egui::github_link_file!(
                    "https://github.com/irevoire/blog",
                    "Source code."
                ));
            });
        });

        match self.main_article {
            Article::Main => self.display_main_article(ctx),
            Article::Database => self.display_database_article(ctx),
            Article::MakingThisBlog => self.display_making_this_blog_article(ctx),
            Article::Cuisine => self.display_cuisine_article(ctx),
        }

        let new = self.as_url();
        if old.trim_end_matches('/').trim_end_matches("/index.html")
            != new.trim_end_matches('/').trim_end_matches("/index.html")
        {
            use web_sys::wasm_bindgen::JsValue;

            if let Some(window) = web_sys::window() {
                if let Ok(history) = window.history() {
                    let url = window.location().href().unwrap();
                    let url = web_sys::Url::new(&url).unwrap();
                    let origin = url.origin();
                    let url = web_sys::Url::new(&format!("{}{}", origin, self.as_url())).unwrap();
                    let url = url.to_string().as_string().unwrap();

                    log::debug!("Setting url to: {url}");
                    history
                        .push_state_with_url(&JsValue::null(), "", Some(&url))
                        .unwrap();
                    self.last_url = Some(url);
                }
            }
        }
    }
}

fn get_url() -> Option<web_sys::Url> {
    let window = web_sys::window()?;
    let url = window.location().href().unwrap();
    let url = web_sys::Url::new(&url).unwrap();
    Some(url)
}

impl Blog {
    pub fn new(_cc: &CreationContext) -> Self {
        #[allow(unused_mut)]
        let mut this = Self::default();

        if let Some(url) = get_url() {
            let path = url.pathname();
            log::debug!("Making a new blog from path: {}", path);
            this = Self::from_url(&path);
        }
        this
    }

    fn from_url(url: &str) -> Self {
        let mut this = Self::default();
        let mut path = url.trim_matches('/').split('/');
        let page = path
            .next()
            .map_or(Article::Main, |part| Article::from_url_part(part).unwrap());
        log::debug!("Identified current opened page as: {:?}", page);
        this.main_article = page;
        match page {
            Article::Main => {}
            Article::Database => {
                this.database = database::Database::from_url_parts(path);
            }
            Article::MakingThisBlog => {
                this.making_this_blog = making_this_blog::MakingThisBlog::from_url_parts(path);
            }
            Article::Cuisine => {
                this.cuisine = cuisine::Cuisine::from_url_parts(path);
            }
        }
        this
    }

    fn as_url(&self) -> String {
        let mut url = String::from("");
        url.push_str(self.main_article.as_url_part());
        match self.main_article {
            Article::Main => {}
            Article::Database => {
                url.push_str(&self.database.as_url_part());
            }
            Article::MakingThisBlog => {
                url.push_str(self.making_this_blog.as_url_part());
            }
            Article::Cuisine => {
                url.push_str(self.cuisine.as_url_part());
            }
        }
        url
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
