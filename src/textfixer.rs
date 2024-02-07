use crate::defaults;
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
    fn apply_linebreaks(&self, lines: &[String]) -> String {
        let mut line_iter = lines.iter();
        let Some(mut outstring) = line_iter.next().cloned() else {
            return String::default(); //input was empty
        };

        for line in line_iter {
            let previous_char = outstring
                .chars()
                .last()
                .expect("Outstring was empty when getting previous char.");
            let Some(following_char) = line.chars().next() else {
                outstring += "\n"; //line is empty, add its linebreak and move on
                continue;
            };

            if let Some(rule) = self
                .ruleset
                .iter()
                .find(|r| r.matches(previous_char, following_char))
            {
                outstring = rule.merge(outstring, line); //apply matching rule
            } else {
                outstring = outstring + "\n" + &line; //no rule applies, add the linebreak and move on
                continue;
            };
        }

        outstring
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
        let fixed = textfixer.apply_linebreaks(&lines);
        eprintln!("'{fixed}'");
    }
}
