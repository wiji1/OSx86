use spin::Mutex;

type TimerHandler = fn();

static TIMER_HANDLER: Mutex<Option<TimerHandler>> = Mutex::new(None);
static mut TICK_COUNT: u64 = 0;

pub fn handle_tick() {
    unsafe {
        TICK_COUNT += 1;
    }

    if let Some(handler) = *TIMER_HANDLER.lock() {
        handler();
    }
}

pub fn set_timer_handler(handler: TimerHandler) {
    *TIMER_HANDLER.lock() = Some(handler);
}

pub fn clear_timer_handler() {
    *TIMER_HANDLER.lock() = None;
}

pub fn get_tick_count() -> u64 {
    unsafe { TICK_COUNT }
}

pub fn reset_tick_count() {
    unsafe { TICK_COUNT = 0; }
}
