extern crate chrono;
extern crate signal;
extern crate nix;
use std::{fmt, process, str, thread, time, env};
use std::cmp::Ordering::Equal;
use std::collections::HashMap;
use chrono::prelude::*;
use nix::sys::signal::{SIGINT};
use signal::trap::Trap;
const COLOR_END: &str = "\x1b[0m";
const COLOR_FG: &str = "\x1b[38;";
const COLOR_BG: &str = "\x1b[48;";
const CLEAR: &str = "\x1b[2J";
const CUR_HIDE: &str = "\x1b[?25l";
const CUR_SHOW: &str = "\x1b[?25h";

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
    "011000110",
    "011111110",
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


/*
const COLORS: [&str; 8] = [
        "Black", "Red",
        "Green", "Yellow",
        "Blue", "Magenta",
        "Cyan", "White"];
fn get_named_color(color: &str, if_bg: bool, light: bool) -> String
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

fn str2hex(str_num: &str) -> u8{
    let mut num:u8 = 0;
        for i in 0..256 {
        if str_num.cmp(&format!("{}", i).as_str()) == Equal ||
            str_num.cmp(&format!("{:x?}", i).as_str()) == Equal ||
            str_num.cmp(&format!("{:X?}", i).as_str()) == Equal{
            num = i as u8
        }
    }
    return num
}

fn str2num(str_num: &str) -> i32{
    let mut num:i32 = 0;
        for i in 0..256 {
        if str_num.cmp(&format!("{}", i).as_str()) == Equal{
            num =  i as i32
        }
    }
    return num
}

fn is_number(s: &String) -> bool{
    for ss in s.as_bytes(){
        if *ss > 57 || *ss < 48{
            return false
        }
    }
    true
}


struct Arguments(HashMap<String, String>);
impl Arguments
{
    fn parse(&self) -> Result<Arguments, &str> {
        let args: Vec<String> = env::args().skip(1).collect();
        let mut arguments: HashMap<String, String> = HashMap::new();
        let help = "Usage:\n\tcolor=ff00ff or color=255000255.\n\tx=20 y=5";
        for arg in args {
            let argument: Vec<&str> = arg.splitn(2, "=").collect();
            let name = argument[0].trim();
            if name.cmp(&"help") == Equal ||
                name.cmp(&"--help") == Equal ||
                name.cmp(&"-h") == Equal||
                argument.len() < 2 ||
                name.is_empty()||
                arguments.contains_key(name)
                {
                    return Err(help)
                }
            else {
                let value = argument[1].trim_matches(|c: char| c.is_whitespace() || c == '"' || c == '\'');
                arguments.insert(name.to_string(), value.to_string());
            }

        }
        Ok(Arguments(arguments))
    }
    fn get_value(&self, key: &str) -> String{
        let mut value:&str = " ";
            if self.0.contains_key(key){
                    value = &self.0[key];
                }
            value.to_string()
    }
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

    fn from_string(mut str_color: &str) -> Result<Color, &str>{
        if str_color.cmp(&" ") == Equal{
            str_color = "ff0088"
        }
        if str_color.len() == 6 {
            let (r, gb): (&str, &str) = str_color.split_at(2);
            let (g, b): (&str, &str) = gb.split_at(2);
            Ok(Color::new(str2hex(r), str2hex(g), str2hex(b)))
        }
        else {
            Err("Invalid Value: Color must be hex numbers of 6 bits.")
        }
    }

}


fn cur_move(x: i32, y: i32) -> String { format!("\x1b[{};{}H", x, y) }

fn draw_pixel(color: Color, color1: Color) -> String{
    let pixel1: &str = "\u{2587}";
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


fn get_start_point(args: Arguments) -> (i32, i32){
    let x = args.get_value("x");
    let y = args.get_value("y");
    if is_number(&x) && is_number(&y){
        let xx = str2num(&x);
        let yy = str2num(&y);
        return (xx, yy)
    }
    err!("x and y must be a number use default value (20, 5)!");
    (20, 5)
}

fn main(){
    let _args: HashMap<String, String> = HashMap::new();
    let args = Arguments(_args).parse().unwrap_or_else(|error| { exit!("{}", error);});
    let color_str = args.get_value("color");
    let color = Color::from_string(&color_str).unwrap_or_else(|error| { exit!("{}", error);});
    let (x, y): (i32, i32) = get_start_point(args);
    let trap = Trap::trap(&[SIGINT]);
    let sleep_time: time::Duration = time::Duration::from_millis(900);
    loop {
        if let Some(SIGINT) = trap.wait(time::Instant::now()) {
            print!("{}{}", CLEAR, CUR_SHOW);
            process::exit(0)
        }
        let local: DateTime<Local> = Local::now();
        let fg_color: Color = color.clone();
        let bg_color: Color = Color::new(0, 0, 0);
        let s = draw_string(fg_color, bg_color,x as i32, y as i32,
                            &format!("{:02}:{:02}:{:02}", local.hour(), local.minute(), local.second()));
        print!("{}{}{}", CUR_HIDE, s, CUR_SHOW);
        thread::sleep(sleep_time)
    }

}
