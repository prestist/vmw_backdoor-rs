use vmw_backdoor as vmw;

fn main() {
    let is_vmw = vmw::is_vmware_cpu();
    println!("VMware CPU detected: {}.", is_vmw);

    let mut backdoor = vmw::access_backdoor().unwrap();
    println!("Raised I/O access to reach backdoor port.");

    let found = match backdoor.probe_vmware_backdoor() {
        Ok(()) => true,
        Err(_) => false,
    };
    println!("VMware backdoor detected: {}.", found);
}