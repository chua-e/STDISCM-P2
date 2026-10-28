use std::println;
use std::sync::{Arc, Mutex};
use tokio::sync::{Semaphore};
use tokio::time::{sleep, Duration};
// use rand::Rng;

/**
 * An instance struct holds information about instance specific 
 * data such as its availability status, the number of parties it
 * has served, and the total time served
 */
struct Instance {
    status: bool,
    parties_served: u32,
    total_time: u32
}

/**
 * The assign function creates a new semaphore with n amount of
 * instances. Before matching a group with an instance, the number
 * of parties and players left are calculated through the `matchmaker()`
 * function. Each instance is then given as a slot per party where it 
 * stays occupied for a certain random amount of time between t1 and t2.
 */
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

/**
 * The matchmaker function calculates the most number of possible parties
 * by getting the most minimal player class' value. It returns the 
 * number of parties, remaining tank players, remaining healer players,
 * and remaining dps players; if any.
 */
fn matchmaker (t: u32, h: u32, d: u32) -> (u32, u32, u32, u32) {
    let parties = t.min(h).min(d / 3);
    let remaining_tanks = t - parties;
    let remaining_healers = h - parties;
    let remaining_dps = d - (parties*3);

    return (parties, remaining_tanks, remaining_healers, remaining_dps);
}

/**
 * Prints a specific instance's availability status
 */
fn print_status(inst: &Vec<Instance>) {
    for (i, x) in inst.iter().enumerate() {
        println!("Instance {}: {}", i + 1, if x.status { "active" } else { "empty" });
    }
    println!();
}

/**
 * Prints the final summary of each instance's final total statistics
 */
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