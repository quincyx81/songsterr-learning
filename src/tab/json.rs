use serde::Deserialize;

#[derive(Deserialize)]
pub struct Tab {
    pub measures: Vec<Bar>
}

#[derive(Deserialize)]
pub struct Bar {
    pub signature: Option<[u8; 2]>,
    pub voices: Vec<Voice> 
} 

#[derive(Deserialize)]
pub struct Voice {
    pub beats: Vec<Beat> 
}

#[derive(Deserialize)]
pub struct Beat {
    pub notes: Vec<Note>,
    pub duration: [u8; 2]
}

#[derive(Deserialize)]
pub struct Note {
   pub string: Option<u8>,
   pub fret: Option<u8>
}

pub fn load(body: &str) -> Tab {
    serde_json::from_str::<Tab>(body).unwrap()
}
