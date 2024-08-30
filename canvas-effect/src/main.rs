use anathema::backend::tui::{Attributes, Color, Style};
use anathema::component::*;
use anathema::default_widgets::Canvas;
use anathema::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::*;
use std::time::Duration;
use std::time::Instant;

#[derive(State)]
struct UIMainState {}

impl UIMainState {
    fn new() -> Self {
        Self {}
    }
}

struct UIMain {
    app_start: Instant,
    time_secs: f64,     // this value is only updated on an animation tick
    anim_tick: usize,   // this value goes up every n seconds
    anim_tick_per: f64, // seconds per animation tick
}

impl UIMain {
    fn new() -> Self {
        Self {
            app_start: Instant::now(),
            time_secs: 0.0f64,
            anim_tick: 0usize,
            anim_tick_per: 1.0 / 60.0,
        }
    }
}

impl Component for UIMain {
    type Message = ();
    type State = UIMainState;

    fn tick(
        &mut self,
        _state: &mut Self::State,
        mut elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
        _dt: Duration,
    ) {
        let time_now_secs = self.app_start.elapsed().as_secs_f64();

        if self.time_secs + self.anim_tick_per > time_now_secs {
            return;
        }

        self.time_secs = time_now_secs;
        self.anim_tick += 1;

        elements.by_attribute("id", "viewport").first(|e, _a| {
            let sz = e.size();
            let canvas = e.to::<Canvas>();
            let mut style = Style::new();

            for y in 0..sz.height {
                for x in 0..sz.width {
                    let s = Alphanumeric.sample_string(&mut rand::thread_rng(), 1);
                    let fg = Color::Rgb {
                        r: random(),
                        g: random(),
                        b: random(),
                    };
                    let bg = Color::Rgb {
                        r: random::<u8>() / 4,
                        g: random::<u8>() / 4,
                        b: random::<u8>() / 2,
                    };
                    style.fg = Some(fg);
                    style.bg = Some(bg);
                    let attr: u8 = random();
                    style.attributes = Attributes::from_bits_truncate(attr);
                    canvas.put(s.chars().last().unwrap(), style, (x as u16, y as u16));
                }
            }
        });
    }

    fn on_key(
        &mut self,
        _key: KeyEvent,
        _state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
    ) {
        // hmm
    }
}

fn main() {
    let doc = Document::new("@main");

    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);

    runtime
        .register_component("main", "src/ui.aml", UIMain::new(), UIMainState::new())
        .unwrap();

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}
