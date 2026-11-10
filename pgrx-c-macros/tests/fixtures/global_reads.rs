//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

/// Times a generated macro checked its thread.
static CHECKS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// Count thread checks instead of comparing threads.
pub fn thread_check() {
    CHECKS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
}

/// The number of thread checks so far.
fn checks() -> usize {
    CHECKS.load(std::sync::atomic::Ordering::Relaxed)
}

/// Read each global through its macro from safe code, then write and dereference with
/// `unsafe`, printing each value and the thread checks each macro made.
fn main() {
    println!("count\t{}\t{}", GLOBAL_COUNT!().get(), checks());
    println!("pending\t{}\t{}", GLOBAL_PENDING!(), checks());
    println!("flag\t{}\t{}", GLOBAL_FLAG!(), checks());
    let _ = GLOBAL_POINTER!();
    println!("pointer\t{}", checks());
    println!("busy\t{}\t{}", GLOBAL_BUSY!(), checks());
    println!("in range\t{}\t{}", GLOBAL_IN_RANGE!(5), checks());
    // SAFETY: the fixture's pointer addresses a live int, its hook is null, and this
    // single-threaded program owns the counter.
    unsafe {
        println!("pointee\t{}", GLOBAL_POINTEE!().get());
        println!("hooked\t{}", GLOBAL_HOOKED!());
        GLOBAL_SET!(3);
    }
    println!("count\t{}\t{}", GLOBAL_COUNT!().get(), checks());
    // Three reads in one expansion check once.
    println!("total\t{}\t{}", GLOBAL_TOTAL!().get(), checks());
    // No read runs, so nothing checks; then two reads run and check once.
    println!("any\t{}\t{}", GLOBAL_ANY!(1), checks());
    println!("any\t{}\t{}", GLOBAL_ANY!(0), checks());
    let mut copy = 0_i32;
    // SAFETY: the macro writes only this local.
    unsafe {
        GLOBAL_COPY!(copy);
    }
    println!("copy\t{copy}\t{}", checks());
    // The discarded operand's three reads each check, and then the macro's own read does.
    println!("after\t{}\t{}", GLOBAL_AFTER!(GLOBAL_TOTAL!()).get(), checks());
    // `sizeof` evaluates nothing, so none of these checks, and none needs `unsafe`: not for
    // an operand that reads through a pointer, nor for a macro that does.
    println!(
        "sizes\t{}\t{}\t{}\t{}\t{}",
        GLOBAL_SIZE!().get(),
        GLOBAL_SUM_SIZE!().get(),
        GLOBAL_POINTEE_SIZE!().get(),
        GLOBAL_HOOK_SIZE!().get(),
        checks()
    );
    let narrow = 1_i16;
    let pointer = &raw const narrow;
    println!(
        "wider\t{}\t{}\t{}\t{}",
        GLOBAL_WIDER!(narrow).get(),
        GLOBAL_WIDER!(GLOBAL_POINTEE!()).get(),
        GLOBAL_POINTEE_WIDER!().get(),
        checks()
    );
    // SAFETY: the operand is not evaluated. The caller's dereference still needs `unsafe`,
    // and the macro's own block inside it is no error.
    println!("wider\t{}", unsafe { GLOBAL_WIDER!(*pointer) }.get());
}
