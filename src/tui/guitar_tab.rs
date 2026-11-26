use crate::{tab, tab::json::Tab};
use ratatui::widgets::{Paragraph, Block, Padding};
use ratatui::style::{Color, Modifier, Style};
use ratatui::Frame;
use ratatui::layout::{Rect};
use ratatui::text::{Line, Span};

const INDENT: usize = 2;

pub fn draw_guitar_tab(frame: &mut Frame, outer: Rect, scroll: &mut u16, tab: &Tab) {
    let block = Block::default().padding(Padding::new(2, 2, 0, 1));
    let inner = block.inner(outer);
    let lines = build_guitar_tab(inner, tab);
    // clamping scroll 
    *scroll = (*scroll).clamp(0, lines.len().saturating_sub(inner.height as usize) as u16);
    render_guitar_tab(frame, lines, inner, outer, block, scroll);
}

fn build_guitar_tab(area: Rect, tab: &Tab) -> Vec<Line<'static>> {
    let area_width = area.width as usize;
    
    if area_width == 0 {
        return vec![];
    }
    
    let full_string_tab: Vec<Vec<char>> = tab::layout::build(tab);
    
    let total_length = full_string_tab[0].len();
    let mut lines: Vec<Line> = vec![] ;
    let mut current_note_progress = 0;
    let mut current_row: u16 = 0;
    
    while current_note_progress < total_length {
        let indent_len = if current_row == 0 { 0 } else { INDENT };
        // .max(1) in extreme case when terminal is 2 char long
        let cap = area_width.saturating_sub(indent_len).max(1);
        let end = (current_note_progress + cap).min(total_length);
        
        for string_notes in &full_string_tab {
            let mut chars_for_line = " ".repeat(indent_len);
            // get string notes from full tab based on current
            // note progress
            chars_for_line.extend(&string_notes[current_note_progress..end]);
            // lines.push(Line::from(chars_for_line));
            lines.push(style_row(chars_for_line.chars().collect()));
        }
        lines.push(Line::from(""));
        current_note_progress = end;
        current_row += 1;
    }

    lines
}
fn style_row(row: Vec<char>) -> Line<'static> {
    let spans: Vec<Span<'static>> = row
        // group neighbors with the same style into one Span
        .chunk_by(|a, b| style_for(*a) == style_for(*b))
        .map(|chunk| Span::styled(chunk.iter().collect::<String>(), style_for(chunk[0])))
        .collect();

    Line::from(spans)
}

fn style_for(c: char) -> Style {
    match c {
        '─' | '┼' | '├' | '┤' => Style::new().fg(Color::Indexed(247)), // dim lines
        '0'..='9' => Style::new().add_modifier(Modifier::BOLD),
        _ => Style::new(), // string names
    }
}

fn render_guitar_tab(frame: &mut Frame, lines: Vec<Line>, inner: Rect, outer: Rect, block: Block, scroll: &u16) {
    let end = (*scroll as usize + inner.height as usize).min(lines.len());
    frame.render_widget(block, outer);
    frame.render_widget(
        Paragraph::new(&lines[*scroll as usize..end]),
        inner
    );
}

