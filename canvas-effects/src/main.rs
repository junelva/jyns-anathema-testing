use anathema::backend::tui::Style;
use anathema::component::*;
use anathema::default_widgets::Canvas;
use anathema::prelude::*;
use rand::prelude::*;
use std::cmp::max;
use std::time::Duration;
use std::time::Instant;

const BUBBLE: &str = "·⋅◌⊙⊚⦾⁜";

#[derive(State)]
struct UIMainState {
    fps: Value<usize>,
}

impl UIMainState {
    fn new() -> Self {
        Self { fps: 24.into() }
    }
}

// example main UI for any "app" that might use CanvasFX.
struct UIMain {}

impl UIMain {
    fn new() -> Self {
        Self {}
    }
}

impl Component for UIMain {
    type Message = ();
    type State = UIMainState;

    fn tick(
        &mut self,
        state: &mut Self::State,
        mut elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
        _dt: Duration,
    ) {
        // note: finding by custom "canvasfx" *TAG*
        //  didn't work, so I'm stashing the editable value
        //  as a custom attribute "fps" on a sub-element of
        //  my widget which can be found by custom "id" attribute.
        // additionally, setting a custom "id" attr on CanvasFX
        //  in template did not result in any query matches either.

        // nonetheless, this is how I wound up transferring values
        // to all child canvasfx components.
        elements.by_attribute("id", "canvasfx").each(|_e, a| {
            a.set("fps", *state.fps.to_mut());

            // from another attempt
            // let e = e.to::<CanvasFX>();
            // e.set_fps(*state.fps.to_mut());
        });
    }

    fn on_key(
        &mut self,
        key: KeyEvent,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_, Self::State>,
    ) {
        match key.code {
            KeyCode::Char('j') => {
                let current = *state.fps.to_mut();
                if current > 1 {
                    *state.fps.to_mut() -= 1;
                }
            }
            KeyCode::Char('k') => {
                let current = *state.fps.to_mut();
                if current < 240 {
                    *state.fps.to_mut() += 1;
                }
            }
            _ => {}
        }
    }
}

#[derive(State)]
struct CanvasFXState {}

impl CanvasFXState {
    fn new() -> Self {
        Self {}
    }
}

struct CanvasFX {
    app_start: Instant,
    time_secs: f64,     // this value is only updated on an animation tick
    anim_tick: usize,   // this value goes up every n seconds
    anim_tick_per: f64, // seconds per animation tick
}

impl CanvasFX {
    fn new() -> Self {
        Self {
            app_start: Instant::now(),
            time_secs: 0.0f64,
            anim_tick: 0usize,
            anim_tick_per: 1.0 / 24.0,
        }
    }

    // unused, did not function as expected
    // fn set_fps(&mut self, fps: usize) {
    //     self.anim_tick_per = 1.0 / (max(fps, 1) as f64);
    // }
}

impl Component for CanvasFX {
    type Message = ();
    type State = CanvasFXState;

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

        elements.by_attribute("id", "canvasfx").first(|e, a| {
            // here we retrieve any configured animation fps value.
            let fps = a.get("fps").unwrap();
            self.anim_tick_per = 1.0 / (max(fps, 1) as f64);

            let sz = e.size();
            let canvas = e.to::<Canvas>();
            let style = Style::new();

            // just some random bubbles/fizz animation
            for y in 0..sz.height {
                for x in 0..sz.width {
                    let mut coord = (x as u16, y as u16);
                    let at_coord = canvas.get(coord);
                    if (at_coord.is_none() || (at_coord.is_some() && at_coord.unwrap().0 == ' '))
                        && random::<f32>() < 0.001
                    {
                        canvas.put(BUBBLE.chars().nth(0).unwrap(), style, coord);
                    } else if at_coord.is_some() {
                        let c = at_coord.unwrap();
                        for (idx, anim_frame) in BUBBLE.chars().enumerate() {
                            if c.0 == anim_frame {
                                let next = idx + 1;
                                let next_char = BUBBLE.chars().nth(next).unwrap_or(' ');
                                canvas.put(' ', style, coord);
                                if coord.1 > 0 {
                                    coord.1 -= 1;
                                }
                                canvas.put(next_char, style, coord);
                            }
                        }
                    }
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
        // unused for now
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

    runtime
        .register_prototype(
            "canvasfx",
            "src/canvasFX.aml",
            CanvasFX::new,
            CanvasFXState::new,
        )
        .unwrap();

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}
