use num_cpus;

fn main() {
    let logical_cpus = num_cpus::get();
    let physical_cpus = num_cpus::get_physical();

    println!("Logical CPUs: {}", logical_cpus);
    println!("Physical CPUs: {}", physical_cpus);
}
