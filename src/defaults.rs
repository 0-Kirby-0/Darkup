use crate::settings;

#[derive(PartialEq)]
pub enum SettingType {
    SmartHyphenRemoval,
    MarkdownSectionHeadings,
    MarkdownSubheadings,
    SimplifiedHeadings,
    SeparateHeadingClarifiers,
}

pub fn setting_list() -> settings::SettingList<SettingType> {
    use SettingType as ST;
    settings::SettingList {
        list: vec![
            settings::Setting::new(
                ST::SmartHyphenRemoval,
                "Smart Hyphen Removal",
                "Leave hyphens if they are part of a proper noun.",
                true,
            ),
            settings::Setting::new(
                ST::MarkdownSectionHeadings,
                "Section Headings",
                "Guess at and mark section headings using Markdown.",
                true,
            ),
            settings::Setting::new(
                ST::MarkdownSubheadings,
                "Subheadings",
                "Guess at and mark subheadings using Markdown.",
                true,
            ),
            settings::Setting::new(
                ST::SimplifiedHeadings,
                "Simplified Headings",
                "Strips fancy symbols from headings, for easier searching.",
                false,
            ),
            settings::Setting::new(
                ST::SeparateHeadingClarifiers,
                "Separate Clarifiers",
                "Strips heading clarifiers (in parantheses), and turns them into subheadings.",
                true,
            ),
        ],
    }
}

pub fn example_text() -> String {
    r"Example Heading (with clarifier)
Subheading: With some text

Cömplícated Häding (with diacritics øóúé)
Regular text with
shitty line bre-
aks but also Proper-
Nouns."
        .to_owned()
}
