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
    "self-loving",
    "hard-working",
    "well-rested",
    "well-loved",
    "loving",
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

fn text_bar_u8_8wide(value: u8, max: u8) -> String {
    let bar_width: u8 = 9;
    let filled = ((value * bar_width) / max + 1).min(bar_width);
    let partial = {
        match (value % (max / 8)) / 4 {
            0 => ' ',
            1 => '▏',
            2 => '▎',
            3 => '▍',
            4 => '▌',
            5 => '▋',
            6 => '▊',
            7 => '▉',
            8 => '█',
            _ => '?',
        }
    };
    let final_filled = {
        let mut res = String::new();
        for i in 0..filled {
            if i < filled - 1 {
                res.push('█');
            } else {
                res.push(partial);
            }
        }
        res
    };
    final_filled + &" ".repeat((bar_width - filled) as usize) + format!("{:02}", value).as_str()
}

#[derive(State)]
struct UIMainState {
    title_flavor: Value<String>,
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
struct PlayerStatsState {
    user_color: Value<Hex>,
    sleep: Value<u8>,
    eat: Value<u8>,
    bar_sleep: Value<String>,
    bar_eat: Value<String>,
}

struct PlayerStats {}

impl Component for PlayerStats {
    type Message = ();
    type State = PlayerStatsState;

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
        "player_stats",
        "src/player_stats.aml",
        || PlayerStats {},
        || PlayerStatsState {
            user_color: Hex::from((107, 107, 255)).into(),
            sleep: SLEEP_MAX.into(),
            eat: EAT_MAX.into(),
            bar_sleep: text_bar_u8_8wide(SLEEP_MAX, SLEEP_MAX).into(),
            bar_eat: text_bar_u8_8wide(EAT_MAX, EAT_MAX).into(),
        },
    );

    let flavor_index = thread_rng().gen_range(0..TITLE_FLAVORS.len());
    runtime
        .register_component(
            "main",
            "src/ui.aml",
            UIMain {},
            UIMainState {
                title_flavor: format!("{} ", TITLE_FLAVORS[flavor_index])
                    .to_string()
                    .into(),
            },
        )
        .unwrap();

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}
