use super::json::{Tab, Bar, Voice, Beat, Note};

const STRINGS: [char; 6] = ['e', 'B', 'G', 'D', 'A', 'E'];
pub fn build(tab: &Tab) -> Vec<Vec<char>>{
    let mut full_string_tab: Vec<Vec<char>>;
    full_string_tab = STRINGS
        .iter()
        .map(
            |s| {
                format!("{s}├").chars().collect() 
        }).collect();
    
    for bar in &tab.measures {
        for (row, tab_part) in full_string_tab.iter_mut().zip(build_bar(&bar)) {
            row.extend(tab_part);
        }
    }
    full_string_tab
}


fn build_bar(bar: &Bar) -> Vec<Vec<char>> {
    // let mut string_notes: Vec<char> = format!("{string_name}├").chars().collect();
    // string_notes.extend(std::iter::repeat('─').take(32));
    // string_notes.push(if bar == bar_num - 1 { '┤' } else { '┼' });
    // full_string_tab.push(string_notes);

    vec![]
}
