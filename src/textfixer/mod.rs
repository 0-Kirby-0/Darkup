mod defaults;
mod headers;
mod linebreaks;
mod settings;
mod texthelpers;

pub struct Textfixer {
    instring: String,
    outstring: String,
    ruleset: Vec<linebreaks::Rule>,
    settings: settings::SettingList<defaults::SettingType>,
}
impl Default for Textfixer {
    fn default() -> Self {
        Self {
            instring: String::default(),
            outstring: String::default(),
            ruleset: defaults::ruleset(),
            settings: defaults::setting_list(),
        }
    }
}

impl Textfixer {
    pub fn set_string(&mut self, instring: &str) {
        self.instring = instring.to_owned();
        self.fix();
    }
    pub fn get_string(&self) -> &str {
        &self.outstring
    }
    pub fn fix(&mut self) {
        let mut lines = self
            .instring
            .lines()
            .map(|l| l.trim().to_owned())
            .collect::<Vec<_>>();
        lines = headers::apply(lines, &self.settings);
        self.outstring = linebreaks::apply(&lines, &self.ruleset, &self.settings);
    }
    pub fn egui_render_settings(&mut self, ui: &mut eframe::egui::Ui) {
        let updated = self.settings.egui_render(ui);
        if updated == settings::SettingUpdated::Updated {
            self.fix();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linebreaks() {
        let textfixer = Textfixer::default();
        let teststr = r"Too often, reckless neonates and ancillae dismiss
elders as hoary old bats, unable to work technology
and fearful of the very concept. While it’s certainly
true that elders either eschew technology themselves
or look down their nose at it in others, only the most
foolish young vampire discounts the ability of elders
to actually learn to take advantage of technology. In
fact, keeping oneself updated is one of the most fear-
some tactics in an elder’s arsenal: Given that many
elders cultivate Resources and other Backgrounds to
levels unattainable by fledglings, combining these ad-
vantages with modern advancements makes for a po-
tent mixture.";
        let lines = teststr
            .lines()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let fixed = linebreaks::apply(&lines, &textfixer.ruleset, &textfixer.settings);
        eprintln!("'{fixed}'");
    }
    #[test]
    fn headers() {
        let textfixer = Textfixer::default();
        let teststr = r"Definitely a Heading (With a subheading)
Followed: By a subheading or something.";
        let lines = teststr
            .lines()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let fixed = headers::apply(lines, &textfixer.settings).join("\n");
        eprintln!("'{fixed}'");
    }
}
