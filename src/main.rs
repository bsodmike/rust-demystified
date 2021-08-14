use monitor::*;

fn main() {
    let disk_space = Monitorable::new(Box::new(MonitorableComponent::DiskSpace));

    // match disk_space.get_context() {
    //     MonitorableComponent::DiskSpace => {
    //         println!("Got Diskspace!");
    //     }
    //     _ => (),
    // }


    let server = Monitorable::new(Box::new(Server { name: "Dell".to_string()}));
    println!("Server name: {:?}", server.get_name());
}
