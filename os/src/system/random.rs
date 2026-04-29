use spin::Mutex;

static RNG_STATE: Mutex<u64> = Mutex::new(0x853c49e6748fea9b);

pub fn seed(seed: u64) {
    *RNG_STATE.lock() = seed;
}

pub fn seed_from_timer() {
    let ticks = crate::system::timer::get_tick_count();
    seed(ticks);
}

pub fn next() -> u64 {
    let mut state = RNG_STATE.lock();

    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;

    *state = x;
    x
}

pub fn next_range(min: u64, max: u64) -> u64 {
    if min >= max {
        return min;
    }

    let range = max - min;
    min + (next() % range)
}

pub fn next_u32() -> u32 {
    next() as u32
}

pub fn next_u32_range(min: u32, max: u32) -> u32 {
    if min >= max {
        return min;
    }

    let range = max - min;
    min + (next_u32() % range)
}
