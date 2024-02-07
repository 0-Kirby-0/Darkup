use super::{defaults, settings};

pub fn apply(
    lines: &[String],
    settings: &settings::SettingList<defaults::SettingType>,
) -> Vec<String> {
    let mut outvec = vec![];
    let mut line_iter = lines.iter().map(|l| l.to_owned());
    while let Some(mut line) = line_iter.next() {
        let mut heading = String::default();
        let mut clarifier = String::default(); //trim leading spaces from tables

        if !line.chars().next().unwrap_or_default().is_ascii_uppercase() {
            outvec.push(line);
            continue;
        }

        let mut in_clarifier = false;
        fn all_caps(instr: &str) -> bool {
            instr.split_ascii_whitespace().all(|w| {
                (w.chars().next().unwrap_or_default().is_ascii_uppercase() | (w.len() <= 3))
                    & w.chars().last().unwrap_or_default().is_ascii_lowercase()
            })
        }
        'build_head: while {
            let first = line.chars().next().unwrap_or_default();
            let last = line.chars().last().unwrap_or_default();
            first.is_ascii_uppercase() & !matches!(last, '.' | ',' | '!' | '?')
        } {
            if let Some((head, tail)) = line.split_once('(') {
                if all_caps(head) {
                    if heading.is_empty() {
                        heading = head.to_owned()
                    } else {
                        heading = heading + " " + head;
                    }
                    line = tail.to_owned();
                    in_clarifier = true;
                    break 'build_head;
                } else {
                    break 'build_head;
                }
            } else if all_caps(&line) {
                if heading.is_empty() {
                    heading = line
                } else {
                    heading = heading + " " + &line;
                }
                line = line_iter.next().unwrap_or_default();
            } else {
                break 'build_head;
            }
        }
        in_clarifier |= line.chars().next().unwrap_or_default() == '(';
        while in_clarifier {
            if let Some((clar, tail)) = line.split_once(')') {
                if tail.len() > 1 {
                    line = heading + "(" + &clarifier + ") " + &line;
                    heading = String::default();
                    clarifier = String::default();
                } else {
                    if clarifier.is_empty() {
                        clarifier = clar.to_owned();
                    } else {
                        clarifier = clarifier + " " + clar;
                    }

                    line = tail.to_owned();
                }
                in_clarifier = false;
            } else {
                if clarifier.is_empty() {
                    clarifier = line;
                } else {
                    clarifier = clarifier + " " + &line;
                }
                line = line_iter.next().unwrap_or_default();
                in_clarifier = false;
            }
        }
        clarifier = clarifier.trim_start_matches('(').to_owned();
        use defaults::SettingType as ST;

        if settings.check(ST::MarkdownSubheadings) {
            if let Some((subheading, rest)) = line.split_once(':') {
                line = "- **".to_owned() + subheading + ":**" + rest;
            }
        }

        if settings.check(ST::SimplifiedHeadings) {
            heading = diacritics::remove_diacritics(&heading);
            heading = heading.replace('’', " ");
            clarifier = diacritics::remove_diacritics(&clarifier);
            clarifier = clarifier.replace('’', " ");
        }

        if !settings.check(ST::SeparateHeadingClarifiers) & !clarifier.is_empty() {
            heading = heading + "(" + &clarifier + ")";
            clarifier = String::default();
        }

        if settings.check(ST::MarkdownSectionHeadings) && !heading.is_empty() {
            heading = "# ".to_owned() + &heading;
            if !clarifier.is_empty() {
                clarifier = "**".to_owned() + &clarifier + "**";
            }
        }

        if !heading.is_empty() {
            heading.push('꠷');
            outvec.push(heading);
        }
        if !clarifier.is_empty() {
            clarifier.push('꠷');
            outvec.push(clarifier);
        }
        if !line.is_empty() {
            outvec.push(line);
        }
    }

    outvec
}
