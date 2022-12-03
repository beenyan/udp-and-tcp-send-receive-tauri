use std::{
    io::{Read, prelude::*},
    net::{TcpListener, TcpStream, UdpSocket},
};

pub static mut TCP_DATA:String = String::new();
pub static mut UDP_DATA:String = String::new();
pub static mut SCTP_DATA:String = String::new();

pub enum ServiceKind{
    TCP,
    UDP,
    SCTP
}

pub struct Service {
    pub port: u16,
    pub service_kind: ServiceKind,
}

impl Service {
    pub fn form(port:u16,service_kind:ServiceKind) -> Service{
        Service { 
            port,
            service_kind,
        }
    }
    
    #[allow(unused)]
    pub fn listener(&self) -> std::io::Result<()> {
        match self.service_kind {
            ServiceKind::TCP=> {
                let listener = TcpListener::bind( format!("127.0.0.1:{}", self.port))?;

                for stream in listener.incoming() {
                    let mut stream = stream.expect("stream error");
                    
                    println!("incoming connection from: {:?}", stream);

                    unsafe {
                        let mut buf = [0; 1500];
                        let content_size = stream.read(&mut buf).unwrap();
                        let content = format!(
                            "{}",
                            String::from_utf8_lossy(&buf).trim_matches(char::from(0)) 
                        );

                        TCP_DATA = content;
                    }
                }
            },
            ServiceKind::UDP=> {
                let socket = UdpSocket::bind(format!("127.0.0.1:{}", self.port))?;

                unsafe {
                    loop {
                        let mut buf = [0; 1500];
                        let (amt, src) = socket.recv_from(&mut buf)?;
                        let content = format!(
                            "{}",
                            String::from_utf8_lossy(&buf).trim_matches(char::from(0)) 
                        );
                        
                        UDP_DATA = content;
                    }
                }
            },
            ServiceKind::SCTP=> {
            }
        }
        Ok(())
    }

    pub fn send_file(&self, buf: &[u8]) -> std::io::Result<()> {
        match self.service_kind {
            ServiceKind::TCP=> {
                let mut stream = TcpStream::connect(format!("127.0.0.1:{}", self.port))?;
        
                stream.write(buf)?;
            },
            ServiceKind::UDP=> {
                let socket = UdpSocket::bind("127.0.0.1:64430")?;
                socket.connect(format!("127.0.0.1:{}", self.port))?;
                socket.send(buf)?;
            },
            ServiceKind::SCTP=> {
            }
        }

        Ok(())
    }
}