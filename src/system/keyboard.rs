use pc_keyboard::DecodedKey;
use spin::Mutex;

pub type KeyHandler = fn(DecodedKey);

static KEY_HANDLER: Mutex<Option<KeyHandler>> = Mutex::new(None);

pub fn set_key_handler(handler: KeyHandler) {
    *KEY_HANDLER.lock() = Some(handler);
}

pub fn clear_key_handler() {
    *KEY_HANDLER.lock() = None;
}

pub fn handle_key(key: DecodedKey) {
    if let Some(handler) = *KEY_HANDLER.lock() {
        handler(key);
    }
}
