use monitor::*;

fn main() {
  let disk_space = Monitorable::<MonitorableComponent>::new(MonitorableComponent::DiskSpace);
  
  match disk_space.get_context() {
    MonitorableComponent::DiskSpace => {
      println!("Got Diskspace!");
    },
    _ => (),
  }
}