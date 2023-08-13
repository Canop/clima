use {
    crate::{
        errors::ProgramError,
        open::MdFile,
    },
    std::io::{self, Write},
    termimad::{
        Area,
        Error as TermimadError,
        MadSkin,
        MadView,
        mad_write_inline,
        crossterm::{
            cursor,
            event::{
                self,
                DisableMouseCapture,
                EnableMouseCapture,
                Event,
                KeyEvent,
                KeyCode::*,
            },
            terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
            QueueableCommand,
        },
    },
};

fn run_scrollable(
    w: &mut io::Stderr,
    area: Area,
    skin: MadSkin,
    markdown: &str,
) -> Result<(), TermimadError> {
    terminal::enable_raw_mode()?;
    let mut view = MadView::from(markdown.to_owned(), area, skin);
    loop {
        view.write_on(w)?;
        w.flush()?;
        let event = event::read();
        debug!("event: {:?}", event);
        if let Ok(Event::Key(KeyEvent{code, ..})) = event {
            match code {
                Up => view.try_scroll_lines(-1),
                Down => view.try_scroll_lines(1),
                PageUp => view.try_scroll_pages(-1),
                PageDown => view.try_scroll_pages(1),
                Char('k') => view.try_scroll_lines(-1),
                Char('j') => view.try_scroll_lines(1),
                _ => break,
            }
        }
    }
    terminal::disable_raw_mode()?;
    Ok(())
}

fn show_path(
    w: &mut io::Stderr,
    y: u16,
    skin: &MadSkin,
    path: &str,
) -> Result<(), TermimadError> {
    w.queue(cursor::MoveTo(0, y))?;
    mad_write_inline!(w, skin, "**Clima >** *$0*", path)
}

fn show_help(
    w: &mut io::Stderr,
    y: u16,
    skin: &MadSkin,
) -> Result<(), TermimadError> {
    w.queue(cursor::MoveTo(0, y))?;
    mad_write_inline!(
        w,
        skin,
        "Use the **↓** and **↑** arrow keys to scroll, any other key to quit"
    )
}

pub fn run(
    file: MdFile,
    skin: MadSkin,
    just_print: bool,
) -> Result<(), ProgramError> {
    if just_print {
        skin.print_text(&file.markdown);
    } else {
        let mut w = std::io::stderr();
        w.queue(EnterAlternateScreen)?;
        w.queue(cursor::Hide)?; // hiding the cursor
        let mut main_area = Area::full_screen();
        main_area.pad(0, 1);
        show_path(&mut w, 0, &skin, &file.path)?;
        show_help(&mut w, main_area.top + main_area.height + 1, &skin)?;
        w.queue(EnableMouseCapture)?;
        run_scrollable(&mut w, main_area, skin, &file.markdown)?;
        w.queue(DisableMouseCapture)?;
        w.queue(cursor::Show)?;
        w.queue(LeaveAlternateScreen)?;
        w.flush()?;
    }
    Ok(())
}
