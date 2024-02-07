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
