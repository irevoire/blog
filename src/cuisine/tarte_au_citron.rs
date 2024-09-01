use egui::{Align, Layout, Ui};
use serde::{Deserialize, Serialize};

use super::pate_brisee::PateBrisee;

#[derive(Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TarteAuCitron {
    // List de courses
    /// 200 g de farine
    farine: bool,
    /// 25 g de fécule de maïs
    maizena: bool,
    /// 13 oeufs
    oeufs: bool,
    /// 2 pincées de sel
    sel: bool,
    /// 200 g de beurre mou
    beurre: bool,
    /// 250 g de jus de citron -> (5 gros citron ou 6 petits)
    citron: bool,
    /// Un citron vert pour le zeste (optionnel)
    citron_vert: bool,
    /// 400 g de sucre en poudre
    sucre: bool,

    // Étapes
    fond_de_tarte: PateBrisee,
}

impl TarteAuCitron {
    pub fn display(&mut self, ui: &mut Ui) {
        ui.heading("Liste de courses");
        self.ingredients(ui);
        ui.separator();
        ui.heading("Étapes");
        self.etapes(ui);
    }

    pub fn ingredients(&mut self, ui: &mut Ui) {
        ui.checkbox(&mut self.farine, "200 g de farine");
        ui.checkbox(&mut self.maizena, "25 g de fécule de maïs");
        ui.checkbox(&mut self.oeufs, "13 oeufs");
        ui.checkbox(&mut self.sel, "2 pincées de sel");
        ui.checkbox(&mut self.beurre, "200 g de beurre mou");
        ui.checkbox(
            &mut self.citron,
            "250 g de jus de citron -> (5 gros citron ou 6 petits)",
        );
        ui.checkbox(
            &mut self.citron_vert,
            "[optionnel] Un citron vert pour son zeste",
        );
        ui.checkbox(&mut self.sucre, "400 g de sucre en poudre");
    }

    pub fn etapes(&mut self, ui: &mut Ui) {
        ui.collapsing("Fond de tarte", |ui| self.fond_de_tarte.etapes(ui));
        ui.collapsing("Lemon curd", |ui| self.lemon_curd(ui));
        ui.collapsing("Meringue", |ui| self.meringue(ui));
        ui.collapsing("Assemblage", |ui| self.assemblage(ui));
    }

    fn lemon_curd(&mut self, ui: &mut Ui) {
        ui.label("1. Mettre un saladier en métal au congélateur");
        ui.label("2. Couper 100g de beurre en cube et laisser a température ambiante");
        ui.label("3. [Optionnel] : récupérer les zestes des citrons");
        ui.label("4. Presser les citrons dans une casserole");
        ui.label("5. Mettre le jus a chauffer a feu doux, il ne doit pas bouillir ni réduire (86 degrés)");
        ui.label("6. Pendant ce temps casser 10 œufs entier dans un saladier");
        ui.label("7. Fouetter les œufs");
        ui.label("8. Mélanger 250g de sucre avec 25g de fécule de maïs");
        ui.label("9. [optionnel] : rajouter les zestes au sucre et a la fécule");
        ui.label("10. Intégrer le mélange aux œufs");
        ui.label("11. Ajouter le mélange au jus de citron, toujours a 86 degrés qui ne doit pas bouillir");
        ui.label(
            "12. Fouetter énergiquement jusqu’à ce que ça monte en volume et que ça s’épaississe ",
        );
        ui.label("13. Une fois que ça s’est épaissi on baisse le feu a feu très doux (60 degrés)");
        ui.label("14. On y intègre les 100g de beurre coupé en morceau au début de la recette");
        ui.label(
            "15. Continuer de remuer jusqu’à ce que le beurre soit totalement intégré a la crème",
        );
        ui.label("16. Sortir du feu et mettre dans le saladier en métal sorti du congélateur");
        ui.label("17. Filmer et mettre au frigo si possible pour couper la cuisson");
    }

    fn meringue(&mut self, ui: &mut Ui) {
        ui.label("1. Préparer 150g de sucre blanc dans un bol");
        ui.label("2. Commencer a battre 3 blancs d’œufs en neige");
        ui.label("3. Intégrer la moitié du sucre dans les blancs lorsqu’ils deviennent mousseux");
        ui.label("4. Intégrer le reste quand les blancs commencent a se tenir");
    }

    fn assemblage(&mut self, ui: &mut Ui) {
        ui.label("1. On met le lemon curd dans le fond de tarte");
        ui.label(
            "2. On met la meringue sur le lemon curd qui devrait se tenir après avoir refroidi",
        );
        ui.label("3. A l’aide d’une fourchette qu’on va planter a répétition dans la meringue on va venir créer un peu de texture");
        ui.label("4. On fait cuire la meringue d’un coup de chalumeau ou au four ouvert collé aux résistances");
        ui.label("5. [optionnel] : On ajoute les zestes de citron vert");
    }

    pub fn link_to_original(&self, ui: &mut Ui) {
        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
            ui.label("Repris d'une vidéo de philippe etchebest");
            ui.separator();
            ui.hyperlink_to("Video", "https://www.youtube.com/watch?v=N7kY8vuCbVI");
            ui.separator();
            ui.hyperlink_to(
                "Liste d'ingrédients",
                "https://philippe-etchebest.com/recette-tarte-citron-meringuee/",
            );
        });
    }
}
