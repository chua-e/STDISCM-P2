use std::sync::{Arc, Mutex};
use tokio::sync::{Semaphore};
use tokio::time::{sleep, Duration};

struct Party {
    id: u32,
    tanks: u32,
    healers: u32,
    dps: u32
}

struct Instance {
    status: bool,
    parties_served: u32,
    total_time: u32
}

async fn assign(n: u32, t: u32, h: u32, d: u32) {
    let mut tank_count = t;
    let mut healer_count = h;
    let mut dps_count = d;

    let permits = Arc::new(Semaphore::new(n.try_into().unwrap()));
    let semaphore = permits.clone();
    let _permit = semaphore.acquire_owned().await.unwrap();
}

fn matchmaker (t: u32, h: u32, d: u32) -> (u32, u32, u32, u32) {
    let mut total = t + h + d;
    let mut remaining_tanks = t;
    let mut remaining_healers = h;
    let mut remaining_dps = d;

    if total >= 5 {
        if remaining_dps >= 3 {
            remaining_dps = remaining_dps - 3;
            total = total - 3;
        }

        if remaining_healers >= 1 {
            remaining_healers = remaining_healers - 1;
            total = total - 1;
        }

        if remaining_tanks >= 1 {
            remaining_tanks = remaining_tanks - 1;
            total = total - 1;
        }
    }

    return (total, remaining_tanks, remaining_healers, remaining_dps);
}

// fn print_reqs () {

// }