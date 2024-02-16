use super::{defaults, settings};
use defaults::SettingType as ST;
use settings::SettingList as SL;

pub fn apply(mut lines: Vec<String>, settings: &SL<ST>) -> Vec<String> {
    lines = headings(lines, settings);
    lines = subheadings(lines, settings);
    lines
}

fn headings(lines: Vec<String>, settings: &SL<ST>) -> Vec<String> {
    let mut outvec = vec![];
    let mut line_iter = lines.into_iter();
    while let Some(mut line) = line_iter.next() {
        let mut heading = String::default();
        let mut clarifier = String::default();

        if !line.chars().next().unwrap_or_default().is_ascii_uppercase() {
            outvec.push(line);
            continue;
        }

        let mut in_clarifier = false;

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
                        heading = format!("{heading} {head}");
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
                    heading = format!("{heading} {line}");
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
                    line = format!("{heading}({clarifier}) {line}");
                    heading = String::default();
                    clarifier = String::default();
                } else {
                    if clarifier.is_empty() {
                        clarifier = clar.to_owned();
                    } else {
                        clarifier = format!("{clarifier} {clar}");
                    }

                    line = tail.to_owned();
                }
                in_clarifier = false;
            } else {
                if clarifier.is_empty() {
                    clarifier = line;
                } else {
                    clarifier = format!("{clarifier} {line}");
                }
                line = line_iter.next().unwrap_or_default();
                in_clarifier = false;
            }
        }
        clarifier = clarifier.trim_start_matches('(').to_owned();

        if settings.check(ST::SimplifiedHeadings) {
            heading = diacritics::remove_diacritics(&heading);
            heading = heading.replace('’', " ");
            clarifier = diacritics::remove_diacritics(&clarifier);
            clarifier = clarifier.replace('’', " ");
        }

        if !settings.check(ST::SeparateHeadingClarifiers) & !clarifier.is_empty() {
            heading = format!("{heading}({clarifier})");
            clarifier = String::default();
        }

        if settings.check(ST::MarkdownSectionHeadings) && !heading.is_empty() {
            heading = "# ".to_owned() + &heading;
            if !clarifier.is_empty() {
                clarifier = format!("**{clarifier}**");
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

fn subheadings(mut lines: Vec<String>, settings: &SL<ST>) -> Vec<String> {
    lines.iter_mut().for_each(|l| {
        let Some((mut subheading, tail)) = l
            .split_once(':')
            .map(|(front, back)| (front.to_owned(), back))
        else {
            return;
        };

        let markdown = settings.check(ST::MarkdownSubheadings);

        if subheading.chars().next().unwrap_or_default() == '•' {
            subheading.trim_start().matches("• ");
            subheading = format!("- {subheading}:");
        } else if all_caps(&subheading) {
            if markdown {
                subheading = format!("- {subheading}:");
            }
        } else {
            return;
        };

        if markdown {
            subheading = format!("**{subheading}**");
        }
        *l = subheading + tail;
    });
    lines
}

fn all_caps(instr: &str) -> bool {
    instr.split_ascii_whitespace().all(|w| {
        (w.chars().next().unwrap_or_default().is_ascii_uppercase() | (w.len() <= 3))
            & w.chars().last().unwrap_or_default().is_ascii_lowercase()
    })
}
