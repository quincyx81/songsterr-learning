use ratatui::widgets::{Paragraph, Block, Padding};
use ratatui::Frame;
use ratatui::layout::{Rect};
use ratatui::text::Line;

const STRINGS: [&str; 6] = ["e", "B", "G", "D", "A", "E"];
const INDENT: usize = 2;

pub fn draw_guitar_tab(frame: &mut Frame, outer: Rect, scroll: &mut u16) {
    let block = Block::default().padding(Padding::new(2, 2, 0, 1));
    let inner = block.inner(outer);
    let lines = build_guitar_tab(inner);
    // clamping scroll 
    *scroll = (*scroll).clamp(0, lines.len().saturating_sub(inner.height as usize) as u16);
    render_guitar_tab(frame, lines, inner, outer, block, scroll);
}

fn build_guitar_tab(area: Rect) -> Vec<Line<'static>> {
    let bar_num = 32; // to be replaced by func argument
    let area_width = area.width as usize;
    
    if area_width == 0 {
        return vec![];
    }

    let mut full_string_tab: Vec<Vec<char>> = vec![];
    for string_name in STRINGS {
        let mut string_notes: Vec<char> = format!("{string_name}├").chars().collect();
        for bar in 0..bar_num {
            string_notes.extend(std::iter::repeat('─').take(32));
            string_notes.push(if bar == bar_num - 1 { '┤' } else { '┼' });
        }
        full_string_tab.push(string_notes);
    }

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
            lines.push(Line::from(chars_for_line));
        }
        lines.push(Line::from(""));
        current_note_progress = end;
        current_row += 1;
    }

    lines
}

fn render_guitar_tab(frame: &mut Frame, lines: Vec<Line>, inner: Rect, outer: Rect, block: Block, scroll: &u16) {
    let end = (*scroll as usize + inner.height as usize).min(lines.len());
    frame.render_widget(block, outer);
    frame.render_widget(
        Paragraph::new(&lines[*scroll as usize..end]),
        inner
    );
}

