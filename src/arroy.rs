use egui::{Context, SidePanel, TopBottomPanel, Ui};
use egui_plot::{Legend, MarkerShape, Plot, Points};
use rand::{rngs::StdRng, Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::Blog;

impl Blog {
    pub fn display_arroy_article(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.display_text_content(ui, |this, ui| {
                ui.heading("Hey, in this article we're going to see the big principles of Arroy.");
                ui.heading("Two means");
                ui.label("The principle of « two means » is to compute a barycenter.");
                ui.label("As a reminder, here's an algorithm showing how to find the barycenter of a buch of points");
                ui.small("For our example we considers that all the points have the same weights");
                this.arroy.charts.barycenter.show(ui);
            });
        });
    }
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Arroy {
    charts: Charts,
}

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
struct Charts {
    barycenter: BaryCenter,
}

#[derive(Debug, derivative::Derivative, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[derivative(Default)]
struct BaryCenter {
    seed: u64,
    #[derivative(Default(value = "100"))]
    points: usize,
    step: usize,
}

impl BaryCenter {
    pub fn show(&mut self, ui: &mut Ui) {
        TopBottomPanel::bottom("barycenter steps").show_inside(ui, |ui| {
            ui.set_min_width(ui.available_width());
            ui.add_sized(
                ui.available_size(),
                egui::Slider::new(&mut self.step, 0..=self.points)
                    .prefix("Step: ")
                    .show_value(false),
            );
        });
        SidePanel::right("barycenter options").show_inside(ui, |ui| {
            if ui
                .button("Regenerate")
                .on_hover_text("Your seed will be lost forever. Will reset your step as well.")
                .clicked()
            {
                let mut buf = [0; std::mem::size_of::<u64>()];
                getrandom::getrandom(&mut buf).unwrap();
                self.seed = u64::from_be_bytes(buf);
                self.step = 0;
            }
            ui.add(egui::DragValue::new(&mut self.points).speed(1));
        });

        let radius = 5.0;
        let plot = Plot::new("Barycenter")
            .data_aspect(1.0)
            .legend(Legend::default());
        plot.show(ui, |plot_ui| {
            let mut rng = StdRng::seed_from_u64(self.seed);
            let mut points: Vec<[f64; 2]> = Vec::new();
            for _ in 0..self.points.min(self.step) {
                points.push(rng.gen());
            }
            plot_ui.points(
                Points::new(points)
                    .shape(MarkerShape::Circle)
                    .radius(radius),
            );
        });
    }
}
