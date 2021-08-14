use monitor::*;

fn main() {
    let monitor_note = String::from("Monitoring StarTech NIC");
    let nic_service_tag = String::from("PEX20000SFPI");
    let nic_description = String::from("StarTech PCIe fiber network card - 2-port open SFP - 10G");
    let nic_mac_address = String::from("00:1B:44:11:3A:B7");
    let network_card = Monitorable::new(
        monitor_note,
        Box::new(NetworkCard {
            description: nic_description.clone(),
            service_tag: nic_service_tag.clone(),
            mac_address: nic_mac_address.clone(),
        }),
    );

    println!("\nNIC Note: {:?}", network_card.get_note());
    println!(
        "NIC Description: {:?}",
        network_card.get_network_card().description
    );
    println!(
        "NIC Mac Address: {:?}",
        network_card.get_network_card().mac_address
    );
    println!();
}
