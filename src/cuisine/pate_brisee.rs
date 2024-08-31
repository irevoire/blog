use egui::{Align, Layout, Ui};

#[derive(Default)]
pub struct PateBrisee {
    // Ingrédients
    /// 200 g de farine
    farine: bool,
    /// 1 jaune d’oeuf
    oeuf: bool,
    /// 4 cl d’eau
    eau: bool,
    /// 2 pincées de sel
    sel: bool,
    /// 100 g de beurre mou
    beurre: bool,
}

impl PateBrisee {
    pub fn display(&mut self, ui: &mut Ui) {
        ui.heading("Ingrédients");
        self.ingredients(ui);
        ui.separator();
        ui.heading("Étapes");
        self.etapes(ui);
    }

    pub fn ingredients(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.farine, "200 g de farine blanche");
        ui.checkbox(&mut self.oeuf, "1 jaune d’oeuf");
        ui.checkbox(&mut self.eau, "4 cl d’eau");
        ui.checkbox(&mut self.sel, "2 pincées de sel");
        ui.checkbox(&mut self.beurre, "100 g de beurre mou");
    }

    pub fn etapes(&mut self, ui: &mut Ui) {
        ui.label("1. Couper 100g de beurre a température ambiante pour qu’il ramollisse");
        ui.label("2. Dans un saladier mettre 200g de farine");
        ui.label("3. Ajouter le beurre");
        ui.label("4. Ajouter 4cl d’eau (40g)");
        ui.label("5. Si le beurre était doux, ajouter une pincée de sel");
        ui.label("6. Ajouter 2 jaune d’œufs (gardez les blancs au frigo pour la meringue)");
        ui.label("7. Tout mélanger jusqu’à ce que la pâte soit homogène");
        ui.label("8. Si elle n’est pas assez souple on peut ajouter un peu d’eau (se mouiller les mains puis recommencer a la malaxer est suffisant en général)");
        ui.label("9. Former une boule");
        ui.label("10. Filmer et mettre au frigo");
        ui.label("11. Laisser reposer 20 minutes");
        ui.label("12. Préchauffer le four pendant ce temps a 200 degrés");
        ui.label("13. Fleurer le plan de travail et la boule puis lui donner la forme souhaitée");
        ui.label("14. Beurrer et fariner le moule");
        ui.label("15. Mettre la pâte dedans");
        ui.label("16. Piquer la pâte a la fourchette pour éviter qu’elle ne gonfle en cuisant");
        ui.label("17. Si possible remplir le fond de la tarte de haricot ou billes d’argiles pour éviter qu’elle ne gonfle");
        ui.label("18. Enfourner pour 20 minutes à 200 degrés");
    }

    pub fn link_to_original(&self, ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.label("Repris d'une vidéo de philippe etchebest");
            ui.separator();
            ui.hyperlink_to("Video", "https://www.youtube.com/watch?v=TUcbb8bbmgs");
            ui.separator();
            ui.hyperlink_to(
                "Liste d'ingrédients",
                "https://philippe-etchebest.com/la-pate-brisee",
            )
            .on_hover_ui(|ui| {
                ui.label(
                    "Attention il a fait une erreur sur le nombre de jaune d'œufs dans la recette",
                );
            });
        });
    }
}
