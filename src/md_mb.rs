use tokio_modbus::prelude::rtu;
use tokio_modbus::Slave;
use tokio_serial::SerialStream;
use tokio_modbus::prelude::*;

//--- use crate::{BAUDRATE, RG_HOLDING_QUANTITY, RG_INPUT_QUANTITY, RgType, SLAVE_NUMB, StRTU, TTY_PATH};
use crate::{RgType, StRTU}; //--- Если использовать функции без констант !

//--- Чтение регистров модбас сразу во внешний вектор цепочкой
//--- от query.rg_start в количестве query.rg_quantity
pub async
fn mb_regs_read_to_vec(query: &StRTU)
   -> Result<Vec<u16>,Box<dyn std::error::Error>> {

    let tty_path = query.connect.port.as_str();
    let slave = Slave(query.connect.slave);
    let builder = tokio_serial::new(tty_path, query.connect.baud);
    let port = SerialStream::open(&builder).expect("Failed to open port!!!");
    let mut ctx = rtu::attach_slave(port, slave);

    let mut rsp = vec!();

    match query.rg_type {
        RgType::input => {
            rsp = ctx
                .read_input_registers(query.rg_start, query.rg_quantity)
                .await?;
        },
        RgType::holding => {
            rsp = ctx
                .read_holding_registers(query.rg_start, query.rg_quantity)
                .await?;
        },
    }

    Ok(rsp)
}

//--- Чтение регистров модбас сразу в вектор цепочкой
//--- от query.rg_start в количестве query.rg_quantity
//--- с распечаткой на экран
//--- ZB: let _ = mb_regs_read(&
pub async
fn mb_regs_read(query: &StRTU)
   -> Result<(),Box<dyn std::error::Error>> {

    let tty_path = query.connect.port.as_str();
    let slave = Slave(query.connect.slave);
    let builder = tokio_serial::new(tty_path, query.connect.baud);
    let port = SerialStream::open(&builder).expect("Failed to open port!!!");
    let mut ctx = rtu::attach_slave(port, slave);

    let mut rsp = vec!();

    match query.rg_type {
        RgType::input => {
            println!("\n<<< Reading {} input registers >>>:\n",query.rg_quantity);
            rsp = ctx.read_input_registers(query.rg_start,query.rg_quantity).await?;
            println!("RG_Input   {:X?}",rsp);
        },
        RgType::holding => {
            println!("\n<<< Reading {} holding registers: >>>\n",query.rg_quantity);
            rsp = ctx.read_holding_registers(query.rg_start,query.rg_quantity).await?;
            println!("RG_Hjlding {:X?}",rsp);
        },
    }

    Ok(())
}

//--- Чтение регистров модбас в цикле по одному в вектор
//--- от query.rg_start всего query.rg_quantity штук <- менее производительно
//--- ZB: let _ = mb_rg_read(&qeri).await;
pub async
fn mb_rg_read(query: &StRTU)
   -> Result<(),Box<dyn std::error::Error>> {

    let tty_path = query.connect.port.as_str();
    let slave = Slave(query.connect.slave);
    let builder = tokio_serial::new(tty_path, query.connect.baud);
    let port = SerialStream::open(&builder).expect("Failed to open port!!!");
    let mut ctx = rtu::attach_slave(port, slave);

    println!("<<< Modbus registers reading >>>\n");

    let mut rsp = vec!();

    match query.rg_type {
        RgType::input => {
            println!("Reading {} input registers:\n",query.rg_quantity);
            for i in query.rg_start..query.rg_quantity { //--- читать по одному Rg за раз
                rsp = ctx.read_input_registers(i,1).await?;
                println!("RG[{:2}] == 0x{:X?}",i,rsp);
            }
        },
        RgType::holding => {
            println!("Reading {} holding registers:\n",query.rg_quantity);
            for i in query.rg_start..query.rg_quantity { //--- читать по одному Rg за раз
                rsp = ctx.read_holding_registers(i,1).await?;
                println!("RG[{:2}] == 0x{:X?}",i,rsp);
            }
        },
    }

    Ok(())
}

/*
//--- Пробные варианты чтения регистров модбас с константными параметрами
//--- в качестве отладки (в работе выгодно применять для неизменных параметров)
//--- ZB: let _ = work_mb().await;
pub async
fn work_mb()
   -> Result<(),Box<dyn std::error::Error>> {
    let tty_path = TTY_PATH;
    let slave = Slave(SLAVE_NUMB);

    let builder = tokio_serial::new(tty_path, BAUDRATE);
    let port = SerialStream::open(&builder).unwrap();
    let mut ctx = rtu::attach_slave(port, slave);

    println!("<<< work_mb() >>>\n");

    let rg_quantity = RG_INPUT_QUANTITY;
    println!("Reading {} input registers:\n", rg_quantity);
    for i in 0..rg_quantity {
        let rsp = ctx.read_input_registers(i, 1).await?;
        println!("RG[{:2}] == 0x{:X?}", i, rsp);
    }

    println!("\n");

    let rg_quantity = RG_HOLDING_QUANTITY;
    println!("Reading {} holding registers:\n", rg_quantity);
    for i in 0..rg_quantity {
        let rsp = ctx.read_holding_registers(i, 1).await?;
        println!("RG[{:2}] == 0x{:X?}", i, rsp);
    }

    Ok(())
}

//--- ZB: let _ = mb_rg_input_read().await;
pub async
fn mb_rg_input_read()
   -> Result<(),Box<dyn std::error::Error>> {
    let tty_path = TTY_PATH;
    let slave = Slave(SLAVE_NUMB);

    let builder = tokio_serial::new(tty_path, BAUDRATE);
    let port = SerialStream::open(&builder).unwrap();
    let mut ctx = rtu::attach_slave(port, slave);

    println!("<<< mb_rg_input_read() >>>\n");

    let rg_quantity = RG_INPUT_QUANTITY;
    println!("Reading {} input registers:\n",rg_quantity);
    for i in 0..rg_quantity {
        let rsp = ctx.read_input_registers(i,1).await?;
        println!("RG[{:2}] == 0x{:X?}",i,rsp);
    }

    Ok(())
}

//--- ZB: let _ = mb_rg_holding_read().await;
pub async
fn mb_rg_holding_read()
   -> Result<(),Box<dyn std::error::Error>> {
    let tty_path = TTY_PATH;
    let slave = Slave(SLAVE_NUMB);

    let builder = tokio_serial::new(tty_path, BAUDRATE);
    let port = SerialStream::open(&builder).unwrap();
    let mut ctx = rtu::attach_slave(port, slave);

    println!("<<< mb_rg_holding_read() >>>\n");

    let rg_quantity = RG_HOLDING_QUANTITY;
    println!("Reading {} holding registers:\n",rg_quantity);
    for i in 0..rg_quantity {
        let rsp = ctx.read_holding_registers(i,1).await?;
        println!("RG[{:2}] == 0x{:X?}",i,rsp);
    }

    Ok(())
}
*/