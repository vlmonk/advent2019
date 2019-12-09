mod vm;

use std::fs;
use vm::{CPUInfo, CPU};

fn result(tag: &str, result: i64, info: CPUInfo) {
    println!(
        "Task {} : {} (ticks: {}, max_mem: {})",
        tag, result, info.ticks, info.addr
    );
}

fn main() {
    let raw = fs::read_to_string("input.txt").expect("cant read");
    let mut cpu = CPU::new_from_str(&raw, vec![1]);
    cpu.run();
    let task_a = (cpu.output[0], cpu.info());

    let mut cpu = CPU::new_from_str(&raw, vec![2]);
    cpu.run();
    let task_b = (cpu.output[0], cpu.info());

    result("I ", task_a.0, task_a.1);
    result("II", task_b.0, task_b.1);
}
