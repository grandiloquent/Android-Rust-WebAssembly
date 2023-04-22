use std::net::{IpAddr, TcpListener, UdpSocket};

pub fn listen_available_port(start: u16) -> Option<u16> {
    (start..9000).find(|port| port_is_available(*port))
    // let t = thread::spawn(move || {
    //     // 1025
    //     for port in (start..65535) {
    //         match  UdpSocket::bind(("127.0.0.1", port)) {
    //             Ok(l) => {
    //
    //                 drop(l);
    //                 return port;
    //             }
    //             _ => {}
    //         }
    //     }
    //     0
    // });
    //
    //
    // Some(t.join().unwrap())
}

pub fn port_is_available(port: u16) -> bool {
    TcpListener::bind(("127.0.0.1", port)).is_ok()
}

pub fn get_local_ip_address(is_ipv6: bool) -> Option<IpAddr> {
    let socket = match UdpSocket::bind(if is_ipv6 { "[::]:0" } else { "0.0.0.0:0" }) {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect(if is_ipv6 {
        "[2001:4860:4860::8888]:80"
    } else {
        "8.8.8.8:80"
    }) {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => Some(addr.ip()),
        Err(_) => None,
    }
}
