extern crate pcap;
use pcap::{Device,Capture};

#[test]
fn it_works() {
    assert!(true);
}

fn main() {
    let device = Device::lookup().unwrap();
	let mut cap = Capture::from_device(device).unwrap();
	cap = cap.timeout(60000);
	cap = cap.promisc(true);
	cap = cap.snaplen(5000);
	let mut active_cap = cap.open().unwrap();
	active_cap.filter("arp").unwrap();

	while let Ok(packet) = active_cap.next() {
		println!("Received packet! {:?}", packet);
	}
}
