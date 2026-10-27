use std::sync::{Arc, Mutex};
use tokio::sync::{Semaphore};
use tokio::time::{sleep, Duration};
use rand::Rng;

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

pub async fn assign(n: u32, t: u32, h: u32, d: u32, t1: u32, t2: u32) {
    let parties = t.min(h).min(d / 3);

    let semaphore = Arc::new(Semaphore::new(n as usize));
    let instances = Arc::new(Mutex::new(
        (0..n).map(|_| Instance { status: false, parties_served: 0, total_time: 0 })
            .collect::<Vec<_>>(),
    ));

    let mut handles = Vec::new();

    for party in 1..=parties {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let instances = Arc::clone(&instances);

        handles.push(tokio::spawn(async move {
            let _permit = permit;
        }));

        let slots = {
            let mut inst = instances.lock().unwrap();
            let i = inst.iter().position(|x| !x.status).unwrap();
            inst[i].status = true;

            i
        };

        let time = rand::random_range(t1..t2);
        sleep(Duration::from_secs(time)).await;

        {
            let mut inst = instances.lock().unwrap();
            inst[slot].status = false;
            inst[slot].parties_serves += 1;
            inst[slot].total_time += time;
        }   
    }

    for handle in handles {
        handle.await.unwrap();
    }
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