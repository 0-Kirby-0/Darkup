use crate::texthelpers::*;
use crate::{defaults, settings};

pub struct Rule {
    pub setting: Option<(defaults::SettingType, bool)>,
    pub previous: SymbolPredicate,
    pub following: SymbolPredicate,
    pub filler: Filler,
}

impl Rule {
    fn matches(&self, previous: char, following: char) -> bool {
        self.previous.symbol.matches(previous) && self.following.symbol.matches(following)
    }
    fn merge(&self, mut left: String, mut right: &str) -> String {
        if self.previous.on_match == Action::Remove {
            left.pop();
        }
        if self.following.on_match == Action::Remove {
            right = &right[1..]
        }

        left + self.filler.get() + right
    }
    fn is_enabled(&self, settings: &settings::SettingList<defaults::SettingType>) -> bool {
        if let Some((setting, enabled)) = self.setting {
            settings.check(setting) == enabled
        } else {
            true //has no associated setting, so is always enabled
        }
    }
}

pub fn apply(
    lines: &[String],
    ruleset: &[Rule],
    settings: &settings::SettingList<defaults::SettingType>,
) -> String {
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

        if let Some(rule) = ruleset
            .iter()
            .find(|r| r.matches(previous_char, following_char) && r.is_enabled(settings))
        {
            outstring = rule.merge(outstring, line); //apply the matching rule
            continue;
        } else {
            outstring = outstring + "\n" + &line; //no rule applies, add the linebreak and move on
            continue;
        };
    }

    outstring
}
