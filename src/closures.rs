use anyhow::{Error, Result};
use log::info;

#[tokio::main]
pub async fn runner() -> Result<()> {
    lesson_1::run().await?;

    Ok(())
}

mod lesson_1 {
    use super::*;
    use std::{
        collections::BTreeMap,
        fmt::{self, Debug, Display},
        sync::Arc,
        time::Duration,
    };
    use tokio::{sync::mpsc, task::JoinHandle};

    #[derive(Debug)]
    pub enum ActorMessage {
        RunTask {
            document: String,
            rows: Vec<u32>,
            alert: Option<Arc<tokio::sync::oneshot::Sender<bool>>>,
            timestamp: String,
        },
    }

    pub struct MyActor {
        receiver: mpsc::Receiver<ActorMessage>,
        next_id: u32,
    }

    impl MyActor {
        pub fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
            MyActor {
                receiver,
                next_id: 0,
            }
        }
        pub fn spawn_tasks<F, D, R>(mut f: F, d: D, r: R) -> JoinHandle<()>
        where
            F: FnMut(D, R) -> JoinHandle<()>,
        {
            f(d, r)
        }

        async fn run_in_parallel<TaskName, ItemCollection, Item>(
            task_name: TaskName,
            items: ItemCollection,
            mut fut: &impl Fn(TaskName, ItemCollection::Item) -> JoinHandle<()>,
        ) -> Vec<()>
        where
            TaskName: Display + Clone,
            ItemCollection: IntoIterator<Item = Item>,
        {
            let futures: Vec<_> = items
                .into_iter()
                .map(|row| Self::spawn_tasks(&mut fut, task_name.clone(), row))
                .collect();

            // do these futures in parallel and return them
            let mut res = Vec::with_capacity(futures.len());
            for f in futures.into_iter() {
                log::info!("run_in_parallel(): {:#?}", &f);
                f.await.expect("Run `do_task` as a parallel task");
                res.push(());
            }

            res
        }

        async fn do_task(task_name: String, mut el: u32) {
            log::info!("do_task(): {} / {}", task_name, el);
        }
    }

    pub async fn run() -> Result<()> {
        let task = "test".to_string();
        let items: Vec<u32> = vec![0, 1, 2];

        let c = |t, elem| tokio::spawn(MyActor::do_task(t, elem));

        let results = MyActor::run_in_parallel::<String, Vec<u32>, u32>(task, items, &c).await;

        log::info!("run(): done.");

        Ok(())
    }
}
