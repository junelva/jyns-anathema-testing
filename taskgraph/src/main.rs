use anathema::component::*;
use anathema::prelude::*;
use std::time::Duration;
use std::time::Instant;

#[derive(State)]
struct UIMainState {}

struct UIMain {
    app_start: Instant,
    time_secs: f64,
}

impl UIMain {
    fn new() -> Self {
        Self {
            app_start: Instant::now(),
            time_secs: 0.0f64,
        }
    }
}

#[derive(State)]
struct UiPtype00State {}
struct UiPtype00 {}

#[derive(State)]
struct UiPtype01State {}
struct UiPtype01 {}

#[derive(State)]
struct UiPtype02State {}
struct UiPtype02 {}

impl Component for UiPtype00 {
    type Message = ();
    type State = UiPtype00State;

    fn tick(
        &mut self,
        _state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
        _dt: Duration,
    ) {
    }

    fn on_key(
        &mut self,
        _key: KeyEvent,
        _state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
    ) {
    }
}

impl Component for UiPtype01 {
    type Message = ();
    type State = UiPtype01State;

    fn tick(
        &mut self,
        _state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
        _dt: Duration,
    ) {
    }

    fn on_key(
        &mut self,
        _key: KeyEvent,
        _state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
    ) {
    }
}

impl Component for UiPtype02 {
    type Message = ();
    type State = UiPtype02State;

    fn tick(
        &mut self,
        _state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
        _dt: Duration,
    ) {
    }

    fn on_key(
        &mut self,
        _key: KeyEvent,
        _state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
    ) {
    }
}

impl Component for UIMain {
    type Message = ();
    type State = UIMainState;

    fn tick(
        &mut self,
        _state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
        _dt: Duration,
    ) {
        self.time_secs = self.app_start.elapsed().as_secs_f64();
    }

    fn on_key(
        &mut self,
        _key: KeyEvent,
        _state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
    ) {
        // hmmm?
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

    let _ = runtime.register_prototype(
        "ui_ptype_00",
        "src/ui_ptype_00.aml",
        || UiPtype00 {},
        || UiPtype00State {},
    );

    let _ = runtime.register_prototype(
        "ui_ptype_01",
        "src/ui_ptype_01.aml",
        || UiPtype01 {},
        || UiPtype01State {},
    );

    let _ = runtime.register_prototype(
        "ui_ptype_02",
        "src/ui_ptype_02.aml",
        || UiPtype02 {},
        || UiPtype02State {},
    );

    let main_state = UIMainState {};
    runtime
        .register_component("main", "src/ui.aml", UIMain::new(), main_state)
        .unwrap();

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}
