pub enum Case {
    Lowercase,
    Uppercase,
    Anycase,
}

pub enum PunctuationKind {
    AnyPunctuation,
    EndOfSentence,
    Continuation,
    Parantheses,
    Hyphen,
    Dash,
    Slash,
}

pub enum Match {
    Anymatch,
    Exact(char),
    Letter(Case),
    Whitespace,
    Linebreak,
    Punctuation(PunctuationKind),
}

impl Match {
    pub fn matches(&self, candidate: char) -> bool {
        use Case as C;
        use Match as M;
        use PunctuationKind as P;
        match self {
            M::Anymatch => true,
            M::Exact(exact) => candidate == *exact,
            M::Letter(case) => match case {
                C::Uppercase => candidate.is_uppercase(),
                C::Lowercase => candidate.is_lowercase(),
                C::Anycase => candidate.is_alphabetic(),
            },
            M::Whitespace => candidate.is_whitespace(),
            M::Linebreak => candidate == '\n',
            M::Punctuation(punctuation) => match punctuation {
                P::AnyPunctuation => candidate.is_ascii_punctuation(),
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

#[derive(PartialEq)]
pub enum Action {
    Remove,
    Leave,
}

pub struct SymbolPredicate {
    pub symbol: Match,
    pub on_match: Action,
}

impl SymbolPredicate {
    pub fn new(symbol: Match, on_match: Action) -> Self {
        Self { symbol, on_match }
    }
}

pub enum Filler {
    None,
    Space,
    Linebreak,
    Exact(String),
}

impl Filler {
    pub fn get(&self) -> &str {
        use Filler as F;
        match self {
            F::None => "",
            F::Space => " ",
            F::Linebreak => "\n",
            F::Exact(filler) => filler,
        }
    }
}
