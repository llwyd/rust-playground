use std::net::{ToSocketAddrs, UdpSocket};
use chrono::{Utc, TimeZone};

fn get_ip(address:&str) -> String {
    let mut addrs = address.to_socket_addrs().unwrap();
    addrs.next().expect("Error").to_string()
}

fn calculate_ntp_time(buffer:&[u8]) -> u32 {
    let time_bytes: [u8;4] = buffer[40..44].try_into().expect("Failed to slice");
    u32::from_be_bytes(time_bytes)
}

fn ntp_to_unix(ntp:u32) -> u32 {
    const NTP2UNIX:u32 = ((70 * 365) + 17) * 86400;
    ntp - NTP2UNIX
}

fn print_date(time:u32) -> () {
    let dt = Utc.timestamp_opt(time.into(), 0).unwrap();
    println!("{}", dt);
}

fn main() {
    const PACKET_SIZE:usize = 48;

    let mut buffer: [u8; PACKET_SIZE] = [0; PACKET_SIZE];
    buffer[0] = 0x23;

    let ntp_address = "pool.ntp.org:123";
    let addr = get_ip(ntp_address);

    println!("Requesting NTP from {} ({})", ntp_address, addr);

    let socket = UdpSocket::bind("0.0.0.0:12345").expect("Couldn't bind");

    let bytes_sent = socket.send_to(&buffer[..], addr).expect("Couldn't send");
    assert_eq!(bytes_sent, PACKET_SIZE);

    let (bytes_recv, _) = socket.recv_from(&mut buffer).expect("Nothing received");
    assert_eq!(bytes_recv, PACKET_SIZE);

    let ntp_time = calculate_ntp_time(&buffer[..]);
    let unix_time = ntp_to_unix(ntp_time);

    print_date(unix_time);
}
