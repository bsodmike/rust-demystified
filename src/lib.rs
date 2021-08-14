use std::cmp::PartialEq;

#[derive(Debug)]
pub struct DiskSpace;
pub struct FreeMem;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MonitorableComponent {
    DiskSpace,
    FreeMem,
}

/**
 * MonitorableContextClone
 * Sub-trait for MonitorableContext trait object.
 */
pub trait MonitorableContextClone {
    fn clone_box(&self) -> Box<dyn MonitorableContext>;
}

impl<T> MonitorableContextClone for T
where
    T: 'static + MonitorableContext + Clone,
{
    fn clone_box(&self) -> Box<dyn MonitorableContext> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn MonitorableContext> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
// END: MonitorableContextClone


pub trait MonitorableContext: MonitorableContextClone {
    fn access_name(&self) -> String;
}

impl MonitorableContext for MonitorableComponent {
    fn access_name(&self) -> String {
        return String::from("No name");
    }
}

#[derive(Clone)]
pub struct Server {
    pub name: String
}

impl MonitorableContext for Server {
    fn access_name(&self) -> String {
        return self.clone().name;
    }
}

#[derive(Clone)]
pub struct Monitorable {
    context: Box<dyn MonitorableContext>,
    name: String
}

pub trait CanMonitor {
    fn get_context(&self) -> Box<dyn MonitorableContext>;

    fn get_name(&self) -> String;
}

impl Monitorable {
    pub fn new(context: Box<dyn MonitorableContext>) -> Box<dyn MonitorableContext>{
        return context;
    }
}

impl CanMonitor for Box<dyn MonitorableContext> {
    fn get_context(&self) -> Box<dyn MonitorableContext> {
        return self.clone();
    }

    fn get_name(&self) -> String {
        return self.access_name();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_is_composable() {
        let disk_space = Monitorable::new(Box::new(MonitorableComponent::DiskSpace));
        
        let server = Monitorable::new(Box::new(Server { 
            name: "Dell".to_string() 
        }));
        
        assert_eq!(server.get_name(), "Dell");
        assert_eq!(disk_space.get_name(), "No name");
        assert_eq!(server.get_name(), server.get_context().get_name());


        // match server.get_name() {
        //     MonitorableComponent::DiskSpace => {
        //         println!("Got Diskspace!");
        //     }
        //     _ => (),
        // }



        // assert_eq!(disk_space.context, MonitorableComponent::DiskSpace);
        // assert_eq!(disk_space.get_context(), MonitorableComponent::DiskSpace);

        // let free_mem = Monitorable::new(Box::new(MonitorableComponent::FreeMem));
        // assert_eq!(free_mem.context, MonitorableComponent::FreeMem);
        // assert_eq!(free_mem.get_context(), MonitorableComponent::FreeMem);
    }
}
