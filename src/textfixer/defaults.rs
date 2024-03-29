use crate::textfixer::{linebreaks, settings, texthelpers};

#[derive(PartialEq, Clone, Copy)]
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

pub fn ruleset() -> Vec<linebreaks::Rule> {
    use linebreaks::Rule as R;
    use texthelpers as TH;
    use SettingType as S;
    use TH::Action::*;
    use TH::Case::*;
    use TH::Filler;
    use TH::Match::*;
    use TH::PunctuationKind::*;
    use TH::SymbolPredicate as SP;

    vec![
        // Taking care of the most common and obvious spurious linebreaks.
        //
        R {
            // Text⏎and more text
            setting: None,
            previous: SP::new(Letter(Lowercase), Leave),
            following: SP::new(Letter(Anycase), Leave),
            filler: Filler::Space,
        },
        R {
            // Text,⏎and more text
            setting: None,
            previous: SP::new(Punctuation(Continuation), Leave),
            following: SP::new(Letter(Anycase), Leave),
            filler: Filler::Space,
        },
        R {
            // Text.⏎More text
            setting: None,
            previous: SP::new(Punctuation(EndOfSentence), Leave),
            following: SP::new(Letter(Uppercase), Leave),
            filler: Filler::Linebreak,
        },
        // Removing unnecessary hyphens
        //
        R {
            // Text with conti-⏎nuation
            setting: None,
            previous: SP::new(Punctuation(Hyphen), Remove),
            following: SP::new(Letter(Lowercase), Leave),
            filler: Filler::None,
        },
        R {
            // Text with Proper-⏎Noun
            setting: Some((S::SmartHyphenRemoval, true)),
            previous: SP::new(Punctuation(Hyphen), Leave),
            following: SP::new(Letter(Uppercase), Leave),
            filler: Filler::None,
        },
        R {
            // Text with Proper-⏎Noun
            setting: Some((S::SmartHyphenRemoval, false)),
            previous: SP::new(Punctuation(Hyphen), Remove),
            following: SP::new(Letter(Uppercase), Leave),
            filler: Filler::None,
        },
        // Dealing with unusual structures falling on linebreaks
        //
        R {
            // This/That/⏎TheOther
            setting: None,
            previous: SP::new(Punctuation(Slash), Leave),
            following: SP::new(Letter(Anycase), Leave),
            filler: Filler::None,
        },
        R {
            // Text (paranthetical)⏎and more text
            setting: None,
            previous: SP::new(Punctuation(Parantheses), Leave),
            following: SP::new(Letter(Anycase), Leave),
            filler: Filler::Space,
        },
        R {
            // Text⏎(paranthetical) and more text
            setting: None,
            previous: SP::new(Letter(Anycase), Leave),
            following: SP::new(Punctuation(Parantheses), Leave),
            filler: Filler::Space,
        },
        R {
            // Text — paranthetical —⏎and more text
            setting: None,
            previous: SP::new(Punctuation(Dash), Leave),
            following: SP::new(Letter(Anycase), Leave),
            filler: Filler::Space,
        },
        R {
            // Text⏎— paranthetical — and more text
            setting: None,
            previous: SP::new(Letter(Anycase), Leave),
            following: SP::new(Punctuation(Dash), Leave),
            filler: Filler::Space,
        },
        R {
            // Quote.⏎— Author
            setting: None,
            previous: SP::new(Punctuation(EndOfSentence), Leave),
            following: SP::new(Punctuation(Dash), Remove),
            filler: Filler::Exact("\n-".to_string()),
        },
        // Miscellaneous replacements
        //
        R {
            // Section.⏎•Bulletpoint
            setting: None,
            previous: SP::new(Punctuation(AnyPunctuation), Leave),
            following: SP::new(Exact('•'), Remove),
            filler: Filler::Exact("\n-".to_string()),
        },
        R {
            // Marker symbol used to stop a linebreak from being removed erroniously.
            // '꠷' (North Indic Placeholder Mark) is used for its apt name and low
            // probability of being found in the source text.
            setting: None,
            previous: SP::new(Exact('꠷'), Remove),
            following: SP::new(Anymatch, Leave),
            filler: Filler::Linebreak,
        },
    ]
}
