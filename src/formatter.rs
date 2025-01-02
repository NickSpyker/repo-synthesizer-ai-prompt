use std::path::Path;

pub fn format(is_first: bool, relative_path: &Path, content: &str) -> String {
    let mut output: String = String::new();

    if !is_first {
        output.push_str(format!("\n{}\n\n", "-".repeat(100)).leak());
    }

    output.push_str(if content.is_empty() {
        format!("{}: (EMPTY FILE)", relative_path.display()).leak()
    } else {
        format!("{}:\n{content}", relative_path.display()).leak()
    });

    output
}
