// Generic TCP Server Implementation
use std::net;
use std::thread;

use std::io::Read;
use std::io::Write;

fn handle(mut stream: net::TcpStream) {
	let mut buffer;
	loop {
		buffer = [0; 1024];
		let _ = match stream.read(&mut buffer) {
			Err(e) => panic!("Error on read: {}", e),
			Ok(message) => {
				if message == 0 {
					break; // EOF
				}
				message // return message. Leaving this open for further implementation - Paul
			},
		};

		match stream.write(&buffer) {
			Err(e) => break,
			Ok(_) => continue,
		}
	}
}

pub fn listen(listen_addr: net::SocketAddr) {
	let listener = net::TcpListener::bind(listen_addr).unwrap();
	println!("Listening started on {}", listen_addr);
    for stream in listener.incoming() {
    	match stream {
    		Err(e) => { println!("Error on listening: {}", e) }
    		Ok(stream) => {
    			thread::spawn(move || {
    				handle(stream)
    			});
    		}
    	}
	}
}

#[cfg(test)]
mod test {
  use std::net;
  use std::thread;
  use super::*;

  #[test]
  fn test_tcp() {
    println!("TCP server test");
    let ip = net::Ipv4Addr::new(127, 0, 0, 1);
    let listen_addr = net::SocketAddrV4::new(ip, 8888);
    let listener = listen(net::SocketAddr::V4(listen_addr));

    // TODO: Add test - Paul
  }
}