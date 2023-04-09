use log::info;
use prometheus_exporter::{self, Exporter};

pub struct MonitoringServer {
    ip: [u8; 4],
    port: u16,
    exporter: Option<Exporter>,
}

impl MonitoringServer {
    pub fn new(port: u16) -> Self {
        MonitoringServer {
            ip: [0, 0, 0, 0],
            port,
            exporter: None,
        }
    }

    pub fn start(&mut self) {
        let socket_addr = format!(
            "{}.{}.{}.{}:{}",
            self.ip[0], self.ip[1], self.ip[2], self.ip[3], self.port
        );
        info!("Starting monitoring server on {socket_addr}...");
        let binding = socket_addr
            .parse()
            .expect("Can't parse address for prometheus server!");
        let exporter = prometheus_exporter::start(binding)
            .expect(format!("Unable to start prometheus server in {socket_addr}").as_str());
        self.exporter = Some(exporter);
        info!("Started monitoring server successfully: {socket_addr}.");
    }
}
