use crate::domain::status::Status;
use std::net::IpAddr;

pub struct RuntimeEgressGuard;

impl Default for RuntimeEgressGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeEgressGuard {
    pub fn new() -> Self {
        Self
    }

    pub fn check_ip(&self, ip: IpAddr) -> Status {
        if ip.is_loopback() || ip.is_multicast() || ip.is_unspecified() {
            return Status::Failed;
        }
        match ip {
            IpAddr::V4(ipv4) => {
                let octets = ipv4.octets();
                if ipv4.is_private()
                    || ipv4.is_link_local()
                    || octets == [169, 254, 169, 254]
                    // Carrier-grade NAT
                    || (octets[0] == 100 && (octets[1] & 0b1100_0000) == 64)
                {
                    Status::Failed
                } else {
                    Status::Verified
                }
            }
            IpAddr::V6(ipv6) => {
                let segments = ipv6.segments();
                if (segments[0] & 0xfe00) == 0xfc00 // ULA
                    || (segments[0] & 0xffc0) == 0xfe80
                // Link-local
                {
                    Status::Failed
                } else {
                    Status::Verified
                }
            }
        }
    }

    pub fn connect<F, T>(&self, host: &str, mut resolve: F, execute: T) -> Result<Status, String>
    where
        F: FnMut(&str) -> Vec<IpAddr>,
        T: FnOnce() -> Result<Status, String>,
    {
        let ips = resolve(host);
        if ips.is_empty() {
            return Err("Resolution failed".to_string());
        }

        for ip in ips {
            if self.check_ip(ip) == Status::Failed {
                return Ok(Status::Failed);
            }
        }

        execute()
    }
}
