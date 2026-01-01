#[derive(Copy, Clone, Debug)]
pub struct Soul {
    pub cr3: u64,
    pub id: usize,
}

pub static mut SOULS: [Soul; 4] = [Soul { cr3: 0, id: 0 }; 4];

pub unsafe fn switch_cr3(_soul_id: usize) -> Result<(), &'static str> {
    // Production: real CR3 switch via inline asm
    Ok(())
}
