use std::{
    net::UdpSocket, 
    sync::{
        Mutex,
        Arc,
    },
    thread,
    time::{Instant, Duration},
    collections::HashMap, 

};
use prettytable::{self, row};
use super::scan;

pub fn udp_port_scan(scan_info: &scan::Scan) {
    // 计算时间
    let time = Instant::now(); 
    // 获取本机 cpu 可用线程
    let num_threads = num_cpus::get();
    // 创建一个 hashmap 用于保存 udp 扫描的结果
    let scan_result = Arc::new(Mutex::new(HashMap::new()));
    // socket 绑定
    let socket = Arc::new(UdpSocket::bind("127.0.0.1:0").expect("无法绑定 socket"));

    // 设置扫描的端口范围
    let mut port_range = 0..65535;
    // 创建一个进程句柄
    let mut handles = vec![];
    
    // 非阻塞模式
    socket.set_nonblocking(true).unwrap();
    // 超时断连
    socket.set_read_timeout(Some(Duration::from_secs(2))).unwrap();

    // 判断是否指定端口号
    if scan_info.start_port != 0 || scan_info.end_port != 0 {
        port_range = scan_info.start_port..scan_info.end_port; // 指定端口
    }

    // 根据本机 cpu 可用线程数创建线程
    for i in 0..num_threads {
        // 克隆 hashmap
        let scan_result = scan_result.clone();
        // 克隆端口范围
        let port_range = port_range.clone();
        // 克隆 ip 结构体
        let scan_info = scan_info.addr.clone();
        // 克隆 socket
        let socket = socket.clone();

        // 为当前的循环创建一个线程
        let handle = thread::spawn(move || {
            // 过滤掉已经向其他线程分配过的端口
            for port in port_range.filter(|&p| p % num_threads == i) {
                // 向主机的该端口发送数据，检测是否开启
                socket.send_to(&[0; 10], format!("{}:{}", scan_info, port)).unwrap();
                
                // 用于检测接收值
                match socket.recv_from(&mut [0u8; 1024]) {
                    Ok(_) => {
                        // 获取互斥锁
                        let mut mutex = scan_result.lock().unwrap();
                        
                        // 插入到 hash map
                        match mutex.insert(port, "unknown") {
                            Some(_) => {
                                println!("insert success")
                            },
                            None => {},
                        }
                    },
                    Err(_) => {},
                }
            }
        });

        // 将当前句柄 push 到总线程句柄中
        handles.push(handle);
    }

    // 遍历总线程句柄，等待他们结束
    for handle in handles {
        handle.join().unwrap();
    }

    // 对 hash map 的 port 进行排序
    let mutex = scan_result.lock().unwrap();
    
    // 将 hash map 转换为一个 vec
    let sort = mutex.clone();
    let mut sorted_pairs: Vec<_> = sort.into_iter().collect();
    sorted_pairs.sort_unstable_by_key(|k| k.0);

    println!("使用 {} 个线程扫描 {} 主机的 {}-{} 端口，开放了 {} 端口:", num_threads, scan_info.addr, scan_info.start_port, scan_info.end_port, sorted_pairs.len());

    // 使用 prettytable 包输出表格
    let mut table = prettytable::Table::new();

    // 添加表头
    table.add_row(prettytable::row!["Port", "Protocol", "Service"]);

    // 遍历 hash map
    for (port, service) in sorted_pairs {
        table.add_row(row![port, "open", service]);
    }

    // 打印表格
    table.printstd();

    // 打印扫描时间
    println!("UDP 扫描，花费时间共计: {:?}", time.elapsed());
}


// 原始扫描，使用原始套接字方法
// pub fn raw_scan() {
//     // 获取可用线程数量
//     let num_threads = num_cpus::get();
//     // 设置扫描时间
//     let time = Instant::now();
//     // 进程句柄
//     let mut handles = vec![];
//     // socket
//     let socket = Arc::new(UdpSocket::bind("127.0.0.1:5000").expect("无法绑定"));

//     for i in 0..num_threads {
//         let socket = socket.clone();

//         let fd = socket.as_raw_fd();
//         let handle = thread::spawn(move || {
            

//         });

//         // 将句柄收集
//         handles.push(handle);
//     }

//     // 等待全部句柄结束
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     // 统计运行时间
//     println!("运行时间共计: {:?}", time.elapsed());
// }