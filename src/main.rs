use std::io::Write;
use std::io::stdout;
use std::process::exit;
use pcap::Device;
use inquire::Select;

fn main() {
    let selected_device:Device;
    let devices = Device::list().unwrap();
    let  device_names = devices
        .iter()
        .map(|device| device.desc.clone().unwrap_or(device.name.clone()))
        .collect::<Vec<String>>();
    let ans = Select::new("Choose a network port:", device_names.clone()).prompt();

    match ans {
        Ok(choice) => {
            let index = device_names.iter().position(|f| *f == choice);

            match index {
                Some(i) => selected_device = devices[i].clone(),
                None => {
                    println!("Error getting Device from List");
                    exit(1)
                }
            }
        },
        Err(_) => println!("Selection canceled"),
    }




}
