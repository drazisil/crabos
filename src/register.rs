pub fn eflags () -> u32 {
    let eflags: u32;
    unsafe {
        asm! (
            "pushfq",
            "pop {0}",
            out(reg) eflags,
        );
    }
    return eflags;
}



