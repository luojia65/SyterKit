use crate::{Config, DynamicInfo};
use core::ptr::addr_of;

static mut DYNAMIC_INFO: DynamicInfo = DynamicInfo::new();

/// Executes the loaded payload
pub unsafe fn run_payload_on(
    config: &Config,
    firmware_address: usize,
    image_address: usize,
    opaque_address: usize,
) -> ! {
    const HART_ID: usize = 0; // Hartid of the current core
    unsafe {
        DYNAMIC_INFO = DYNAMIC_INFO
            .with_next_stage(config.next_stage.mode, image_address)
            .with_boot_hart(HART_ID)
    };

    type KernelEntry =
        unsafe extern "C" fn(hart_id: usize, dtb_addr: usize, dynamic_info: *const DynamicInfo);
    let kernel_entry: KernelEntry = unsafe { core::mem::transmute(firmware_address) };
    let opaque_address = config.opaque.as_ref().map_or(0, |_| opaque_address);

    unsafe {
        kernel_entry(HART_ID, opaque_address, addr_of!(DYNAMIC_INFO));
    }

    loop {}
}
