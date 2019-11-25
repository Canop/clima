use crossterm::{
    cursor,
    input::{self, InputEvent, KeyEvent},
    queue,
    screen::{EnterAlternateScreen, LeaveAlternateScreen, RawScreen},
    style::Color::*,
};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};
use termimad::{Alignment, Area, MadSkin, MadView, Result};

fn run_scrollable(w: &mut io::Stderr, area: Area, skin: MadSkin, markdown: &str) -> Result<()> {
    let mut view = MadView::from(markdown.to_owned(), area, skin);
    let mut events = input::input().read_sync();
    loop {
        view.write_on(w)?;
        w.flush()?;
        if let Some(InputEvent::Keyboard(key)) = events.next() {
            match key {
                KeyEvent::Up => view.try_scroll_lines(-1),
                KeyEvent::Down => view.try_scroll_lines(1),
                KeyEvent::PageUp => view.try_scroll_pages(-1),
                KeyEvent::PageDown => view.try_scroll_pages(1),
                _ => break,
            }
        }
    }
    Ok(())
}

fn show_path(w: &mut io::Stderr, y: u16, skin: &MadSkin, target: &Path) -> Result<()> {
    queue!(w, cursor::MoveTo(0, y))?;
    let path = target.to_string_lossy();
    mad_write_inline!(w, skin, "**Clima >** *$0*", &path)
}

fn show_help(w: &mut io::Stderr, y: u16, skin: &MadSkin) -> Result<()> {
    queue!(w, cursor::MoveTo(0, y))?;
    mad_write_inline!(
        w,
        skin,
        "Use the **🡑** and **🡓** arrow keys to scroll, any other key to quit"
    )
}

fn make_skin() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.table.align = Alignment::Center;
    skin.set_headers_fg(Yellow);
    skin.bold.set_fg(Yellow);
    skin.italic.set_fg(White);
    skin.scrollbar.thumb.set_fg(AnsiValue(178));
    skin
}

pub fn run(target: &Path) -> Result<()> {
    let markdown = fs::read_to_string(target)?;
    let skin = make_skin();

    let mut w = std::io::stderr();
    queue!(w, EnterAlternateScreen)?;
    queue!(w, cursor::Hide)?; // hiding the cursor
    let _raw_screen = RawScreen::into_raw_mode()?;

    let mut main_area = Area::full_screen();
    main_area.pad(0, 1);
    show_path(&mut w, 0, &skin, target)?;
    show_help(&mut w, main_area.top + main_area.height + 1, &skin)?;
    run_scrollable(&mut w, main_area, skin, &markdown)?;

    queue!(w, cursor::Show)?;
    queue!(w, LeaveAlternateScreen)?;
    w.flush()?;
    Ok(())
}
