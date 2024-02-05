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
    use linebreaks::Case as C;
    use linebreaks::Match as M;
    use linebreaks::PunctuationKind as P;
    use linebreaks::Rule as R;
    use linebreaks::Substitute as Sub;
    use SettingType as S;

    let hypen = '-';
    let dash = '—';

    vec![
        //Taking care of the most common and obvious spurious linebreaks.
        //
        R {
            //Text⏎and more text
            setting: None,
            previous: M::Letter(C::Lowercase),
            following: M::Letter(C::Any),
            substitutions: [Sub::Leave, Sub::Replace(" ".to_string()), Sub::Leave],
        },
        R {
            //Text,⏎and more text
            setting: None,
            previous: M::Punctuation(P::Continuation),
            following: M::Letter(C::Any),
            substitutions: [Sub::Leave, Sub::Replace(" ".to_string()), Sub::Leave],
        },
        R {
            //Text.⏎More text
            setting: None,
            previous: M::Punctuation(P::EndOfSentence),
            following: M::Letter(C::Uppercase),
            substitutions: [Sub::Leave, Sub::Leave, Sub::Leave],
        },
        //Removing unnecessary hyphens
        //
        R {
            //Text with conti-⏎nuation
            setting: None,
            previous: M::Punctuation(P::Hyphen),
            following: M::Letter(C::Lowercase),
            substitutions: [Sub::Remove, Sub::Remove, Sub::Leave],
        },
        R {
            //Text with Proper-⏎Noun
            setting: Some((S::SmartHyphenRemoval, true)),
            previous: M::Punctuation(P::Hyphen),
            following: M::Letter(C::Uppercase),
            substitutions: [Sub::Leave, Sub::Remove, Sub::Leave],
        },
        R {
            //Text with Proper-⏎Noun
            setting: Some((S::SmartHyphenRemoval, false)),
            previous: M::Punctuation(P::Hyphen),
            following: M::Letter(C::Uppercase),
            substitutions: [Sub::Remove, Sub::Remove, Sub::Leave],
        },
        //Dealing with unusual structures falling on linebreaks
        //
        R {
            // This/That/⏎TheOther
            setting: None,
            previous: M::Punctuation(P::Slash),
            following: M::Letter(C::Any),
            substitutions: [Sub::Leave, Sub::Remove, Sub::Leave],
        },
        R {
            // Text (paranthetical)⏎and more text
            setting: None,
            previous: M::Punctuation(P::Parantheses),
            following: M::Letter(C::Any),
            substitutions: [Sub::Leave, Sub::Replace(" ".to_string()), Sub::Leave],
        },
        R {
            // Text⏎(paranthetical) and more text
            setting: None,
            previous: M::Letter(C::Any),
            following: M::Punctuation(P::Parantheses),
            substitutions: [Sub::Leave, Sub::Replace(" ".to_string()), Sub::Leave],
        },
        R {
            // Text — paranthetical —⏎and more text
            setting: None,
            previous: M::Punctuation(P::Dash),
            following: M::Letter(C::Any),
            substitutions: [Sub::Leave, Sub::Replace(" ".to_string()), Sub::Leave],
        },
        R {
            // Text⏎— paranthetical — and more text
            setting: None,
            previous: M::Letter(C::Any),
            following: M::Punctuation(P::Dash),
            substitutions: [Sub::Leave, Sub::Replace(" ".to_string()), Sub::Leave],
        },
        R {
            // Quote.⏎— Author
            setting: None,
            previous: M::Punctuation(P::EndOfSentence),
            following: M::Punctuation(P::Dash),
            substitutions: [Sub::Leave, Sub::Leave, Sub::Replace("-".to_string())],
        },
        // Miscellaneous replacements
        //
        R {
            // Section.⏎•Bulletpoint
            setting: None,
            previous: M::Punctuation(P::Any),
            following: M::Exact('•'),
            substitutions: [Sub::Leave, Sub::Replace("\n-".to_string()), Sub::Remove],
        },
        R {
            // Marker symbol used to stop a linebreak from being removed erroniously.
            // '꠷' (North Indic Placeholder Mark) is used for its apt name and low
            // probability of being found in the source text.
            setting: None,
            previous: M::Exact('꠷'),
            following: M::Any,
            substitutions: [Sub::Remove, Sub::Leave, Sub::Leave],
        },
    ]
}
