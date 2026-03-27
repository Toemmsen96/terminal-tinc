use std::io;
use tui::{
    backend::CrosstermBackend,
    style::Color,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders,
    },
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run app
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let size = f.size();
            
            let canvas = Canvas::default()
                .block(Block::default().title("Catan Board (Hexagons)").borders(Borders::ALL))
                .x_bounds([-60.0, 60.0])
                .y_bounds([-60.0, 60.0])
                .paint(|ctx| {
                    let r = 10.0;
                    let h = r * 3.0f64.sqrt();
                    let w = 2.0 * r;

                    // Pointy topped hexagons
                    let angles = [30.0, 90.0, 150.0, 210.0, 270.0, 330.0, 390.0];
                    let rad = |deg: f64| deg * std::f64::consts::PI / 180.0;
                    let aspect_ratio = 0.5; // Adjusted width multiplier (was 2.0)
                    
                    let mut draw_hex = |cx: f64, cy: f64| {
                        let mut pts = vec![];
                        for &a in &angles {
                            pts.push((cx + r * rad(a).cos() * aspect_ratio, cy + r * rad(a).sin()));
                        }
                        for i in 0..6 {
                            ctx.draw(&Line {
                                x1: pts[i].0,
                                y1: pts[i].1,
                                x2: pts[i+1].0,
                                y2: pts[i+1].1,
                                color: Color::Yellow,
                            });
                        }
                    };

                    // Draw a small 1-circle grid of hexagons (center + 6 neighbors)
                    draw_hex(0.0, 0.0);
                    
                    // Neighbors
                    let offset_x = w * 0.75 * aspect_ratio;
                    let offset_y = h / 2.0;
                    
                    // Right and Left
                    draw_hex(offset_x, offset_y);
                    draw_hex(offset_x, -offset_y);
                    draw_hex(-offset_x, offset_y);
                    draw_hex(-offset_x, -offset_y);
                    draw_hex(0.0, h);
                    draw_hex(0.0, -h);
                });
                
            f.render_widget(canvas, size);
        })?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}