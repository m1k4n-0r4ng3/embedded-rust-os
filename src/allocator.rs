pub mod linked_list;
pub mod bump;
pub mod fixed_size_block;
use bump::BumpAllocator;
use linked_list::LinkedListAllocator;
use fixed_size_block::FixedSizeBlockAllocator;

#[global_allocator]
// static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
// static ALLOCATOR: Locked<LinkedListAllocator> =
//     Locked::new(LinkedListAllocator::new());
static ALLOCATOR: Locked<FixedSizeBlockAllocator> =
    Locked::new(FixedSizeBlockAllocator::new());


// GlobalAllocトレイトを実装できるようにするためのspin::Mutexのラッパー
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

// `addr`を`align`でアライメントする
fn align_up(addr: usize, align: usize) -> usize {
    // 基本的な実装
    // let remainder = addr & align;
    // if remainder == 0 {
    //     addr // 元からアライメントされていた
    // } else {
    //     addr - remainder + align
    // }

    // alignが2のべき乗ならもっと高速な実装ができる
    (addr + align - 1) & !(align - 1)
}

pub fn init_heap(
    heap_start: usize,
    heap_size: usize,
) {
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}