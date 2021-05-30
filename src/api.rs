pub struct ApiBuilder {
    host: Option<String>,
    protocol: Option<String>,
    port: Option<u32>,
    timeout: Option<u32>
}

impl ApiBuilder {
    pub fn new() -> Self {
        Self {
            host: None,
            protocol: None,
            port: None,
            timeout: None
        }
    }
    
    pub fn build(self) -> Api {
        Api {
            host: self.host.unwrap_or_else(|| ""),
            protocol: (),
            port: 0,
            timeout: 0
        }
    }
    
    pub fn host(mut self, h: String) -> Self {
        self.host = Some(h);
        self
    }
    
    pub fn protocol(mut self, h: String) -> Self {
        self.protocol = Some(h);
        self
    }
    
    pub fn port(mut self, h: u32) -> Self {
        self.port = Some(h);
        self
    }
    
    pub fn timeout(mut self, h: u32) -> Self {
        self.timeout = Some(h);
        self
    }
}

struct Api {
    host: String,
    protocol: String,
    port: u32,
    timeout: u32
}