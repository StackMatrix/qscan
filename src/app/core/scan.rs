use std::net::IpAddr;

use regex::Regex;

pub struct Scan {
    pub addr: String,
    pub start_port: usize,
    pub end_port: usize,
}

impl Scan {
    pub fn new() -> Self {
        Self { 
            addr: "".to_string(),
            start_port: 0,
            end_port: 0,
        }
    }

    pub fn match_ip(input: String) -> bool {
        // 匹配 ip 地址是否正确
        if let Ok(ip) = input.parse::<IpAddr>() {
            if ip.is_ipv4() {
                println!("开始扫描: {}...", input);
                return  true;
            } else if ip.is_ipv6() {
                println!("开始扫描: {}...", input);
                return  true;
            }
            
            println!("{} 不是一个正确的 ip 地址", input);
            return  false;
        }
        
        println!("{} 不是一个正确的 ip 地址", input);
        return  false;
    }

    pub fn verify_port(&mut self, port: String) -> bool {
        let re = Regex::new(r"^(\d{0,5})-(\d{0,5})$").unwrap();

        if re.is_match(&port) {
            for cap in re.captures_iter(port.as_str()) {
                if cap.len() > 0 {
                    self.start_port = cap[1].parse::<usize>().unwrap();
                    self.end_port = cap[2].parse::<usize>().unwrap();
                }
            }
            return true;
        }
        
        println!("无效匹配，默认将扫描全部端口");
        return false;
    }
}