#[tokio::main]
async fn main() {
    let ping_result = nitrapi::ping::all().await;
    match ping_result {
        Ok(ping) => println!("ping\n\t{}: {}", ping.status, ping.message),
        Err(err) => println!("error pinging: {}", err)
    }

    let maintenance_result = nitrapi::maintenance::all().await;
    match maintenance_result {
        Ok(m) =>
            println!("maintenance\n\tstatus: {}\n\tcloud_backend: {}\n\tdomain_backend: {}\n\tdns_backend: {}\n\tpmacct_backend: {}",
                     m.status,
                     m.data.maintenance.cloud_backend,
                     m.data.maintenance.domain_backend,
                     m.data.maintenance.dns_backend,
                     m.data.maintenance.pmacct_backend
            ),
        Err(err) => println!("error getting maintenance status: {}", err)
    }

    let version_result = nitrapi::version::all().await;
    match version_result {
        Ok(version) => println!("version\n\t{}: {}", version.status, version.message),
        Err(err) => println!("error pinging: {}", err)
    }
}
