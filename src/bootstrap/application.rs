use clap::Command;

use crate::app::core::{tcp_scan, tips, scan, udp_scan};

pub fn run(){
    // 初始提示
    tips::init_tips();

    // 获取用户输入命令
    let cmd = Command::new("qscan")
        // .about("一款由 Rust 开发的多线程端口扫描工具")
        .version("1.0.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .author("Stack Boom")
        .arg(
            clap::Arg::new("port")
               .short('p')
               .help("Port rang: <start port>-<end port>
Example: scan -p 0-100 udp 127.0.0.1
If not set range, then default scan all port.")
               .long("port-range")
               .value_parser(clap::builder::NonEmptyStringValueParser::new())
               .action(clap::ArgAction::Set)
               .default_value("0-65535")
        )
        .subcommand(
            Command::new("udp")
            .about("Use UDP method for scan port.")
            .short_flag('u')
            .arg_required_else_help(true)
            .arg(clap::arg!(<IP> "Target IP address."))
        )
        .subcommand(
            Command::new("tcp")
                .about("Use TCP method for scan port.")
                .short_flag('t')
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("syn")
                    .short_flag('s')
                    .about("SYN method.")
                    .arg_required_else_help(true)
                    .arg(clap::arg!(<IP> "Target IP address."))
                )
                .subcommand(
                    Command::new("ack")
                    .short_flag('a')
                    .about("ACK method.")
                    .arg_required_else_help(true)
                    .arg(clap::arg!(<IP> "Target IP address."))
                )
        )
        .get_matches();

    // 实例化结构体
    let mut scan_info = scan::Scan::new();

    // 获取端口
    let port: &String = cmd.get_one("port").expect("required");

    if scan::Scan::verify_port(&mut scan_info, port.to_string()) == false {
        return;
    }

    // 匹配命令是否正确
    match cmd.subcommand() {
        Some(("tcp", sub_matches)) => {
            let type_command = sub_matches.subcommand().unwrap();

            match type_command {
                ("syn", sub_matches) => {
                    let addr = sub_matches.get_one::<String>("IP").expect("required");

                    if scan::Scan::match_ip(addr.to_string()) == false {
                        return;
                    }

                    scan_info.addr = addr.to_string();
                    tcp_scan::syn_port_scan(&scan_info);
                }
                ("ack", sub_matches) => {
                    let ip = sub_matches.get_one::<String>("IP").expect("required");
                    println!("Scan {ip:?} ...");
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        Some(("udp", sub_matches)) => {
            let addr = sub_matches.get_one::<String>("IP").expect("required");

            if scan::Scan::match_ip(addr.to_string()) == false {
                return;
            }

            scan_info.addr = addr.to_string();
            udp_scan::udp_port_scan(&scan_info);
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}