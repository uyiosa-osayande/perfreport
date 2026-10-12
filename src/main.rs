// ============================================================================
//  BOILERPLATE : PerfReport (Rust)  |  IS2083 Lab 3
// ============================================================================
//  This boilerplate already compiles and runs. It prints the banner, computes
//  a score from stub numbers, and prints it. Your job is to replace each TODO
//  with a real reading from the sysinfo crate, following the handout, one at a
//  time. Run `cargo run` after each change to be sure it still builds.
//
//  The scoring logic and helpers are done for you. You mostly add println! and
//  set the mem_load and disk_used_pct variables from real data.
// ============================================================================

use sysinfo::{Disks, System};

const GB: f64 = 1024.0 * 1024.0 * 1024.0;

// --- Given: findings + scoring (do not change) ------------------------------
struct Findings {
    mem_load: u64,
    disk_used_pct: f64,
}

fn compute_score(f: &Findings) -> i32 {
    let mut score: i32 = 100;
    if f.mem_load > 85 {
        score -= 25;
    } else if f.mem_load > 70 {
        score -= 10;
    }
    if f.disk_used_pct > 90.0 {
        score -= 25;
    } else if f.disk_used_pct > 75.0 {
        score -= 10;
    }
    if score < 0 {
        score = 0;
    }
    score
}

fn rating(score: i32) -> &'static str {
    if score >= 85 {
        "GOOD"
    } else if score >= 60 {
        "FAIR"
    } else {
        "POOR"
    }
}

fn main() {
    println!("==== PerfReport (Rust) ====");

    // This line makes the sysinfo tools available. You will read from `sys`.
    let mut sys = System::new_all();
    sys.refresh_all();

    // Section 1 TODO: processors.
    let cpus = sys.cpus().len();
    println!("Logical processors : {}", cpus);

    // Section 2 TODO: memory.
    let total = sys.total_memory();
    let used = sys.used_memory();
    let mem_load = if total > 0 {
        (used as f64 / total as f64 * 100.0) as u64
    } else {
        0
    };
    println!("Memory total (GB)  : {:.2}", total as f64 / GB);
    println!("Memory free  (GB)  : {:.2}", (total - used) as f64 / GB);
    println!("Memory in use (%)  : {}", mem_load);

    // Section 3 TODO: uptime.
    println!("Uptime (minutes)   : {}", System::uptime() / 60);

    // Section 4 TODO: disk.
    let mut disk_used_pct: f64 = 0.0;
    let disks = Disks::new_with_refreshed_list();
    if let Some(disk) = disks.list().first() {
        let dtotal = disk.total_space();
        let dfree = disk.available_space();
        let mut used_pct = 0.0;
        if dtotal > 0 {
            used_pct = 100.0 * (dtotal - dfree) as f64 / dtotal as f64;
        }
        println!("Disk total (GB)    : {:.2}", dtotal as f64 / GB);
        println!("Disk free  (GB)    : {:.2}", dfree as f64 / GB);
        println!("Disk used  (%)     : {:.2}", used_pct);
        disk_used_pct = used_pct;
    }

    // Section 5: score (given, once the two variables above are real)
    let findings = Findings {
        mem_load,
        disk_used_pct,
    };
    let score = compute_score(&findings);
    println!("Efficiency score   : {} / 100 ({})", score, rating(score));
    println!("Done.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_score() {
        let findings = Findings {
            mem_load: 50,
            disk_used_pct: 40.0,
        };

        assert_eq!(compute_score(&findings), 100);
    }

    #[test]
    fn test_high_memory() {
        let findings = Findings {
            mem_load: 92,
            disk_used_pct: 40.0,
        };

        assert_eq!(compute_score(&findings), 75);
    }

    #[test]
    fn test_high_disk_usage() {
        let findings = Findings {
            mem_load: 50,
            disk_used_pct: 95.0,
        };

        assert_eq!(compute_score(&findings), 75);
    }
}
