use std::arch::asm;
fn main() {
    let o: u64;
    unsafe {
        asm!("mov {}, 5", out(reg) o);
    }

    assert_eq!(o, 5)
}
