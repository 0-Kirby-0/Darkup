pub enum SettingUpdated {
    NoChange,
    Updated,
}
pub struct Setting<SettingType> {
    pub kind: SettingType,
    pub enabled: bool,
    pub label: String,
    pub explanation: String,
}

impl<SettingType> Setting<SettingType>
where
    SettingType: PartialEq,
{
    pub fn new(kind: SettingType, label: &str, explanation: &str, default_enabled: bool) -> Self {
        Self {
            kind,
            label: label.to_owned(),
            explanation: explanation.to_owned(),
            enabled: default_enabled,
        }
    }
}

pub struct SettingList<SettingType> {
    pub list: Vec<Setting<SettingType>>,
}

impl<SettingType> SettingList<SettingType>
where
    SettingType: PartialEq,
{
    pub fn check(&self, kind: SettingType) -> bool {
        self.list
            .iter()
            .find(|s| s.kind == kind)
            .map(|s| s.enabled)
            .expect("Setting type without matching setting.")
    }

    pub fn egui_render(&mut self, ui: &mut eframe::egui::Ui) -> SettingUpdated {
        let mut updated = SettingUpdated::NoChange;
        self.list.iter_mut().for_each(|setting| {
            let cb = ui
                .checkbox(&mut setting.enabled, setting.label.clone())
                .on_hover_text(setting.explanation.clone());

            if cb.clicked() {
                updated = SettingUpdated::Updated;
            }
        });

        updated
    }
}
