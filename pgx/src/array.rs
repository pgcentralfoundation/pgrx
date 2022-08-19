use pgx_pg_sys::*;

extern "C" {
    pub fn pgx_ARR_NELEMS(arrayType: *mut ArrayType) -> i32;
    pub fn pgx_ARR_NULLBITMAP(arrayType: *mut ArrayType) -> *mut bits8;
}

#[inline]
pub unsafe fn get_arr_data_ptr<T>(arr: *mut ArrayType) -> *mut T {
    extern "C" {
        pub fn pgx_ARR_DATA_PTR(arrayType: *mut ArrayType) -> *mut u8;
    }
    pgx_ARR_DATA_PTR(arr) as *mut T
}

#[inline]
pub fn get_arr_nelems(arr: *mut ArrayType) -> i32 {
    unsafe { pgx_ARR_NELEMS(arr) }
}

#[inline]
pub fn get_arr_ndim(arr: *mut ArrayType) -> i32 {
    extern "C" {
        pub fn pgx_ARR_NDIM(arrayType: *mut ArrayType) -> i32;
    }
    unsafe { pgx_ARR_NDIM(arr) }
}

#[inline]
pub fn get_arr_nullbitmap<'a>(arr: *mut ArrayType) -> &'a [bits8] {
    unsafe {
        let len = (pgx_ARR_NELEMS(arr) + 7) / 8;
        std::slice::from_raw_parts(pgx_ARR_NULLBITMAP(arr), len as usize)
    }
}

#[inline]
pub fn get_arr_nullbitmap_mut<'a>(arr: *mut ArrayType) -> &'a mut [u8] {
    unsafe {
        let len = (pgx_ARR_NELEMS(arr) + 7) / 8;
        std::slice::from_raw_parts_mut(pgx_ARR_NULLBITMAP(arr), len as usize)
    }
}

#[inline]
pub fn get_arr_hasnull(arr: *mut ArrayType) -> bool {
    // copied from array.h
    unsafe { (*arr).dataoffset != 0 }
}

#[inline]
pub fn get_arr_dims<'a>(arr: *mut ArrayType) -> &'a [i32] {
    extern "C" {
        pub fn pgx_ARR_DIMS(arrayType: *mut ArrayType) -> *mut i32;
    }
    unsafe {
        let len = (*arr).ndim;
        std::slice::from_raw_parts(pgx_ARR_DIMS(arr), len as usize)
    }
}
