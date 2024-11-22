#![no_std]

#[macro_use]
extern crate axlog;

use allocator::{BaseAllocator, ByteAllocator, PageAllocator};

/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    pagesize: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        // info!("EarlyAllocator::new()");
        Self {
            start: 0,
            end: 0,
            pagesize: PAGE_SIZE,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        debug!("EarlyAllocator::init({:#x}, {:#x})", start, size);
        self.start = start;
        self.end = start + size;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> allocator::AllocResult {
        info!("EarlyAllocator::add_memory({:#x}, {:#x})", start, size);
        allocator::AllocResult::Ok(())
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(
        &mut self,
        layout: core::alloc::Layout,
    ) -> allocator::AllocResult<core::ptr::NonNull<u8>> {
        info!("EarlyAllocator::alloc({:#x})", layout.size());
        allocator::AllocResult::Ok(core::ptr::NonNull::dangling())
    }

    fn dealloc(&mut self, ptr: core::ptr::NonNull<u8>, layout: core::alloc::Layout) {
        info!(
            "EarlyAllocator::dealloc({:#x}), ptr is{:#x} ",
            layout.size(),
            ptr.as_ptr() as usize
        );
    }

    fn total_bytes(&self) -> usize {
        info!("EarlyAllocator::total_bytes()");
        0
    }

    fn used_bytes(&self) -> usize {
        info!("EarlyAllocator::used_bytes()");
        0
    }

    fn available_bytes(&self) -> usize {
        info!("EarlyAllocator::available_bytes()");
        0
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;
    fn alloc_pages(
        &mut self,
        num_pages: usize,
        align_pow2: usize,
    ) -> allocator::AllocResult<usize> {
        info!("EarlyAllocator::alloc_pages({}, {})", num_pages, align_pow2);
        allocator::AllocResult::Ok(0)
    }

    fn dealloc_pages(&mut self, start: usize, count: usize) {
        info!("EarlyAllocator::dealloc_pages({:#x}, {})", start, count);
    }

    fn total_pages(&self) -> usize {
        info!("EarlyAllocator::total_pages()");
        0
    }

    fn used_pages(&self) -> usize {
        info!("EarlyAllocator::used_pages()");
        0
    }

    fn available_pages(&self) -> usize {
        info!("EarlyAllocator::available_pages()");
        0
    }
}
