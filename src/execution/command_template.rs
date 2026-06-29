//! Shell command template expansion for build and run commands.
//!
//! The reducer accepts shell-style command strings because fuzzing setups often
//! need environment variables, wrappers, flags, or compound commands. This
//! module only substitutes known placeholders. Execution is handled elsewhere.

use std::path::Path;

use crate::error::CodeMinimizerError;

/// Values available to command templates for a single trial side.
#[derive(Clone, Debug)]
pub struct TemplateContext<'a> {
    /// Path to the source file inside the side-specific trial directory.
    pub input: &'a Path,
    /// Directory containing the trial input file and command outputs.
    pub dir: &'a Path,
    /// Stem of the original input file.
    pub stem: &'a str,
    /// Directory reserved for compiler or runner output.
    pub output: &'a Path,
}

/// Expands supported placeholders in a command template.
pub fn expand_template(template: &str, context: &TemplateContext<'_>) -> anyhow::Result<String> {
    let mut expanded = String::with_capacity(template.len() + 32);
    let mut chars = template.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '{' {
            expanded.push(ch);
            continue;
        }

        let mut name = String::new();
        let mut closed = false;
        for next in chars.by_ref() {
            if next == '}' {
                closed = true;
                break;
            }
            name.push(next);
        }

        if !closed {
            return Err(CodeMinimizerError::InvalidCommandTemplate(format!(
                "Unclosed placeholder in template '{template}'"
            ))
            .into());
        }

        let value = match name.as_str() {
            "input" => path_to_string(context.input)?,
            "dir" => path_to_string(context.dir)?,
            "stem" => context.stem.to_owned(),
            "output" => path_to_string(context.output)?,
            _ => return Err(CodeMinimizerError::UnknownPlaceholder(name).into()),
        };

        expanded.push_str(&shell_words::quote(&value));
    }

    Ok(expanded)
}

/// Converts a path to UTF-8 text suitable for command template expansion.
fn path_to_string(path: &Path) -> anyhow::Result<String> {
    path.to_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| anyhow::anyhow!("Command template paths must be valid UTF-8"))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn expand_template_replaces_known_placeholders() {
        let context = TemplateContext {
            input: Path::new("/tmp/case/Test.java"),
            dir: Path::new("/tmp/case"),
            stem: "Test",
            output: Path::new("/tmp/case/out"),
        };

        let command = expand_template(
            "javac {input} -d {output} && java -cp {dir} {stem}",
            &context,
        )
        .unwrap();

        assert!(command.contains("/tmp/case/Test.java"));
        assert!(command.contains("/tmp/case/out"));
        assert!(command.contains("Test"));
    }

    #[test]
    fn expand_template_rejects_unknown_placeholder() {
        let context = TemplateContext {
            input: Path::new("/tmp/input.js"),
            dir: Path::new("/tmp"),
            stem: "input",
            output: Path::new("/tmp/out"),
        };

        let error = expand_template("node {missing}", &context).unwrap_err();
        assert!(
            error.to_string().contains("Unknown command placeholder"),
            "Unexpected error: {error}"
        );
    }
}
