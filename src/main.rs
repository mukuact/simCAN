extern crate simCAN;
use simCAN::Bus;
use simCAN::ecu::ECU;

fn main() {
    let bus = Bus::new();
    let mut ecu1 = ECU::new(1, &bus);
    let mut ecu2 = ECU::new(2, &bus);
    ecu1.send_dataframe("hoge");
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
