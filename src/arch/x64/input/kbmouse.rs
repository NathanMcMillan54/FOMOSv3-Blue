use crate::input::MOUSEDEVICE;
use novusk::drivers::input::kb_mouse::input::kb_mouse_input;

pub extern "C" fn mouse_input() -> Result<(), &'static str> {
    let (ux, uy) = kb_mouse_input();

    unsafe { MOUSEDEVICE.update(ux as usize, uy as usize); }

    return Ok(())
}
