#![allow(dead_code, private_interfaces)]

use std::sync::Arc;

use egui::{text::LayoutJob, Align, Color32, FontId, FontSelection, RichText, Style};

pub struct TextCompositor {
    layout: LayoutJob,
    style: Arc<Style>,
    v: Vec<FragmentStyle>,
}

impl TextCompositor {
    pub fn new(style: Arc<Style>) -> Self {
        Self {
            layout: LayoutJob::default(),
            style,
            v: Vec::new(),
        }
    }

    pub fn push(&mut self, style: FragmentStyle) {
        if let Some(last) = self.v.last() {
            self.v.push(last.merge(&style));
        } else {
            self.v.push(style);
        }
    }

    pub fn pop(&mut self) {
        self.v.pop();
    }

    pub fn italic(&mut self) {
        self.push(FragmentStyle::new().italics());
    }

    pub fn strong(&mut self) {
        self.push(FragmentStyle::new().strong());
    }

    pub fn append(&mut self, text: &str) {
        let style = self.v.last().cloned().unwrap_or_default();
        let mut text = RichText::new(text);
        let fallback_font = FontSelection::Default;

        if let Some(color) = style.color {
            text = text.color(color);
        }
        if let Some(font) = style.font.clone() {
            text = text.font(font);
        }
        if style.heading {
            text = text.heading();
        }
        if style.bold {
            text = text.strong();
        }
        if style.italics {
            text = text.italics();
        }

        text.append_to(
            &mut self.layout,
            &self.style,
            fallback_font,
            style.align.unwrap_or_default(),
        );
    }

    pub fn finish(self) -> LayoutJob {
        self.layout
    }
}

#[derive(Default, Clone)]
struct FragmentStyle {
    color: Option<Color32>,
    font: Option<FontId>,
    align: Option<Align>,
    heading: bool,
    bold: bool,
    italics: bool,
}

impl FragmentStyle {
    fn new() -> Self {
        Self::default()
    }

    fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    fn font(mut self, font: FontId) -> Self {
        self.font = Some(font);
        self
    }

    fn align(mut self, align: Align) -> Self {
        self.align = Some(align);
        self
    }

    fn strong(mut self) -> Self {
        self.bold = true;
        self
    }

    fn italics(mut self) -> Self {
        self.italics = true;
        self
    }

    fn heading(mut self) -> Self {
        self.heading = true;
        self
    }

    // The last always wins
    fn merge(&self, other: &FragmentStyle) -> Self {
        Self {
            color: self.color.or(other.color),
            font: self.font.clone().or(other.font.clone()),
            align: self.align.or(other.align),
            bold: self.bold || other.bold,
            italics: self.italics || other.italics,
            heading: self.heading || other.heading,
        }
    }
}
