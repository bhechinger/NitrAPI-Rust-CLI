#[tokio::main]
async fn main() {
    let ping_result = nitrapi::ping::all().await;

    if let Ok(ping) = ping_result {
        println!("ping\n\t{}: {}", ping.status, ping.message)
    };

    let maintenance_result = nitrapi::maintenance::all().await;

    if let Ok(maintenance) = maintenance_result {
        println!("maintenance\n\tstatus: {}\n\tcloud_backend: {}\n\tdomain_backend: {}\n\tdns_backend: {}\n\tpmacct_backend: {}",
                 maintenance.status,
                 maintenance.data.maintenance.cloud_backend,
                 maintenance.data.maintenance.domain_backend,
                 maintenance.data.maintenance.dns_backend,
                 maintenance.data.maintenance.pmacct_backend
        )
    };

    let version_result = nitrapi::version::all().await;

    if let Ok(ping) = version_result {
        println!("ping\n\t{}: {}", ping.status, ping.message)
    };
}
