use ratatui::widgets::{Paragraph, Block, Padding};
use ratatui::Frame;
use ratatui::layout::{Rect};
use ratatui::text::Line;

const STRINGS: [&str; 6] = ["e", "B", "G", "D", "A", "E"];
const ROW_H: u16 = 7;// 6 strings and one blank line
const INDENT: usize = 2;

pub fn render_guitar_tab(frame: &mut Frame, outer: Rect) {
    let block = Block::default().padding(Padding::new(2, 2, 0, 1));
    let area = block.inner(outer);
    frame.render_widget(block, outer);

    let bar_num = 22; // to be replaced by func argument
    let area_width = area.width as usize;

    if area_width == 0 || area.height < 6 {
        return;
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
    let mut current_note_progress = 0;
    let mut current_row: u16 = 0;
    
    while current_note_progress < total_length {
        let indent_len = if current_row == 0 { 0 } else { INDENT };
        // .max(1) in extreme case when terminal is 2 char long
        let cap = area_width.saturating_sub(indent_len).max(1);
        let end = (current_note_progress + cap).min(total_length);
        
        let y = area.y + current_row * ROW_H;

        if y + 6 > area.bottom() {
            break;
        }

        let mut lines: Vec<Line> = vec![] ;

        for string_notes in &full_string_tab {
            let mut chars_for_line = " ".repeat(indent_len);
            // get string notes from full tab based on current
            // note progress
            chars_for_line.extend(&string_notes[current_note_progress..end]);
            lines.push(Line::from(chars_for_line));

        }
        frame.render_widget(
            Paragraph::new(lines),
            Rect { x: area.x, y, width: (indent_len + end - current_note_progress) as u16, height: 6 },
        );
        current_note_progress = end;
        current_row += 1;
    
    }
}
