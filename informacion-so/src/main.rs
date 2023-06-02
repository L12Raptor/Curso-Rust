fn main() {
    let osinfo = systeminfo::from_system_os();

    println!("OS type: {}", osinfo.os);

    println!("OS version: {}", osinfo.version);

    println!("OS edition: {}", osinfo.edition);

    println!("OS architecture: {}", osinfo.architecture);

    println!();

    let sysinfo = systeminfo::from_system_hardware();

    let sys_manufacturer = sysinfo.system_manufacturer;

    if !sys_manufacturer.is_empty() {
        println!("System manufacturer: {}", sys_manufacturer);

        let mut is_vm = false;

        let vm_manufacturers = vec!["QEMU", "innotek", "VMware"];

        for manufacturer in vm_manufacturers.iter() {
            if sys_manufacturer.contains(manufacturer) {
                is_vm = true;

                break;
            }
        }

        println!("Is Virtual Machine: {}", is_vm);
    } else {
        println!("Error getting system information, try running as administrator.");
    }
}
