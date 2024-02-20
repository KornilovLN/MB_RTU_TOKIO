// SPDX-FileCopyrightText: Copyright (c) 2017-2024 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0
//! Asynchronous RTU client example

extern crate ansi_term;
use ansi_term::Colour;
use std::time::{Duration, Instant};

mod md_utils;
mod md_about;
mod md_concept;
mod md_mb;

use crate::md_mb::mb_regs_read_to_vec;

const TTY_PATH: &str = "/dev/ttyACM0";
const SLAVE_NUMB: u8 = 1;
const BAUDRATE: u32 = 19200;

const RG_INPUT_START: u16 = 0;
const RG_HOLDING_START: u16 = 0;
const RG_INPUT_QUANTITY: u16 = 16;
const RG_HOLDING_QUANTITY: u16 = 23;

const MAXREAD: usize = 10;


enum RgType {
    input,
    holding,
}

#[derive(Clone, Debug)]
pub struct StConnect {
    pub port: String,       //--- TTY_PATH.to_string()
    pub slave: u8,          //--- 1
    pub baud: u32,          //--- 19200
}

pub struct StRTU {
    pub connect: StConnect, //--- Параметры соединения
    pub rg_start: u16,      //--- 0
    pub rg_quantity: u16,   //--- 16 или 23
    pub rg_type: RgType,    //--- input, holding
}


#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    work_info();

    println!();

    //--- Определим, какие регистры будем читать в цикле
    let prt = TTY_PATH;

    let conni: StConnect = crate::StConnect {
        port: prt.to_string(),
        slave: SLAVE_NUMB,
        baud: BAUDRATE,
    };
    let connh = conni.clone();

    let query_i: StRTU = crate::StRTU {
        connect: conni,
        rg_start:RG_INPUT_START,
        rg_quantity:RG_INPUT_QUANTITY,
        rg_type: RgType::input
    };

    let query_h: StRTU = crate::StRTU {
        connect: connh,
        rg_start:RG_HOLDING_START,
        rg_quantity:RG_HOLDING_QUANTITY,
        rg_type: RgType::holding
    };

    //--- Цикл чтения регистров с амеом времени
    let start = Instant::now();

    for _ in 0..MAXREAD {
        let res_i = mb_regs_read_to_vec(&query_i).await;
        println!("RG_Input   {:X?}", res_i);

        let res_h = mb_regs_read_to_vec(&query_h).await;
        println!("RG_Holding {:X?}", res_h);
    }

    let duration = start.elapsed();
    println!("\nTime elapsed is: {}",
             Colour::Yellow.paint(format!("{:?}", duration )));

    Ok(())
}

fn work_info() {
    println!();
    //--- Информация о процессоре, программе и авторе
    md_utils::iron();
    md_about::target("Modbus reader",
        "CLI: Учебно-исследовательская программа");
    md_about::get_json_from_file();

    println!();
    //--- Подробно о программе из файла concept
    let reading_file_result = md_concept::read_concept();
    let file_result = match reading_file_result {
        Ok(file) => file,
        Err(error) => {
            let frm_err = format!("{}: {}",
                Colour::Red.paint("Проблема открытия 'concept.txt'".to_string()),
                Colour::Yellow.paint(error.to_string()) );
            panic!("{}",frm_err);
        },
    };
}

