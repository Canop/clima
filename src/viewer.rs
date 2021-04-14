use {
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
        style::Color::*,
        QueueableCommand,
    },
    std::{
        fs,
        io::{self, Write},
        path::Path,
    },
    termimad::{
        Alignment, Area, MadSkin, MadView, Result,
    },
};

fn run_scrollable(w: &mut io::Stderr, area: Area, skin: MadSkin, markdown: &str) -> Result<()> {
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

fn show_path(w: &mut io::Stderr, y: u16, skin: &MadSkin, target: &Path) -> Result<()> {
    w.queue(cursor::MoveTo(0, y))?;
    let path = target.to_string_lossy();
    mad_write_inline!(w, skin, "**Clima >** *$0*", &path)
}

fn show_help(w: &mut io::Stderr, y: u16, skin: &MadSkin) -> Result<()> {
    w.queue(cursor::MoveTo(0, y))?;
    mad_write_inline!(
        w,
        skin,
        "Use the **↓** and **↑** arrow keys to scroll, any other key to quit"
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

pub fn run(launch_args: crate::cli::AppLaunchArgs) -> Result<()> {
    let target = launch_args.target;
    let markdown = fs::read_to_string(&target)?;
    let skin = make_skin();

    if launch_args.just_print {
        skin.print_text(&markdown);
    } else {
        let mut w = std::io::stderr();
        w.queue(EnterAlternateScreen)?;
        w.queue(cursor::Hide)?; // hiding the cursor
        let mut main_area = Area::full_screen();
        main_area.pad(0, 1);
        show_path(&mut w, 0, &skin, &target)?;
        show_help(&mut w, main_area.top + main_area.height + 1, &skin)?;
        w.queue(EnableMouseCapture)?;
        run_scrollable(&mut w, main_area, skin, &markdown)?;
        w.queue(DisableMouseCapture)?;
        w.queue(cursor::Show)?;
        w.queue(LeaveAlternateScreen)?;
        w.flush()?;
    }
    Ok(())
}
