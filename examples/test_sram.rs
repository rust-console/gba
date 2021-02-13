#![no_std]
#![feature(start)]
#![forbid(unsafe_code)]

use gba::{fatal, info, time_this01};
use gba::sram::*;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  fatal!("{}", info);
  loop {}
}

#[derive(Default)]
struct Lcg(u32);
impl Lcg {
    fn reset(&mut self) {
        *self = Default::default();
    }

    fn next(&mut self) -> u8 {
        self.0 = self.0 * 2891336453 + 100001;
        (self.0 >> 22) as u8 ^ self.0 as u8
    }
}

const GBA_CLOCKRATE: u32 = 16780000;

fn check_status<T>(r: Result<T, Error>) -> T {
    match r {
        Ok(v) => v,
        Err(e) => panic!("Error encountered: {:?}", e),
    }
}

fn do_test() {
    let access = get_accessor();
    if access.len() == 0 {
        panic!("No SRAM accessor is installed.")
    }
    let block_ct = access.len() / 512;

    info!("Writing SRAM...");
    let mut lcg = Lcg::default();
    for i in 0..block_ct {
        let mut buffer = [0; 512];
        for j in 0..512 {
            buffer[j] = lcg.next();
        }
        check_status(access.write(i * 512, &buffer, true));
    }

    info!("Validating SRAM...");
    lcg.reset();
    for i in 0..block_ct {
        let mut buffer = [0; 512];
        check_status(access.read(i * 512, &mut buffer));
        for j in 0..512 {
            let cur = lcg.next();
            if buffer[j] != cur {
                panic!("SRAM read does not match SRAM write: {} != {} @ 0x{:x}",
                       buffer[j], cur, i * 512 + j);
            }
        }
    }
}

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    use_battery_backed_sram();

    let time = time_this01!(do_test());
    let seconds = time / GBA_CLOCKRATE;
    let fractional = (time / (GBA_CLOCKRATE / 100)) % 100;
    info!("Finished in {}.{:02} seconds.", seconds, fractional);

    0
}