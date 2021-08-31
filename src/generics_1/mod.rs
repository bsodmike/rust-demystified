use std::cmp::PartialEq;

#[derive(Debug)]
pub struct DiskSpace;
pub struct FreeMem;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MonitorableComponent {
    DiskSpace,
    FreeMem,
}

pub struct Monitorable<T> {
    context: T,
}

pub trait CanMonitor<T> {
    fn get_context(&self) -> T
    where
        T: Copy;
}

impl Monitorable<MonitorableComponent> {
    pub fn new(item: MonitorableComponent) -> Self {
        Self { context: item }
    }
}

impl<T> CanMonitor<T> for Monitorable<T> {
    fn get_context(&self) -> T
    where
        T: Copy,
    {
        return self.context;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_is_composable() {
        let disk_space = Monitorable::<MonitorableComponent>::new(MonitorableComponent::DiskSpace);
        assert_eq!(disk_space.context, MonitorableComponent::DiskSpace);
        assert_eq!(disk_space.get_context(), MonitorableComponent::DiskSpace);

        let free_mem = Monitorable::<MonitorableComponent>::new(MonitorableComponent::FreeMem);
        assert_eq!(free_mem.context, MonitorableComponent::FreeMem);
        assert_eq!(free_mem.get_context(), MonitorableComponent::FreeMem);
    }
}
