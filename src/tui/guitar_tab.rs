use ratatui::widgets::{Paragraph, Block, Padding};
use ratatui::Frame;
use ratatui::layout::{Rect, Layout, Constraint, Direction};
use std::rc::Rc;
use ratatui::text::Line;

pub fn render_guitar_tab<'a>(frame: &mut Frame, area: Rect) {
    let layout = split_guitar_tab_layout_area(area);
    let note_lines = ["e", "B", "G", "D", "A", "E"];
    let mut guitar_tab;
    let dashes = layout[0].width as usize - 10;
    let mut p_text: Vec<Line> = Vec::new();
    let mut line_content;
    for note in &note_lines {
        line_content = format!("{}|{}", &note, "─".repeat(dashes));
        p_text.push(Line::from(line_content).centered());
    }
    for i in 0..6 {
        guitar_tab = Paragraph::new(p_text.clone()).block(Block::default().padding(Padding::new(0, 0, 1, 0)));
        frame.render_widget(guitar_tab, layout[i]);
    }
}

fn split_guitar_tab_layout_area(area:Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
            Constraint::Percentage(16),
        ])
        .split(area)
}
