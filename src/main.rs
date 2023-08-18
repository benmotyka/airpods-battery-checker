use std::error::Error;

use bluest::Adapter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let adapter = Adapter::default().await.ok_or("Bluetooth adapter not found")?;
    adapter.wait_available().await?;

    println!("getting connected devices");
    let devices = adapter.connected_devices().await?;
    if devices.is_empty() {
        println!("no connected devices");
        return Ok(());
    }
    for device in devices {
        println!("found {}, {:?}", device.name().unwrap(), device);
        adapter.connect_device(&device).await?;
        let services = device.discover_services().await?;
        println!("services: {:?}", services);
        for service in services {
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
                    println!("      {:?}: {:?}", descriptor, descriptor.read().await);
                }
            }
        }
    }

    Ok(())
}