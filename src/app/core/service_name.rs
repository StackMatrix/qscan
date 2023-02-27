pub fn get_service_name(port: usize) -> Option<String> {
    match port {
        21 => Some("ftp".to_string()),
        22 => Some("ssh".to_string()),
        23 => Some("telnet".to_string()),
        25 => Some("smtp".to_string()),
        53 => Some("dns".to_string()),
        67 => Some("Bootstrap Protocol Server".to_string()),
        68 => Some("Bootstrap Protocol Client".to_string()),
        69 => Some("TFTP".to_string()),
        79 => Some("Finger Service".to_string()),
        80 => Some("http".to_string()),
        110 => Some("pop3".to_string()),
        111 => Some("Remote Procedure Call".to_string()),
        143 => Some("imap".to_string()),
        443 => Some("https".to_string()),
        465 => Some("smtps".to_string()),
        587 => Some("submission".to_string()),
        993 => Some("imaps".to_string()),
        995 => Some("pop3s".to_string()),
        3306 => Some("mysql".to_string()),
        5432 => Some("postgres".to_string()),
        _ => None,
    }
}

