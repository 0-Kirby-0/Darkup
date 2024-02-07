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

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };

    (1, item)
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
