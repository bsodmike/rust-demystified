#![allow(unused_doc_comments)]

use std::any::Any;
use std::cmp::PartialEq;

#[derive(Debug)]
pub struct DiskSpace;
pub struct FreeMem;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MonitorableComponent {
    DiskSpace,
    FreeMem,
}

pub trait MonitorableContext: MonitorableContextClone {
    fn as_any(&self) -> &dyn Any;
    fn access_description(&self) -> Option<String>;
    fn access_service_tag(&self) -> Option<String>;
}

/**
 * MonitorableContextClone
 * Supertrait for MonitorableContext.
 * Ref: https://doc.rust-lang.org/rust-by-example/trait/supertraits.html?highlight=sub-traits#supertraits
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

/**
 * Implmement trait MonitorableContext
 */
impl MonitorableContext for MonitorableComponent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn access_description(&self) -> Option<String> {
        return None;
    }

    fn access_service_tag(&self) -> Option<String> {
        return None;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Server {
    pub description: String,
    pub service_tag: String,
    pub build_date: String,
}

impl MonitorableContext for Server {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn access_description(&self) -> Option<String> {
        return Some(self.clone().description);
    }

    fn access_service_tag(&self) -> Option<String> {
        return Some(self.clone().service_tag.to_string());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkCard {
    pub description: String,
    pub service_tag: String,
    pub mac_address: String,
}

impl MonitorableContext for NetworkCard {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn access_description(&self) -> Option<String> {
        return Some(self.clone().description);
    }

    fn access_service_tag(&self) -> Option<String> {
        return Some(self.clone().service_tag.to_string());
    }
}

#[derive(Clone)]
pub struct Monitorable {
    context: Box<dyn MonitorableContext>,
    note: String,
}

pub trait CanMonitor {
    fn get_context(&self) -> Box<dyn MonitorableContext>;
    fn get_description(&self) -> Option<String>;
    fn get_service_tag(&self) -> Option<String>;
}

impl Monitorable {
    pub fn new(note: String, context: Box<dyn MonitorableContext>) -> Self {
        return Self {
            context: context,
            note: note,
        };
    }

    // This constructor has only been included to demonstrate that we can return the concrete-type
    // at invocation if we wanted to; however, it makes more sense to access this through
    // an instance of `Monitorable` itself.
    pub fn new_for_context(context: Box<dyn MonitorableContext>) -> Box<dyn MonitorableContext> {
        return context;
    }
}

pub trait CanMonitorShared {
    fn get_context(&self) -> &Box<dyn MonitorableContext>;
    fn get_note(&self) -> &String;
    fn get_server(&self) -> &Server;
    fn get_network_card(&self) -> &NetworkCard;
}

impl CanMonitorShared for Monitorable {
    fn get_context(&self) -> &Box<dyn MonitorableContext> {
        return &self.context;
    }

    fn get_note(&self) -> &String {
        return &self.note;
    }

    fn get_server(&self) -> &Server {
        return self
            .context
            .as_any()
            .downcast_ref::<Server>()
            .expect("This should be a nice server");
    }

    fn get_network_card(&self) -> &NetworkCard {
        return self
            .context
            .as_any()
            .downcast_ref::<NetworkCard>()
            .expect("This should be a well behaved NIC");
    }
}

impl CanMonitor for Box<dyn MonitorableContext> {
    fn get_context(&self) -> Box<dyn MonitorableContext> {
        return self.clone();
    }

    fn get_description(&self) -> Option<String> {
        return self.access_description();
    }

    fn get_service_tag(&self) -> Option<String> {
        return self.access_service_tag();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_composable() {
        /**
         * Here we test returning the concrete type at run-time
         */
        let server_tag = String::from("001");
        let server_description = String::from("Dell PowerEdge R740xd");
        let disk_space = Monitorable::new_for_context(Box::new(MonitorableComponent::DiskSpace));

        match disk_space.get_description() {
            None => {
                assert!(true)
            }
            _ => {}
        }

        let free_mem = Monitorable::new_for_context(Box::new(MonitorableComponent::FreeMem));

        match free_mem.get_description() {
            None => {
                assert!(true)
            }
            _ => {}
        }

        let server = Monitorable::new_for_context(Box::new(Server {
            description: server_description.clone(),
            service_tag: server_tag.clone(),
            build_date: String::from("01JAN2021"),
        }));

        if let Some(i) = server.get_description() {
            assert_eq!(i, server_description);
        }

        /**
         * This would be the preferred invocation approach, as the return type is that of `Monitorable`
         */
        let mut monitor_note = String::from("Monitoring Dell Server");
        let server_build_date = String::from("01JAN2021");
        let server2 = Monitorable::new(
            monitor_note.clone(),
            Box::new(Server {
                description: server_description.clone(),
                service_tag: server_tag.clone(),
                build_date: server_build_date.clone(),
            }),
        );

        assert_eq!(*server2.get_note(), monitor_note);

        let server2_service_tag = server2.get_context().access_service_tag();
        if let Some(i) = server2_service_tag {
            assert_eq!(i, server_tag);
        }

        assert_eq!(*server2.get_server().build_date, server_build_date);

        monitor_note = String::from("Monitoring StarTech NIC");
        let nic_service_tag = String::from("PEX20000SFPI");
        let nic_description =
            String::from("StarTech PCIe fiber network card - 2-port open SFP - 10G");
        let nic_mac_address = String::from("00:1B:44:11:3A:B7");
        let network_card = Monitorable::new(
            monitor_note,
            Box::new(NetworkCard {
                description: nic_description.clone(),
                service_tag: nic_service_tag.clone(),
                mac_address: nic_mac_address.clone(),
            }),
        );

        assert_eq!(
            *network_card.get_network_card().mac_address,
            nic_mac_address
        );
    }

    #[test]
    fn use_of_enums() {
        let note = String::from("Monitoring of disk space");
        let disk_space = Monitorable::new(note.clone(), Box::new(MonitorableComponent::DiskSpace));

        let context = disk_space.get_context();
        let disk_space_variant = context
            .as_any()
            .downcast_ref::<MonitorableComponent>()
            .expect("This should be a DiskSpace variant");

        assert_eq!(disk_space.note, note);

        if let Some(i) = Some(disk_space_variant) {
            assert_eq!(*i, MonitorableComponent::DiskSpace)
        }

        // We could test the above in this manner as well, although not as succint.
        match disk_space_variant {
            MonitorableComponent::DiskSpace => {
                assert!(true);
            }
            _ => {}
        }
    }
}
