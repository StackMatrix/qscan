# qscan

## 端口扫描

### tcp 扫描
stack@stackdeMacBook-Pro qscan % sudo cargo run -- -p 0-65535 -t -s 127.0.0.1 
   Compiling qscan v0.1.0 (/Users/stack/Project/rust/qscan)
    Finished dev [unoptimized + debuginfo] target(s) in 1.09s
     Running `target/debug/qscan -p 0-65535 -t -s 127.0.0.1`
||===================================||
||     ____  _____ ____ ___  ____    ||
||    / __ \/ ___/ ___/ __ \/ __ \   ||
||   / /_/ (__  ) /__/ /_/ / / / /   ||
||   \__, /____/\___/\__,_/_/ /_/    ||
||     /_/                           ||
||===================================||
||      mult process scan tools      ||
||===================================||
开始扫描: 127.0.0.1...
使用 8 个线程扫描 127.0.0.1 主机的 0-65535 端口，开放了 9 端口:
+-------+--------+---------+
| Port  | Status | Service |
+-------+--------+---------+
| 3306  | open   | mysql   |
+-------+--------+---------+
| 4301  | open   | unknown |
+-------+--------+---------+
| 4310  | open   | unknown |
+-------+--------+---------+
| 6379  | open   | unknown |
+-------+--------+---------+
| 7890  | open   | unknown |
+-------+--------+---------+
| 7891  | open   | unknown |
+-------+--------+---------+
| 9090  | open   | unknown |
+-------+--------+---------+
| 35600 | open   | unknown |
+-------+--------+---------+
| 55786 | open   | unknown |
+-------+--------+---------+
TCP SYN 扫描，花费时间共计: 1.740997958s

### udp 扫描
stack@stackdeMacBook-Pro qscan % sudo cargo run -- -p 0-65535 -u 127.0.0.1  
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/qscan -p 0-65535 -u 127.0.0.1`
||===================================||
||     ____  _____ ____ ___  ____    ||
||    / __ \/ ___/ ___/ __ \/ __ \   ||
||   / /_/ (__  ) /__/ /_/ / / / /   ||
||   \__, /____/\___/\__,_/_/ /_/    ||
||     /_/                           ||
||===================================||
||      mult process scan tools      ||
||===================================||
开始扫描: 127.0.0.1...
使用 8 个线程扫描 127.0.0.1 主机的 0-65535 端口，开放了 1 端口:
+-------+----------+---------+
| Port  | Protocol | Service |
+-------+----------+---------+
| 64130 | open     | unknown |
+-------+----------+---------+
UDP 扫描，花费时间共计: 3.921859792s


### 为什么 rust 端口扫描的结果与 nmap 不一致
端口扫描工具在扫描时会使用不同的扫描技术和算法，以及使用不同的默认超时时间、扫描方式等，因此在扫描结果上会有一定的差异。下面是一些可能导致 Rust 端口扫描结果与 Nmap 不一致的原因：
超时时间不同：默认情况下，Rust 的 UdpSocket 接收超时时间为 500 毫秒，而 Nmap 默认情况下使用的超时时间为 2 秒。如果 UDP 端口在超时时间内未响应，则 Rust 端口扫描会认为该端口关闭，而 Nmap 则可能会认为该端口为开放状态。
扫描技术不同：Rust 端口扫描使用的是 UDP 发送数据包的方式进行扫描，而 Nmap 可以使用多种扫描技术，如 SYN 扫描、FIN 扫描、NULL 扫描等。这些技术在扫描时会产生不同的包，导致扫描结果有差异。
监听方式不同：Rust 端口扫描是通过发送 UDP 数据包并等待响应的方式进行扫描，而 Nmap 可以使用多种方式进行扫描，如使用原始套接字进行扫描，监听网络流量等。这些方式可能会导致扫描结果有差异。
防火墙影响：Nmap 可以通过多种方式绕过防火墙，如使用分片数据包、使用随机 IP 地址等，而 Rust 端口扫描没有这些绕过防火墙的功能，如果被目标主机的防火墙阻止，则会导致扫描结果不一致。
因此，要确保 Rust 端口扫描的结果与 Nmap 一致，需要针对上述问题进行调整。可以尝试调整 Rust 端口扫描的超时时间、使用其他的扫描技术，或者使用 Nmap 进行比对分析，以找出不一致的原因并进行调整。