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
   
    let bar_num = tab.measures.len();
    for (i, bar) in tab.measures.iter().enumerate() {
        for (row, tab_part) in full_string_tab.iter_mut().zip(build_bar(&bar, i == bar_num - 1)) {
            row.extend(tab_part);
        }
    }
    full_string_tab
}


fn build_bar(bar: &Bar, is_last: bool) -> Vec<Vec<char>> {
    let mut bar_str = vec![vec!['─'; 2]; 6]; // create start of two dashes for each string
    let beats = &bar.voices[0].beats;
    for i in 0..6 {
        bar_str[i].extend(std::iter::repeat('─').take(beats.len() * 5)); // populate each string
        bar_str[i].push(if is_last { '┤' } else { '┼' });                // with 4 dashes, 2 for
                                                                         // note and 2 for spacing
    }

    for (i, beat) in beats.iter().enumerate() {
        for note in &beat.notes {
            let Some(fret) = note.fret else { continue };
            let Some(string) = note.string else { continue };
            
            let beat_index = 2 + i * 5;
            let fret_str = fret.to_string();

            bar_str[string as usize].splice(beat_index..beat_index + fret_str.len(), fret_str.chars());
        }
    }

    // let mut string_notes: Vec<char> = format!("{string_name}├").chars().collect();
    // string_notes.extend(std::iter::repeat('─').take(32));
    // string_notes.push(if bar == bar_num - 1 { '┤' } else { '┼' });
    // full_string_tab.push(string_notes);
    
    bar_str
}
