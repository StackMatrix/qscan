use std::{
    sync::{
        Arc,
        Mutex,
    },
    time::Duration,
    collections::HashMap,
    thread, net::{ToSocketAddrs, TcpStream}, 
    time::Instant,
};
use prettytable::{Table, row};
use crate::app::core::{scan, service_name};

pub fn syn_port_scan(scan_info: &scan::Scan) {
    let start = Instant::now(); // 计算时间
    let num_threads = num_cpus::get(); // 获取当前系统上可用的 CPU 数量
    let hash_map = Arc::new(Mutex::new(HashMap::new())); // 创建一个 hashmap

    let mut port_range= 0..65535; // 指定端口;
    let mut handles = vec![]; // 进程句柄
    
    // 判断是否指定端口号
    if scan_info.start_port != 0 || scan_info.end_port != 0 {
        port_range = (scan_info.start_port)..(scan_info.end_port); // 指定端口
    }

    for i in 0..num_threads {  // 创建 cpu 的可用线程数量
        let hash_map = Arc::clone(&hash_map);
        let port_range = port_range.clone(); // 克隆端口范围
        let scan_info = scan_info.addr.clone(); // 复制 IP 地址
        
        let handle = thread::spawn(move || {
            for port in port_range.filter(|&p| p % num_threads == i) {
                let mut mutex = hash_map.lock().unwrap();

                match format!("{}:{}", scan_info.as_str(), port).to_socket_addrs() {
                    Ok(mut addrs) => {
                        match TcpStream::connect_timeout(&addrs.next().unwrap(), Duration::from_secs(2)) {
                            Ok(_) => {
                                let service_name = service_name::get_service_name(port).unwrap_or_else(|| "unknown".to_string());
                                mutex.insert(port, service_name);
                            }
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 对 hash map 的 port 进行排序
    let mutex = hash_map.lock().unwrap();
    
    // 将 hash map 转换为一个 vec
    let sort = mutex.clone();
    let mut sorted_pairs: Vec<_> = sort.into_iter().collect();

    // 对扫描的端口进行排序，因为多线程扫描结果会导致端口乱序
    sorted_pairs.sort_unstable_by_key(|k| k.0);

    println!("使用 {} 个线程扫描 {} 主机的 {}-{} 端口，开放了 {} 端口:", num_threads, scan_info.addr, scan_info.start_port, scan_info.end_port, sorted_pairs.len());

    // 使用 prettytable 包输出表格
    let mut table = Table::new();
    table.add_row(row!["Port", "Status", "Service"]);

    for (port, service) in sorted_pairs {
        table.add_row(row![port, "open", service]);
    }
    table.printstd();

    println!("TCP SYN 扫描，花费时间共计: {:?}", start.elapsed());
}
