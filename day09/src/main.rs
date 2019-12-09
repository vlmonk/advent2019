mod vm;

use std::fs;
use vm::CPU;

fn main() {
    let raw = fs::read_to_string("input.txt").expect("cant read");
    let mut cpu = CPU::new_from_str(&raw, vec![1]);
    cpu.run();
    let task_a = (cpu.output[0], cpu.ticks);

    let mut cpu = CPU::new_from_str(&raw, vec![2]);
    cpu.run();
    let task_b = (cpu.output[0], cpu.ticks);

    println!("Task I : {} (ticks: {})", task_a.0, task_a.1);
    println!("Task II: {} (ticks: {})", task_b.0, task_b.1);
}
