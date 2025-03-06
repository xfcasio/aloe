use std::alloc::{GlobalAlloc, System, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

struct AloeAllocator(System, AtomicUsize);

#[global_allocator]
static GLOBAL: AloeAllocator = AloeAllocator(System, AtomicUsize::new(0));

use printf as c_printf;
extern "C" {
    fn printf(format: *const u8, ...) -> i32;
}

macro_rules! printf {
    ($s: expr $(, $args: expr)*) => (
        { c_printf(concat!($s, "\0").as_ptr() $(, $args)*); }
    )
}

unsafe impl GlobalAlloc for AloeAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.1.fetch_add(1, Ordering::SeqCst);

        let alloc_num = self.1.load(Ordering::Relaxed) - 1;
        (0..alloc_num).for_each(|_| printf!("  "));
        printf!(
            "<\u{001b}[33mallocation \u{001b}[32mdepth\u{001b}[37m=\u{001b}[32m%d\u{001b}[0m>\n",
            alloc_num);
        
        self.0.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let alloc_num = self.1.load(Ordering::Relaxed) - 1;
        (0..alloc_num).for_each(|_| printf!("  "));
        printf!(
            "</\u{001b}[33mallocation \u{001b}[32mdepth\u{001b}[37m=\u{001b}[32m%d\u{001b}[0m>\n",
            alloc_num);

        self.1.fetch_sub(1, Ordering::SeqCst);
        self.0.dealloc(ptr, layout)
    }
}


#[test]
fn test1() {
    let _ = vec![1, 2, 3];
}
