use super::json::{Tab, Bar, Beat};

const STRINGS: [char; 6] = ['e', 'B', 'G', 'D', 'A', 'E'];
const PREFIX: usize = 2;
const CHARS_PER_TICK: usize = 4;


pub fn build(tab: &Tab) -> Vec<Vec<char>>{
    let mut full_string_tab: Vec<Vec<char>>;
    full_string_tab = STRINGS
        .iter()
        .map(
            |s| {
                format!("{s}├").chars().collect() 
        }).collect();
   
    let bar_num = tab.measures.len();
    let mut cur_signature: [u8; 2] = [4, 4];
    for (i, bar) in tab.measures.iter().enumerate() {
        if let Some(sig) = bar.signature {
            cur_signature = sig;
        }
        for (row, tab_part) in full_string_tab.iter_mut().zip(build_bar(bar, i == bar_num - 1, cur_signature)) {
            row.extend(tab_part);
        }
    }
    full_string_tab
}


fn build_bar(bar: &Bar, is_last: bool, signature: [u8; 2]) -> Vec<Vec<char>> {
    let mut bar_str = vec![vec!['─'; PREFIX]; 6]; // create start of two dashes for each string
    let beats: &[Beat] = bar.voices.first().map(|v| {v.beats.as_slice()}).unwrap_or(&[]); 
    
    // searching D - the biggest denominator through all beats + bar signature included
    // to then use for calculating total amount of 'ticks' inside bar
    let biggest_den  = beats.iter()
        .map(|beat| beat.duration[1])
        .chain(std::iter::once(signature[1]))
        .max().unwrap();
   
    let bar_ticks: usize;
    if beats.is_empty() {
        bar_ticks = ticks(signature, biggest_den);
    } else {
        bar_ticks = beats
            .iter()
            .map(|beat| ticks(beat.duration, biggest_den)
        ).sum();
    }

    for s in bar_str.iter_mut() {
        s.extend(std::iter::repeat('─').take(bar_ticks * CHARS_PER_TICK));
        s.push(if is_last { '┤' } else { '┼' });
    }
    
    let mut cursor_ticks: usize = 0;

    for beat in beats {    
        let beat_index = PREFIX + cursor_ticks * CHARS_PER_TICK;

        for note in &beat.notes {
            let Some(fret) = note.fret else { continue };
            let Some(string) = note.string else { continue };
            
            let fret_str = fret.to_string();
            bar_str[string as usize].splice(beat_index..beat_index + fret_str.len(), fret_str.chars());
        }

        cursor_ticks += ticks(beat.duration, biggest_den);
    }

    // let mut string_notes: Vec<char> = format!("{string_name}├").chars().collect();
    // string_notes.extend(std::iter::repeat('─').take(32));
    // string_notes.push(if bar == bar_num - 1 { '┤' } else { '┼' });
    // full_string_tab.push(string_notes);
    
    bar_str
}

fn ticks(sig: [u8; 2], big_den: u8) -> usize {
    (sig[0] * (big_den / sig[1])) as usize
}
