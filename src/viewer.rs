use crossterm::{
    AlternateScreen, Color::*, InputEvent::*, KeyEvent::*, TerminalCursor, TerminalInput,
};
use std::fs;
use std::io;
use std::path::Path;
use termimad::*;

fn run_scrollable(area: Area, skin: MadSkin, markdown: &str) -> io::Result<()> {
    let cursor = TerminalCursor::new();
    cursor.hide()?;
    let mut view = MadView::from(markdown.to_owned(), area, skin);
    let mut events = TerminalInput::new().read_sync();
    loop {
        view.write()?;
        if let Some(Keyboard(key)) = events.next() {
            match key {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                _ => break,
            }
        }
    }
    cursor.show()?;
    Ok(())
}

fn show_path(y: u16, skin: &MadSkin, target: &Path) {
    let cursor = TerminalCursor::new();
    cursor.goto(0, y).unwrap();
    skin.print_inline(&format!("**Clima >** *{}*", target.to_string_lossy()));
}

fn show_help(y: u16, skin: &MadSkin) {
    let cursor = TerminalCursor::new();
    cursor.goto(0, y).unwrap();
    skin.print_inline("Use the **🡑** and **🡓** arrow keys to scroll, any other key to quit");
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(Yellow);
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(White);
    skin.scrollbar.set_thumb_fg(AnsiValue(178));
    skin
}

pub fn run(target: &Path) -> io::Result<()> {
    let markdown = fs::read_to_string(target)?;
    let _alt_screen = AlternateScreen::to_alternate(true);
    let skin = make_skin();
    let mut main_area = Area::full_screen();
    main_area.pad(0, 1);
    show_path(0, &skin, target);
    show_help(main_area.top + main_area.height + 1, &skin);
    run_scrollable(main_area, skin, &markdown)
}
