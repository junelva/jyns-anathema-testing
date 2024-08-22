use anathema::component::*;
use anathema::prelude::*;
use anathema::state::Hex;
use rand::{thread_rng, Rng};
use std::time::Duration;

const SLEEP_MAX: u8 = 24;
const EAT_MAX: u8 = 24;

const TITLE_FLAVORS: &[&str] = &[
    "beautiful",
    "beloved",
    "lovable",
    "special",
    "worthy",
    "happy",
    "darling",
    "tenacious",
    "capable",
    "peaceful",
    "strongest",
    "kindest",
    "graceful",
    "goodly",
    "cutest",
    "friendly",
    "hard-working",
    "well-rested",
    "well-loved",
    "ever-loving",
    "best",
    "kindly",
    "trustworthy",
    "intuitive",
    "delightful",
    "pragmatic",
    "curious",
    "problem-solving",
    "creative",
    "patient",
    "sagely",
];

fn value_u8_formatted(value: u8, _max: u8) -> String {
    format!("{:02}", value).to_string()
}

#[derive(State)]
struct UIMainState {
    title_flavor: Value<String>,
    bun_ids: Value<List<String>>,
}

impl UIMainState {
    fn new() -> Self {
        Self {
            title_flavor: TITLE_FLAVORS[thread_rng().gen_range(0..TITLE_FLAVORS.len())]
                .to_string()
                .into(),
            bun_ids: List::empty(),
        }
    }
}

struct UIMain {}
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
        // hmmm
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

#[derive(State)]
struct BunStatsState {
    user_color: Value<Hex>,
    sleep: Value<u8>,
    eat: Value<u8>,
    bar_sleep: Value<String>,
    bar_eat: Value<String>,
}

struct BunStats {}

impl Component for BunStats {
    type Message = ();
    type State = BunStatsState;

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
        "bunstats",
        "src/ui_bunstats.aml",
        || BunStats {},
        || BunStatsState {
            user_color: Hex::from((107, 107, 255)).into(),
            sleep: SLEEP_MAX.into(),
            eat: EAT_MAX.into(),
            bar_sleep: value_u8_formatted(SLEEP_MAX, SLEEP_MAX).into(),
            bar_eat: value_u8_formatted(EAT_MAX, EAT_MAX).into(),
        },
    );

    let mut main_state = UIMainState::new();
    main_state.bun_ids.push_back("bun".to_string());
    runtime
        .register_component("main", "src/ui.aml", UIMain {}, main_state)
        .unwrap();

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}
