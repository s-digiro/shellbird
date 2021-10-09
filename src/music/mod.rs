use mpd::Client;

pub mod mpd_sender;
pub mod mpd_listener;

fn get_mpd_conn(ip: &str, port: &str) -> Option<Client> {
    match Client::connect(format!("{}:{}", ip, port)) {
        Ok(conn) => Some(conn),
        _ => None,
    }
}

