use std::error::Error;

use bluest::Adapter;
use tracing::info;
use tracing::metadata::LevelFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found")?;
    adapter.wait_available().await?;

    println!("getting connected devices");
    let devices = adapter.connected_devices().await?;
    for device in devices {
        println!("found {:?}", device);
        adapter.connect_device(&device).await?;
        let services = device.discover_services().await?;
        println!("services: {:?}", services);
        for service in services {
            println!("1");
            println!("  {:?}", service);
            let characteristics = service.discover_characteristics().await?;
            for characteristic in characteristics {
                println!("    {:?}", characteristic);
                let props = characteristic.properties().await?;
                println!("      props: {:?}", props);
                if props.read {
                    println!("      value: {:?}", characteristic.read().await);
                }

                let descriptors = characteristic.discover_descriptors().await?;
                for descriptor in descriptors {
                    println!("2");
                    println!("      {:?}: {:?}", descriptor, descriptor.read().await);
                }
            }
        }
    }
    println!("done");

    Ok(())
}