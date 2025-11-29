use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

use camino::{Utf8Path, Utf8PathBuf};

use crate::runtest::ProcRes;

/// Representation of information to invoke a debugger and check its output
pub(super) struct DebuggerCommands {
    /// Commands for the debugger
    pub commands: Vec<String>,
    /// Lines to insert breakpoints at
    pub breakpoint_lines: Vec<usize>,
    /// Contains the source line number to check and the line itself
    check_lines: Vec<(usize, String)>,
    /// Source file name
    file: Utf8PathBuf,
    /// The revision being tested, if any
    revision: Option<String>,
}

impl DebuggerCommands {
    pub fn parse_from(
        file: &Utf8Path,
        debugger_prefix: &str,
        test_revision: Option<&str>,
    ) -> Result<Self, String> {
        let command_directive = format!("{debugger_prefix}-command");
        let check_directive = format!("{debugger_prefix}-check");

        let mut breakpoint_lines = vec![];
        let mut commands = vec![];
        let mut check_lines = vec![];
        let mut counter = 0;
        let reader = BufReader::new(File::open(file.as_std_path()).unwrap());
        for (line_no, line) in reader.lines().enumerate() {
            counter += 1;
            let line = line.map_err(|e| format!("Error while parsing debugger commands: {}", e))?;

            // Breakpoints appear on lines with actual code, typically at the end of the line.
            if line.contains("#break") {
                breakpoint_lines.push(counter);
                continue;
            }

            let Some(after_prefix) = line.trim_start().strip_prefix("//@").map(str::trim_start)
            else {
                continue;
            };

            // Handle revision-specific directives like `//@ [revision] directive`
            // by stripping the revision prefix if present
            let (line_revision, directive_line) =
                if let Some(after_open_bracket) = after_prefix.strip_prefix('[') {
                    if let Some((revision, after_close_bracket)) =
                        after_open_bracket.split_once(']')
                    {
                        (Some(revision), after_close_bracket.trim_start())
                    } else {
                        // Malformed revision prefix, skip this line
                        continue;
                    }
                } else {
                    (None, after_prefix)
                };

            // Only process directives that apply to the current revision:
            // - Directives without a revision prefix apply to all revisions
            // - Directives with a revision prefix only apply when it matches the test revision
            let applies_to_revision = match (test_revision, line_revision) {
                // Directives with a revision prefix only apply to that specific revision
                (Some(test_rev), Some(line_rev)) => test_rev == line_rev,
                // No test revision means we're not running a revisioned test,
                // so directives with revision prefixes shouldn't be processed
                (None, Some(_)) => false,
                // If a directive has no revision prefix, it applies to all revisions
                (_, None) => true,
            };

            if !applies_to_revision {
                continue;
            }

            if let Some(command) = parse_name_value(directive_line, &command_directive) {
                commands.push(command);
            }
            if let Some(pattern) = parse_name_value(directive_line, &check_directive) {
                check_lines.push((line_no, pattern));
            }
        }

        Ok(Self {
            commands,
            breakpoint_lines,
            check_lines,
            file: file.to_path_buf(),
            revision: test_revision.map(str::to_owned),
        })
    }

    /// Given debugger output and lines to check, ensure that every line is
    /// contained in the debugger output. The check lines need to be found in
    /// order, but there can be extra lines between.
    pub fn check_output(&self, debugger_run_result: &ProcRes) -> Result<(), String> {
        // (src_lineno, ck_line)  that we did find
        let mut found = vec![];
        // (src_lineno, ck_line) that we couldn't find
        let mut missing = vec![];
        //  We can find our any current match anywhere after our last match
        let mut last_idx = 0;
        let dbg_lines: Vec<&str> = debugger_run_result.stdout.lines().collect();

        for (src_lineno, ck_line) in &self.check_lines {
            if let Some(offset) = dbg_lines
                .iter()
                .skip(last_idx)
                .position(|out_line| check_single_line(out_line, &ck_line))
            {
                last_idx += offset;
                found.push((src_lineno, dbg_lines[last_idx]));
            } else {
                missing.push((src_lineno, ck_line));
            }
        }

        if missing.is_empty() {
            Ok(())
        } else {
            let fname = self.file.file_name().unwrap();
            let revision_suffix =
                self.revision.as_ref().map_or(String::new(), |r| format!("#{}", r));
            let mut msg = format!(
                "check directive(s) from `{}{}` not found in debugger output. errors:",
                self.file, revision_suffix
            );

            for (src_lineno, err_line) in missing {
                write!(msg, "\n    ({fname}:{num}) `{err_line}`", num = src_lineno + 1).unwrap();
            }

            if !found.is_empty() {
                let init = "\nthe following subset of check directive(s) was found successfully:";
                msg.push_str(init);
                for (src_lineno, found_line) in found {
                    write!(msg, "\n    ({fname}:{num}) `{found_line}`", num = src_lineno + 1)
                        .unwrap();
                }
            }

            Err(msg)
        }
    }
}

/// Split off from the main `parse_name_value_directive`, so that improvements
/// to directive handling aren't held back by debuginfo test commands.
fn parse_name_value(line: &str, name: &str) -> Option<String> {
    if let Some(after_name) = line.strip_prefix(name)
        && let Some(value) = after_name.strip_prefix(':')
    {
        Some(value.to_owned())
    } else {
        None
    }
}

/// Check that the pattern in `check_line` applies to `line`. Returns `true` if they do match.
fn check_single_line(line: &str, check_line: &str) -> bool {
    // Allow check lines to leave parts unspecified (e.g., uninitialized
    // bits in the  wrong case of an enum) with the notation "[...]".
    let line = line.trim();
    let check_line = check_line.trim();
    let can_start_anywhere = check_line.starts_with("[...]");
    let can_end_anywhere = check_line.ends_with("[...]");

    let check_fragments: Vec<&str> =
        check_line.split("[...]").filter(|frag| !frag.is_empty()).collect();
    if check_fragments.is_empty() {
        return true;
    }

    let (mut rest, first_fragment) = if can_start_anywhere {
        let Some(pos) = line.find(check_fragments[0]) else {
            return false;
        };
        (&line[pos + check_fragments[0].len()..], 1)
    } else {
        (line, 0)
    };

    for current_fragment in &check_fragments[first_fragment..] {
        let Some(pos) = rest.find(current_fragment) else {
            return false;
        };
        rest = &rest[pos + current_fragment.len()..];
    }

    can_end_anywhere || rest.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_single_line_exact_match() {
        assert!(check_single_line("$1 = 42", "$1 = 42"));
        assert!(!check_single_line("$1 = 42", "$1 = 43"));
    }

    #[test]
    fn test_check_single_line_with_wildcard() {
        assert!(check_single_line("$1 = 42 (some extra stuff)", "$1 = 42[...]"));
        assert!(check_single_line("prefix $1 = 42 suffix", "[...]$1 = 42[...]"));
        assert!(!check_single_line("$1 = 43", "$1 = 42[...]"));
    }

    #[test]
    fn test_parse_name_value() {
        assert_eq!(parse_name_value("gdb-command:run", "gdb-command"), Some("run".to_string()));
        assert_eq!(parse_name_value("gdb-command:", "gdb-command"), Some("".to_string()));
        assert_eq!(parse_name_value("gdb-check:$1 = 42", "gdb-check"), Some("$1 = 42".to_string()));
        assert_eq!(parse_name_value("lldb-command:run", "gdb-command"), None);
        assert_eq!(parse_name_value("gdb-command run", "gdb-command"), None);
    }
}
