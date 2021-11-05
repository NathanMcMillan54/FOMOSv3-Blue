use novusk::libs::libwin::{graphics::graphics::BLACK, Window};

pub fn terminal_main() {
    let mut window = Window::new(Some("Terminal"), Some((300, 140)), (50, 50), BLACK);

    window.display();
}
