use std::println;
use std::sync::{Arc, Mutex};
use tokio::sync::{Semaphore};
use tokio::time::{sleep, Duration};
use rand::Rng;

struct Instance {
    status: bool,
    parties_served: u32,
    total_time: u32
}

pub async fn assign(n: u32, t: u32, h: u32, d: u32, t1: u32, t2: u32) {
    let (parties, left_t, left_h, left_d) = matchmaker(t, h, d);

    let semaphore = Arc::new(Semaphore::new(n as usize));
    let instances = Arc::new(Mutex::new(
        (0..n).map(|_| Instance { status: false, parties_served: 0, total_time: 0 })
            .collect::<Vec<_>>(),
    ));

    let mut handles = Vec::new();

    for _party in 1..=parties {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        let instances = Arc::clone(&instances);

        handles.push(tokio::spawn(async move {
            let _permit = permit;

            let slot = {
                let mut inst = instances.lock().unwrap();
                let i = inst.iter().position(|x| !x.status).unwrap();
                inst[i].status = true;
                print_status(&inst);

                i
            };

            let time = rand::random_range(t1..=t2);
            sleep(Duration::from_secs(time as u64)).await;

            {
                let mut inst = instances.lock().unwrap();
                inst[slot].status = false;
                inst[slot].parties_served += 1;
                inst[slot].total_time += time;
                print_status(&inst);
            }   
        }));
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let inst = instances.lock().unwrap();
    print_summary(&inst, left_h, left_t, left_d);
}

fn matchmaker (t: u32, h: u32, d: u32) -> (u32, u32, u32, u32) {
    let parties = t.min(h).min(d / 3);
    let remaining_tanks = t - parties;
    let remaining_healers = h - parties;
    let remaining_dps = d - (parties*3);

    return (parties, remaining_tanks, remaining_healers, remaining_dps);
}

fn print_status(inst: &Vec<Instance>) {
    for (i, x) in inst.iter().enumerate() {
        println!("Instance {}: {}", i + 1, if x.status { "active" } else { "empty" });
    }
    println!();
}

fn print_summary(inst: &Vec<Instance>, left_h: u32, left_t: u32, left_d: u32) {
    let mut total_parties = 0;
    let mut total_time = 0;

    println!("Summary: ");

    for (i, x) in inst.iter().enumerate() {
        println!("Instance {}:", i + 1);
        println!("Parties served - {}", x.parties_served);
        println!("Total time - {} seconds", x.total_time);
        println!();

        total_parties += x.parties_served;
        total_time += x.total_time;
    }

    println!("Total summary:");
    println!("Total parties served - {}", total_parties);
    println!("Total time served - {} seconds", total_time);
    println!("Leftover players: {} tanks, {} healers, {} DPS", left_t, left_h, left_d);
}