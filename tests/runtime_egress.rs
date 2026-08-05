use agent_preflight::app::runtime_egress::RuntimeEgressGuard;
use agent_preflight::domain::status::Status;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};

#[test]
fn blocks_private_address_resolved_for_allowed_hostname() {
    let guard = RuntimeEgressGuard::new();
    let resolver = |host: &str| -> Vec<IpAddr> {
        if host == "allowed.example.com" {
            vec![IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))]
        } else {
            vec![]
        }
    };

    let executed = Arc::new(Mutex::new(false));
    let executed_clone = executed.clone();

    let result = guard.connect("allowed.example.com", resolver, || {
        *executed_clone.lock().unwrap() = true;
        Ok(Status::Verified)
    });

    assert_eq!(result, Ok(Status::Failed));
    assert!(!*executed.lock().unwrap());
}

#[test]
fn rechecks_redirect_destination() {
    let guard = RuntimeEgressGuard::new();
    let resolver = |host: &str| -> Vec<IpAddr> {
        if host == "redirect.example.com" {
            vec![IpAddr::V4(Ipv4Addr::new(169, 254, 169, 254))] // Metadata IP
        } else {
            vec![]
        }
    };

    let executed = Arc::new(Mutex::new(false));
    let executed_clone = executed.clone();

    let result = guard.connect("redirect.example.com", resolver, || {
        *executed_clone.lock().unwrap() = true;
        Ok(Status::Verified)
    });

    assert_eq!(result, Ok(Status::Failed));
    assert!(!*executed.lock().unwrap());
}

#[test]
fn rejects_dns_rebinding_sequence() {
    let guard = RuntimeEgressGuard::new();

    // Simulate first resolution returning a valid IP, then a second resolution returning a private IP.
    // The test is that `connect` checks the exact IP right before execution.
    // Wait, connect itself takes the resolver and checks the returned IPs.
    let mut call_count = 0;
    let resolver = move |host: &str| -> Vec<IpAddr> {
        call_count += 1;
        if host == "rebind.example.com" {
            if call_count == 1 {
                vec![IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))] // Rebound to private
            } else {
                vec![IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))]
            }
        } else {
            vec![]
        }
    };

    let executed = Arc::new(Mutex::new(false));
    let executed_clone = executed.clone();

    let result = guard.connect("rebind.example.com", resolver, || {
        *executed_clone.lock().unwrap() = true;
        Ok(Status::Verified)
    });

    assert_eq!(result, Ok(Status::Failed));
    assert!(!*executed.lock().unwrap());
}

#[test]
fn blocks_metadata_redirect() {
    let guard = RuntimeEgressGuard::new();
    let resolver = |host: &str| -> Vec<IpAddr> {
        if host == "metadata.redirect.local" {
            vec![IpAddr::V4(Ipv4Addr::new(169, 254, 169, 254))]
        } else {
            vec![]
        }
    };

    let executed = Arc::new(Mutex::new(false));
    let executed_clone = executed.clone();

    let result = guard.connect("metadata.redirect.local", resolver, || {
        *executed_clone.lock().unwrap() = true;
        Ok(Status::Verified)
    });

    assert_eq!(result, Ok(Status::Failed));
    assert!(!*executed.lock().unwrap());
}

#[test]
fn never_calls_transport_for_rejected_destination() {
    let guard = RuntimeEgressGuard::new();
    let resolver = |host: &str| -> Vec<IpAddr> {
        if host == "bad.example.com" {
            vec![IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))]
        } else {
            vec![]
        }
    };

    let executed = Arc::new(Mutex::new(false));
    let executed_clone = executed.clone();

    let result = guard.connect("bad.example.com", resolver, || {
        *executed_clone.lock().unwrap() = true;
        Ok(Status::Verified)
    });

    assert_eq!(result, Ok(Status::Failed));
    assert!(!*executed.lock().unwrap()); // Verify executor was not called
}
