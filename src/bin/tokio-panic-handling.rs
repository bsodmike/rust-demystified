//! Correctly handle panic failures in threads spawned with Tokio.

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let control_path_1 = tokio::spawn(async {
        println!("control path (path_1)");
        for i in (0..5).rev() {
            println!("(path_1) ...{i}");
            sleep(Duration::from_millis(1000)).await;
        }
        panic!("(path_1) BOOOOOOOOOOOOOM!!!!");
    });

    let control_path_2 = tokio::spawn(async {
        println!("control path (path_2)");
        loop {
            sleep(Duration::from_millis(222)).await;
            println!("(path_2)");
        }
    });

    tokio::select! { // as a bonus you get error handling with _?_
        v = control_path_1 => v?,
        v = control_path_2 => v?,
    }
}
