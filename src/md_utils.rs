//-----------------------------------------------------------------------------
//--- Модуль Utils
//-----------------------------------------------------------------------------
//--- Author: Kornilov LN (Starmark)
//--- Github: https://github.com/KornilovLN
//--- e-mail: ln.KornilovStar@gmail.com
//--- date:   28.09.2023 01:45:00
//-----------------------------------------------------------------------------
//--- 
//--- 
//-----------------------------------------------------------------------------
use std::fs::File;

use std::io;
use std::io::Write;

extern crate num_cpus;

extern crate datetime;
use std::{thread, time};

extern crate chrono;
use chrono::{Datelike, Timelike, Utc};

//use std::fmt;
extern crate ansi_term;
use ansi_term::Colour;

//=================================================================================================
pub fn print_line_char(ch: &str, n: i32) {
    for i in 0..n {
        print!("{}",ch);
    }
    println!();
}

pub fn print_num_line_char(ch: &str, n: i32) {
    for i in 0..n {
        print!("{:4}: {}", i, ch);
    }
    println!();
}

pub fn title_with_border(text: &str) {
    let stroka = text.to_string();
    let mut len = ((stroka.chars().count()) as i32) + 8; //--- так можно. 8 символов "<<< "*2
    //let len = stroka.len() as i32 + 1; //--- так нельзя - ?????????????????

    let frm_ch = format!("{}",Colour::Green.bold().paint("-".to_string()));
    let frm_stroka = format!("{}",Colour::Yellow.bold().paint(stroka));
    let frm_skoba_l = format!("{}",Colour::Green.bold().paint("<<< ".to_string()));
    let frm_skoba_r = format!("{}",Colour::Green.bold().paint(" >>>".to_string()));
    print!("\n\t");
    print_line_char(&frm_ch, len);
    println!("\t{}{frm_stroka}{}",frm_skoba_l, frm_skoba_r);
    print!("\t");
    print_line_char(&frm_ch, len);
}

///--- print simple line
pub fn line_char(ch: &str, len: i32) {
    print!("\t");
    for _ in 0..len-1 {    
        print!("{}",ch);
    }
    println!();
}

//--- text - some title;
//--- ch - simbol line;
//--- len - lenght title-line 
pub fn title_into_line(text: &str, ch: &str, len: i32) {
    let mut text_len = text.chars().count() as i32;
    text_len += 3+1+1;                       //--- ZB: "--- text --------------------"
    let line_len = len - text_len - 1;  //---               ^__________________^    
    let res_stroka = format!("\t{}{}{}{}{} ",ch,ch,ch," ",text);
    print!("{}",res_stroka);
    print_line_char(ch, line_len);
} 

//-----------------------------------------------------------------------------

// --- Получить информацию о железе
pub fn get_num_cpus() -> usize {
	num_cpus::get()
}

pub fn iron() {
    let frm_num_cpus = format!("{}: {}", 
        Colour::Green.paint("Количество ядер процессора: ".to_string()),
        Colour::Yellow.paint(&get_num_cpus().to_string()));
    
    title_into_line("Информация о железе", "-", 45);
    println!("\t{}", frm_num_cpus);
    line_char("-", 45);
}

// --- Получить текущий timestamp
pub fn get_timestamp() -> i64 {
	let dt = Utc::now();
	let timestamp: i64 = dt.timestamp_micros();	
	timestamp		
}

// --- Взять текущее значение Utc и возвратить дату и время
// --- Применять примерно так:
// --- println!("\t{}",Colour::Yellow.paint(dttm));
pub fn get_date_time() -> String {
	let now = Utc::now();
	let (is_common_era, year) = now.year_ce();	
	let (is_pm, hour) = now.hour12();

	format!(
        "{}-{:02}-{:02} {:?} ({})  {:02}:{:02}:{:02} {}",
		year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" },
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    )
}

// --- Задержка
pub fn waiter(pause: u64) {
	thread::sleep(time::Duration::from_secs(pause));
}

// --- Запись в файл
pub fn write_out(f: String, st: &str) -> io::Result<()> {
	let mut out = File::create(f)?;
	write!(out,"{}",st)?;
	Ok(())
} 

// --- Ввод с терминала с выводом подсказки commentn
pub fn read_string(comment:&str) -> String {
    print!("{}", comment);
    let _ = io::stdout().flush();

    let mut string: String = String::new();

    io::stdin().read_line(&mut string)
        .ok()
        .expect("Error read line!");

    return string;
}

const MAX_EL_OUT: i32 = 24;

pub fn out_arr(dt: [u32; 64], prompt: &str){
    print!("{}",prompt);
    let mut cnt = 0;
    for el in &dt {
        print!("{} ", el);
        cnt += 1;
        if cnt > MAX_EL_OUT {
            break;
        } 
    }
    println!("..."); 
} 

pub fn out_vec(dt: Vec<i32>, prompt: &str){
    print!("{}",prompt);
    let mut cnt = 0;
    for el in &dt {
        print!("{} ", el);
        cnt += 1;
        if cnt > MAX_EL_OUT {
            break;
        } 
    }
    println!("..."); 
}

pub fn out_vec_float(dt: &Vec<f32>, prompt: &str){
    print!("{}",prompt);
    let mut cnt = 0;
    for el in dt {
        print!("{} ", el);
        cnt += 1;
        if cnt > MAX_EL_OUT {
            break;
        }
    }
    println!("...");
}


