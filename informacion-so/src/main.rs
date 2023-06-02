fn main() {
    let operating_system_info = os_info::get();

    println!("OS type: {}", operating_system_info.os_type());
    println!("OS version: {}", operating_system_info.version());
    println!("OS edition: {}", operating_system_info.edition().unwrap_or_default());
    println!("OS architecture: {}", operating_system_info.architecture().unwrap_or_default());
}
