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
    println!("pending\t{}\t{}", GLOBAL_PENDING!().get(), checks());
    println!("flag\t{}\t{}", GLOBAL_FLAG!().get(), checks());
    let _ = GLOBAL_POINTER!();
    println!("pointer\t{}", checks());
    println!("busy\t{}\t{}", GLOBAL_BUSY!().get(), checks());
    println!("in range\t{}\t{}", GLOBAL_IN_RANGE!(5).get(), checks());
    // SAFETY: the fixture's pointer addresses a live int, its hook is null, and this
    // single-threaded program owns the counter.
    unsafe {
        println!("pointee\t{}", GLOBAL_POINTEE!().get());
        println!("hooked\t{}", GLOBAL_HOOKED!().get());
        GLOBAL_SET!(3);
    }
    println!("count\t{}\t{}", GLOBAL_COUNT!().get(), checks());
}
