use rand::prelude::*;

pub struct User {
    pub name: String,
    pub key: String
}

pub fn new(_name: String) -> User {
    User {
        name: _name,
        key: createKey()
    }
}

pub fn empty() -> User {
    User {
        name: String::from(""),
        key: String::from("")
    }
}

impl User {
}


pub fn createKey() -> String {
    let mut rng = rand::thread_rng();
    let validChars: [&str; 26] = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    
    let mut key: String = String::from("");
    
    for i in 0..32 {
        let mut flt: f64 = rng.gen();
        flt *= 26.0;
        key += validChars[flt as usize];
    }
    
    key.to_string()
}