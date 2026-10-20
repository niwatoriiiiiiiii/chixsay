use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

pub(super) fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    // Replace tabs with spaces and remove control characters
    let cleaned: String = text
        .chars()
        .map(|c| if c == '\t' { ' ' } else { c })
        .filter(|c| !c.is_control() || *c == '\n')
        .collect();

    let mut lines = Vec::new();

    for paragraph in cleaned.trim_end_matches('\n').split('\n') {
        let mut current = String::new();
        let mut current_width = 0;

        for g in paragraph.graphemes(true) {
            let w = g.width();
            if current_width + w > max_width && !current.is_empty() {
                lines.push(std::mem::take(&mut current));
                current_width = 0;
            }
            current.push_str(g);
            current_width += w;
        }
        lines.push(current);
    }

    lines
}
