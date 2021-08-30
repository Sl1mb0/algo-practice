fn my_atoi_fast(s: String) -> i32 {
    let mut sign: char = '+';
    let mut v: i32 = 0;
    let a: String = s.chars().rev().collect::<String>();
    
    for (i,c) in a.as_str().chars().enumerate() {
        if !(c.is_ascii_digit()) {
            if c != ' ' {
                sign = c;
            }
        }
        else { v += c.to_digit(10).unwrap() as i32 * 10_i32.pow(i as u32) as i32; }
    }

    if sign == '-' { v = 0 - v; }
    v
}

fn my_atoi_small(s: String) -> i32 {
    let mut digits: isize = 0;
    let mut v: i32 = 0;

    match s.find('+') {
        Some(index) => {
            digits = index as isize + 1 },
        None => match s.find('-') {
            Some(index) => { 
                digits = index as isize + 1 },
            None => {}
        },
    }
 
    for c in s.as_str().chars() {
        if c.is_ascii_digit() {
            v += c.to_digit(10).unwrap() as i32 * 10_i32.pow(digits as u32) as i32;
            digits -= 1;
        }
    }

    if s.find('-').is_some() { v = 0-v;}
    v
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let opt: String = (&*args[1]).to_string();
    let time: String  = "time".to_string();
    let space: String = "space".to_string();
    if opt == time {
        my_atoi_fast(i32::MAX.to_string());
    }
    else if opt == space {
        my_atoi_small(i32::MAX.to_string());
    }
}
