use crate::{linebreaks, settings};

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

pub fn ruleset() -> Vec<linebreaks::Rule> {
    use linebreaks as LB;
    use SettingType as S;
    use LB::Action::*;
    use LB::Case::*;
    use LB::Filler;
    use LB::Match::*;
    use LB::PunctuationKind as P;
    use LB::Rule as R;
    use LB::SymbolPredicate as SP;

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
            previous: SP::new(Punctuation(P::Continuation), Leave),
            following: SP::new(Letter(Anycase), Leave),
            filler: Filler::Space,
        },
        R {
            // Text.⏎More text
            setting: None,
            previous: SP::new(Punctuation(P::EndOfSentence), Leave),
            following: SP::new(Letter(Uppercase), Leave),
            filler: Filler::Linebreak,
        },
        // Removing unnecessary hyphens
        //
        R {
            // Text with conti-⏎nuation
            setting: None,
            previous: SP::new(Punctuation(P::Hyphen), Remove),
            following: SP::new(Letter(Lowercase), Leave),
            filler: Filler::None,
        },
        R {
            // Text with Proper-⏎Noun
            setting: Some((S::SmartHyphenRemoval, true)),
            previous: SP::new(Punctuation(P::Hyphen), Leave),
            following: SP::new(Letter(Uppercase), Leave),
            filler: Filler::None,
        },
        R {
            // Text with Proper-⏎Noun
            setting: Some((S::SmartHyphenRemoval, false)),
            previous: SP::new(Punctuation(P::Hyphen), Remove),
            following: SP::new(Letter(Uppercase), Leave),
            filler: Filler::None,
        },
        // Dealing with unusual structures falling on linebreaks
        //
        R {
            // This/That/⏎TheOther
            setting: None,
            previous: SP::new(Punctuation(P::Slash), Leave),
            following: SP::new(Letter(Anycase), Leave),
            filler: Filler::None,
        },
        R {
            // Text (paranthetical)⏎and more text
            setting: None,
            previous: SP::new(Punctuation(P::Parantheses), Leave),
            following: SP::new(Letter(Anycase), Leave),
            filler: Filler::Space,
        },
        R {
            // Text⏎(paranthetical) and more text
            setting: None,
            previous: SP::new(Letter(Anycase), Leave),
            following: SP::new(Punctuation(P::Parantheses), Leave),
            filler: Filler::Space,
        },
        R {
            // Text — paranthetical —⏎and more text
            setting: None,
            previous: SP::new(Punctuation(P::Dash), Leave),
            following: SP::new(Letter(Anycase), Leave),
            filler: Filler::Space,
        },
        R {
            // Text⏎— paranthetical — and more text
            setting: None,
            previous: SP::new(Letter(Anycase), Leave),
            following: SP::new(Punctuation(P::Dash), Leave),
            filler: Filler::Space,
        },
        R {
            // Quote.⏎— Author
            setting: None,
            previous: SP::new(Punctuation(P::EndOfSentence), Leave),
            following: SP::new(Punctuation(P::Dash), Remove),
            filler: Filler::Exact("\n-".to_string()),
        },
        // Miscellaneous replacements
        //
        R {
            // Section.⏎•Bulletpoint
            setting: None,
            previous: SP::new(Punctuation(P::AnyPunctuation), Leave),
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
