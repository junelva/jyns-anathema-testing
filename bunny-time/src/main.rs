use anathema::component::*;
use anathema::prelude::*;
use anathema::state::Hex;
use rand::{thread_rng, Rng};
use std::time::Duration;
use std::time::Instant;

const SLEEP_MAX: u8 = 24;
const EAT_MAX: u8 = 24;

const FRAME_COUNT: isize = 4;
const WEATHER_CHANGE_PER_TICKS: usize = 10;
const WEATHER_PATTERNS: &[&str] = &[
    "   *", "  * ", " *  ", "*   ", "   ~", "  ~ ", " ~  ", "~   ", " oOo", "oOo ", "Oo o", "o oO",
    "/// ", "// //", "/ ///", " ///", "* * ", " * *", "* * ", " * *", "~~~ ", "~~ ~", "~ ~~",
    " ~~~", ". . ", " . .", ". . ", " . .",
];

#[derive(Clone, Copy)]
enum WeatherPattern {
    Star = 0,
    Wind = FRAME_COUNT,
    Cloud = FRAME_COUNT * 2,
    Rain = FRAME_COUNT * 3,
    Snow = FRAME_COUNT * 4,
    Wimdy = FRAME_COUNT * 5,
    Peaceful = FRAME_COUNT * 6,
}

const WEATHER_FORECAST: &[&str] = &[
    "stars shine",
    "wind passes",
    "clouds go by",
    "rain may come",
    "snow may fall",
    "sometimes it's wimdy",
    "stillness, silence",
];

fn weather_forecast(state: WeatherPattern) -> &'static str {
    match state {
        WeatherPattern::Star => WEATHER_FORECAST[0],
        WeatherPattern::Wind => WEATHER_FORECAST[1],
        WeatherPattern::Cloud => WEATHER_FORECAST[2],
        WeatherPattern::Rain => WEATHER_FORECAST[3],
        WeatherPattern::Snow => WEATHER_FORECAST[4],
        WeatherPattern::Wimdy => WEATHER_FORECAST[5],
        WeatherPattern::Peaceful => WEATHER_FORECAST[6],
    }
}

fn weather_scrolled_fill(scroll: usize, fill_type: WeatherPattern) -> &'static str {
    let scroll = scroll % FRAME_COUNT as usize;
    WEATHER_PATTERNS[fill_type as usize + scroll]
}

fn cycle_weather(states: &[WeatherPattern; 3]) -> [WeatherPattern; 3] {
    let mut rng = rand::thread_rng();
    [
        states[1],
        states[2],
        match rng.gen_range(0..7) {
            0 => WeatherPattern::Star,
            1 => WeatherPattern::Wind,
            2 => WeatherPattern::Cloud,
            3 => WeatherPattern::Rain,
            4 => WeatherPattern::Snow,
            5 => WeatherPattern::Wimdy,
            6 => WeatherPattern::Peaceful,
            _ => WeatherPattern::Peaceful,
        },
    ]
}

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
    "calming",
];

const FUTURE_FLAVORS: &[&str] = &[
    "you are safe",
    "it's ok",
    "take some time",
    "you're all right",
    "this is fine",
    "we'll be ok",
    "it takes time",
    "you'll get through",
    "it'll be ok",
    "actually ok",
];

fn value_u8_formatted(value: u8, _max: u8) -> String {
    format!("{:02}", value).to_string()
}

#[derive(State)]
struct UIMainState {
    title_flavor: Value<String>,
    bun_ids: Value<List<String>>,
    string_lvl: Value<String>,
    string_exp: Value<String>,
    string_date: Value<String>,
    string_wind: Value<String>,
    string_future: Value<String>,
    string_forecast: Value<String>,
}

impl UIMainState {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            title_flavor: TITLE_FLAVORS[thread_rng().gen_range(0..TITLE_FLAVORS.len())]
                .to_string()
                .into(),
            bun_ids: List::empty(),
            string_lvl: "???".to_string().into(),
            string_exp: "???".to_string().into(),
            string_date: "???".to_string().into(),
            string_wind: "???".to_string().into(),
            string_future: FUTURE_FLAVORS[rng.gen_range(0..FUTURE_FLAVORS.len())]
                .to_string()
                .into(),
            string_forecast: weather_forecast(WeatherPattern::Cloud).to_string().into(),
        }
    }
}

struct UIMain {
    app_start: Instant,
    time_secs: f64,     // this value is only updated on an animation tick
    anim_tick: usize,   // this value goes up every n seconds
    anim_tick_per: f64, // seconds per animation tick
    weather_states: [WeatherPattern; 3],
    lvl: usize,
    exp: usize,
    wind: usize,
}

impl UIMain {
    fn new() -> Self {
        Self {
            app_start: Instant::now(),
            time_secs: 0.0f64,
            anim_tick: 0usize,
            anim_tick_per: 1.0,
            weather_states: [
                WeatherPattern::Star,
                WeatherPattern::Wind,
                WeatherPattern::Cloud,
            ],
            lvl: 0,
            exp: 0,
            wind: 0,
        }
    }
}

impl Component for UIMain {
    type Message = ();
    type State = UIMainState;

    fn tick(
        &mut self,
        state: &mut Self::State,
        mut elements: Elements<'_, '_>,
        _context: Context<'_>,
        _dt: Duration,
    ) {
        *state.string_lvl.to_mut() = format!("{:2}", self.lvl).to_string();
        *state.string_exp.to_mut() = format!("{:2}", self.exp).to_string();
        *state.string_wind.to_mut() = format!("{:3}", self.wind).to_string();
        *state.string_date.to_mut() =
            format!("{}", chrono::offset::Local::now().format("%H:%M %a"))
                .to_string()
                .to_uppercase();

        let time_now_secs = self.app_start.elapsed().as_secs_f64();
        if self.time_secs + self.anim_tick_per < time_now_secs {
            self.time_secs = time_now_secs;
            self.anim_tick += 1;

            self.exp += 1;
            if self.exp >= 60 {
                self.exp = 0;
                self.lvl += 1;
                if self.lvl > 60 {
                    self.lvl = 60;
                }
            }

            if self.anim_tick % WEATHER_CHANGE_PER_TICKS == 0 {
                self.weather_states = cycle_weather(&self.weather_states);
                *state.string_forecast.to_mut() =
                    weather_forecast(self.weather_states[2]).to_string();
                let mut rng = rand::thread_rng();
                *state.string_future.to_mut() =
                    FUTURE_FLAVORS[rng.gen_range(0..FUTURE_FLAVORS.len())].to_string()
            }

            elements
                .query()
                .by_attribute("id", "sky-left")
                .first(|_e, a| {
                    a.set(
                        "fill",
                        weather_scrolled_fill(self.anim_tick, self.weather_states[0]),
                    );
                });

            elements
                .query()
                .by_attribute("id", "sky-middle")
                .first(|_e, a| {
                    a.set(
                        "fill",
                        weather_scrolled_fill(self.anim_tick, self.weather_states[1]),
                    );
                });

            elements
                .query()
                .by_attribute("id", "sky-right")
                .first(|_e, a| {
                    a.set(
                        "fill",
                        weather_scrolled_fill(self.anim_tick, self.weather_states[2]),
                    );
                });
        }
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
    main_state.bun_ids.push_back("bun".to_string());
    main_state.bun_ids.push_back("bun".to_string());
    runtime
        .register_component("main", "src/ui.aml", UIMain::new(), main_state)
        .unwrap();

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}
