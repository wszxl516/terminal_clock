extern crate chrono;
extern crate signal;
extern crate nix;
use std::{fmt, process, str, thread, time};
use chrono::prelude::*;
use nix::sys::signal::{SIGINT};
use signal::trap::Trap;
const COLOR_END: &str = "\x1b[0m";
const COLOR_FG: &str = "\x1b[38;";
const COLOR_BG: &str = "\x1b[48;";
const CLEAR: &str = "\x1b[2J";
const CUR_HIDE: &str = "\x1b[?25l";
const CUR_SHOW: &str = "\x1b[?25h";

/*
const COLORS: [&str; 8] = [
        "Black", "Red",
        "Green", "Yellow",
        "Blue", "Magenta",
        "Cyan", "White"];
*/
const ASC_BLANK: [&str; 9] = [
    "000000000",
    "000000000",
    "000000000",
    "000000000",
    "000000000",
    "000000000",
    "000000000",
    "000000000",
    "000000000"];

const NUM_0: [&str; 9] = [
    "001111100",
    "011000110",
    "011000110",
    "011000110",
    "011000110",
    "011000110",
    "011000110",
    "011000110",
    "001111100"];

const NUM_1: [&str; 9] = [
    "000111000",
    "001111000",
    "00001100",
    "000011000",
    "000011000",
    "000011000",
    "000111000",
    "000111000",
    "011111110"];
const NUM_2: [&str; 9] = [
    "011111110",
    "000000110",
    "000000110",
    "000000110",
    "011111110",
    "011000000",
    "011000000",
    "011000000",
    "011111110"];

const NUM_3: [&str; 9] = [
    "011111110",
    "000000110",
    "000000110",
    "000000110",
    "011111110",
    "000000110",
    "000000110",
    "000000110",
    "011111110"];

const NUM_4: [&str; 9] = [
    "011000110",
    "011000110",
    "011000110",
    "011111110",
    "000000110",
    "000000110",
    "000000110",
    "000000110",
    "000000110"];
const NUM_5: [&str; 9] = [
    "011111110",
    "011000000",
    "011000000",
    "011000000",
    "011111110",
    "000000110",
    "000000110",
    "000000110",
    "011111110"];


const NUM_6: [&str; 9] = [
    "011111110",
    "011000000",
    "011000000",
    "011000000",
    "011111110",
    "011000110",
    "011000110",
    "011000110",
    "011111110"];

const NUM_7: [&str; 9] = [
    "011111110",
    "000000110",
    "000000110",
    "000001100",
    "000001100",
    "000011000",
    "000011000",
    "000011000",
    "000011000"];

const NUM_8: [&str; 9] = [
    "000111000",
    "011000110",
    "011000110",
    "011000110",
    "000111000",
    "011000110",
    "011000110",
    "011000110",
    "000111000"];


const NUM_9: [&str; 9] = [
    "011111110",
    "011000110",
    "011000110",
    "011000110",
    "011111110",
    "000000110",
    "000000110",
    "000000110",
    "011111110"];
const COLON: [&str; 9] = [
    "000000000",
    "000111000",
    "000111000",
    "000000000",
    "000000000",
    "000000000",
    "000111000",
    "000111000",
    "000000000"];

/*fn get_named_color(color: &str, if_bg: bool, light: bool) -> String
{

    let mut color_no: u8 = 30;
    for c in COLORS.iter()
        {
             if c.to_string() == color.to_string()
                {
                    break
                }
            color_no += 1;
        }
    if if_bg == true
        {
            color_no += 10;
        }
    if light == true
        {
            color_no += 60;
        }
    format!("\x1b[{}m", color_no)
}
*/
macro_rules! err {
    ($($arg:tt)*) => {
    println!("{}2;255;0;0m{}{}", COLOR_FG, format!($($arg)*), COLOR_END)
    };
}

macro_rules! exit {
    ($($arg:tt)*) => {
    err!($($arg)*);
    process::exit(0)
    };

}



struct Color{
    red: u8,
    green: u8,
    blue: u8
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}{}RGB: {:02X}{:02X}{:02X}{}", COLOR_FG,
               self.to_string(24), self.red, self.green, self.blue, COLOR_END)
    }
}


impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Color {
        assert!(u8::min_value() <= red && red <= u8::max_value());
        assert!(u8::min_value() <= green && green <= u8::max_value());
        assert!(u8::min_value() <= blue && blue <= u8::max_value());
        Color { red, green, blue }
    }
    fn to_string(&self, color_bit: u8) -> String {

       if color_bit == 8
           {
                let r = ((self.red as f64 / 51.0).round() * 36.0) as u8;
                let g = ((self.green as f64 / 51.0).round() * 6.0) as u8;
                let b = (self.blue as f64 / 51.0).round() as u8;
                format!("5;{}m", 16 + r + g + b)
 
            }
        else if color_bit == 24
            {
                format!("2;{};{};{}m", self.red, self.green, self.blue)
            }
        else
            {
            exit!("color_bit must be 8 or 24!");
            }
    }
    fn clone(&self) -> Color {
        Color::new(self.red, self.green, self.blue)
    }

}


fn cur_move(x: i32, y: i32) -> String {
    format!("\x1b[{};{}H", x, y)
}

fn draw_pixel(color: Color, color1: Color) -> String{
    let pixel1: &str = "\u{2585}";
    format!("{}{}{}{}{}{}", COLOR_FG, color.to_string(24), COLOR_BG, color1.to_string(24) , pixel1, COLOR_END)

}

fn draw_asc(color_fg: Color, color_bg: Color, asc: char, x: i32, y: i32) -> String{
       let pic = match asc {
       '0' => NUM_0,
       '1' => NUM_1,
       '2' => NUM_2,
       '3' => NUM_3,
       '4' => NUM_4,
       '5' => NUM_5,
       '6' => NUM_6,
       '7' => NUM_7,
       '8' => NUM_8,
       '9' => NUM_9,
       ':' => COLON,
        _  => ASC_BLANK
   };
    let mut str_all: String = "".to_string();
    for n in 0..pic.len() {
        let mut yy = y;
        for xx in pic[n].as_bytes() {
            str_all += &cur_move(n as i32 + x, yy);
            if *xx == 49 {
                str_all += &draw_pixel(color_fg.clone(), color_bg.clone());
            }
            else {
                str_all += " ";
            }
            yy += 1;
        }
    }
    str_all
}
fn draw_string(color_fg: Color, color_bg: Color, mut x: i32, y: i32, string: &str) -> String{
        let mut sss: String = "".to_string();
        for ss in string.as_bytes() {
            sss += &draw_asc(color_fg.clone(), color_bg.clone(), *ss as char, y, x);
            x  += 9;

        }
        sss
}

fn main(){
    let trap = Trap::trap(&[SIGINT]);
    loop {
        if let Some(SIGINT) = trap.wait(time::Instant::now()) {
            print!("{}{}", CLEAR, CUR_SHOW);
            process::exit(0)
        }
        let local: DateTime<Local> = Local::now();
        let fg_color: Color = Color::new(0, 0, 255);
        let bg_color: Color = Color::new(0, 0, 255);
        let sleep_time: time::Duration = time::Duration::new(1, 0);
        let s = draw_string(fg_color, bg_color,5,20,
                            &format!("{:02}:{:02}:{:02}", local.hour(), local.minute(), local.second()));
        print!("{}{}{}{}", CLEAR, CUR_HIDE, s, CUR_SHOW);
        thread::sleep(sleep_time)
    }

}