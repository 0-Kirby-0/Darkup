use crate::defaults;
use crate::headers;
use crate::linebreaks;
use crate::settings;

pub struct Textfixer {
    ruleset: Vec<linebreaks::Rule>,
    pub settings: settings::SettingList<defaults::SettingType>,
}
impl Default for Textfixer {
    fn default() -> Self {
        Self {
            ruleset: defaults::ruleset(),
            settings: defaults::setting_list(),
        }
    }
}

impl Textfixer {
    pub fn fix(&self, instring: &str) -> String {
        let mut lines = instring
            .lines()
            .map(|l| l.trim().to_owned())
            .collect::<Vec<_>>();
        lines = headers::apply(&lines, &self.settings);
        linebreaks::apply(&lines, &self.ruleset, &self.settings)
    }
}

/*TODO!

fn fix(&self, instring) -> String {
    mark_tables();
    mark_headings();
    apply_linebreak_rules();

    outstring
}

fn mark_tables()
fn mark_headings()
fn apply_linebreak_rules()
 */

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
        let teststr = r"Dux Bellorum
(Camarilla; 4-point Title)
When the Camarilla mobilizes its members as
a war-force, it often selects a Dux Bellorum from
among the ranks of the Archons, Justicars, or even
extremely competent Alastors. The Dux Bellorum is
a battle marshal, the master of a Camarilla combat
engagement. He may be a front-line warlord, leading
a bloody charge into a Sabbat domain, or he may be
a scheming tactician, organizing guerilla strikes to de-
stabilize an enemy territory from within.";
        let lines = teststr
            .lines()
            .map(|s| s.to_owned())
            .collect::<Vec<String>>();
        let fixed = headers::apply(&lines, &textfixer.settings).join("\n");
        eprintln!("'{fixed}'");
    }
}
