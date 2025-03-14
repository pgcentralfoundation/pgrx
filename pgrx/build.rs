fn main() {
    if size_of::<pgrx_pg_sys::TransactionId>() == size_of::<u64>() {
        println!("cargo:rustc-cfg=xid8");
    }
}
