extern crate simCAN;
use simCAN::Bus;
use simCAN::ecu::ECU;

fn main() {
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 2);
    //println!("2:{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 4);
    //println!("4;{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 6);
    //println!("6:{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 8);
    //println!("8:{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 10);
    //println!("10:{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 12);
    //println!("12:{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 14);
    //println!("14:{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 16);
    //println!("16:{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 18);
    //println!("18:{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 20);
    //println!("20:{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 22);
    //println!("22:{:b}", hoge);
    //let mut hoge: i32 = 0b10000000000000000000000000000000;
    //hoge |= (1 << 2);
    //println!("{:b}", hoge);

    //let bus = Bus::new();
    //let mut ecu1 = ECU::new(1, &bus);
    //let mut ecu2 = ECU::new(2, &bus);
    //ecu1.send_dataframe("hoge");
}

fn test_bus_and_ecu(){
    let bus = Bus::new();
    let mut ecu1 = ECU::new(1, &bus);
    let mut ecu2 = ECU::new(2, &bus);
    ecu1.send("hoge");
    ecu1.receive();
    ecu2.receive();
    ecu2.send("piyo");
    ecu1.receive();
    ecu2.receive();
}
