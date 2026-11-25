use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::Block;
use ratatui::style::Style;
use ratatui::widgets::{List, ListState};
use crate::api::{SavedSong, pull_data_from_cloudfront};
use crate::tui::guitar_tab::draw_guitar_tab;
use crate::tab;


pub fn draw_test_span(items: &Vec<SavedSong>, client: &mut reqwest::Client) -> color_eyre::Result<()> {
    let mut scroll: u16 = 0; 
    let mut state = ListState::default();
    
    // temp reading from file and deserializing tab
    let body = std::fs::read_to_string("playground/current.json").unwrap();
    let tab = tab::json::load(&body);

    color_eyre::install()?;

    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| {
                let layouts = build_layout(&frame);
                let list = test_render_list(items); 
                draw_guitar_tab(frame, layouts[2], &mut scroll, &tab);
                frame.render_stateful_widget(list, layouts[0], &mut state);
                // test_render_list(frame, layouts[0])
            }).unwrap();
                
                if let Some(key) = event::read().unwrap().as_key_press_event() {
                    if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                        break;
                    } else if key.code == KeyCode::Down {
                        state.select_next();
                    } else if key.code == KeyCode::Up {
                        state.select_previous();
                    } else if key.code == KeyCode::Enter {
                        let cur_item = &items[state.selected().unwrap()];
                        // cloning shi for tokio spawn
                        let mut client = client.clone();
                        let href = cur_item.href.clone();
                        let data_id = cur_item.data_id;
                        tokio::spawn( async move {
                            pull_data_from_cloudfront(&href, data_id, &mut client).await;
                        });
                    } else if key.code == KeyCode::Char('j') {
                        scroll = scroll.saturating_add(1);
                    } else if key.code == KeyCode::Char('k') {
                        scroll = scroll.saturating_sub(1);
                    }

                
            }
        }
    });
    let index = state.selected().unwrap();
    println!("{}", items[index].href);
    Ok(())
}

fn test_render_list<'a>(items: &'a Vec<SavedSong>) ->  List<'a> {
    let title_list: Vec<&str> = items.into_iter().map(|item| item.name.as_str()).collect();
    let list = List::new(title_list)
        .block(Block::bordered().title("List"))
        .highlight_style(Style::new().reversed())
        .repeat_highlight_symbol(true);
    return list;
}

fn build_layout(frame: &Frame) -> [Rect; 4] {
    let outer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ])
        .split(frame.area());

    let left_inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(outer_layout[0]);
    
    let right_inner_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(100),
            Constraint::Percentage(0),
        ])
        .split(outer_layout[1]);
    
   [left_inner_layout[0], left_inner_layout[1], right_inner_layout[0], right_inner_layout[1]] 

}

