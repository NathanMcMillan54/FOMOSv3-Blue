use super::desktop::DESKTOP_SIZE;

pub unsafe fn x86_64_setup() {
    DESKTOP_SIZE = (80, 25);
}
