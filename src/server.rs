use crate::{constants::MAX_ALLOWABLE_WORKERS, threadpool::ThreadPool};
use std::net::{TcpListener, TcpStream};

pub struct WebserverConfig {
    max_workers: usize,
    host: String,
    port: String
}

impl WebserverConfig {
    pub fn new(max_workers: usize, host: String, port: String) -> Self {
        assert!(max_workers <= MAX_ALLOWABLE_WORKERS);
        return WebserverConfig {
            max_workers,
            host,
            port
        }
    }
    fn get_addr(&self) -> String {
        let host = &self.host;
        let port = &self.port;
        
        format!("{host}:{port}")
    }
}

pub struct Webserver {
    config: WebserverConfig,
    pool: ThreadPool
}

impl Webserver {
    pub fn new(config: WebserverConfig) -> Webserver {
        let pool = ThreadPool::new(config.max_workers);

        Webserver { config, pool }
    }

    pub fn start(&self, handler: fn(TcpStream)) {
        let address = self.config.get_addr();

        let listener = match TcpListener::bind(address) {
            Ok(tcp_lister) => {
                println!("Server Listening at {}", self.config.get_addr());
                tcp_lister
            },
            Err(error) => panic!("Error Starting Webserver: {:?}", error)
        };

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => self.pool.execute(move || handler(stream)),
                Err(e) => println!("Error With TCP Stream {:?}", e)
            }
        }
    }
}


