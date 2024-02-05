use crate::defaults;
use crate::settings;

pub enum Case {
    Lowercase,
    Uppercase,
    Any,
}

pub enum PunctuationKind {
    Any,
    EndOfSentence,
    Continuation,
    Parantheses,
    Hyphen,
    Dash,
    Slash,
}

pub enum Match {
    Any,
    Exact(char),
    Letter(Case),
    Whitespace,
    Linebreak,
    Punctuation(PunctuationKind),
}

impl Match {
    fn matches(&self, candidate: char) -> bool {
        use Case as C;
        use Match as M;
        use PunctuationKind as P;
        match self {
            M::Any => true,
            M::Exact(exact) => candidate == *exact,
            M::Letter(case) => match case {
                C::Uppercase => candidate.is_uppercase(),
                C::Lowercase => candidate.is_lowercase(),
                C::Any => candidate.is_alphabetic(),
            },
            M::Whitespace => candidate.is_whitespace(),
            M::Linebreak => candidate == '\n',
            M::Punctuation(punctuation) => match punctuation {
                P::Any => candidate.is_ascii_punctuation(),
                P::EndOfSentence => matches!(candidate, '.' | '!' | '?'),
                P::Continuation => matches!(candidate, ',' | ':' | ';'),
                P::Parantheses => matches!(candidate, '(' | ')' | '[' | ']'),
                P::Hyphen => candidate == '-',
                P::Dash => candidate == '—',
                P::Slash => candidate == '/',
            },
        }
    }
}
pub enum Substitute {
    Replace(String),
    Remove,
    Leave,
}

impl Substitute {
    fn apply(&self, candidate: char) -> String {
        match self {
            Self::Replace(replacement) => replacement.clone(),
            Self::Remove => String::new(),
            Self::Leave => candidate.to_string(),
        }
    }
}

pub struct Rule {
    pub setting: Option<(defaults::SettingType, bool)>,
    pub previous: Match,
    pub following: Match,

    pub substitutions: [Substitute; 3],
}

impl Rule {
    fn apply(&self, previous: char, following: char) -> Option<String> {
        if self.previous.matches(previous) && self.following.matches(following) {
            Some(
                self.substitutions[0].apply(previous)
                    + &self.substitutions[1].apply('\n')
                    + &self.substitutions[2].apply(following),
            )
        } else {
            None
        }
    }
}
