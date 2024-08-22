use std::time::Duration;

use anathema::component::*;
use anathema::prelude::*;

use anathema::state::Hex;

fn text_bar_u8_8wide(value: u8) -> String {
    let total = 8;
    let filled = ((value as usize * total) / 255 + 1).min(total);
    let empty = if filled > 0 { total - filled } else { 7 };
    let partial = {
        match (value % 32) / 4 {
            0 => '.',
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
    final_filled + &".".repeat(empty) + format!("{:03}", value).as_str()
}

#[derive(State)]
struct UIMainState {}
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
struct PickRgbState {
    user_color: Value<Hex>,
    ui_sel_count: Value<u8>,
    ui_sel_index: Value<u8>,
    bar_r: Value<String>,
    bar_g: Value<String>,
    bar_b: Value<String>,
}

struct PickRgb {}

impl Component for PickRgb {
    type Message = ();
    type State = PickRgbState;

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
        key: KeyEvent,
        state: &mut Self::State,
        _elements: Elements<'_, '_>,
        _context: Context<'_>,
    ) {
        match key.code {
            KeyCode::Char('j') => {
                let ui_sel_count = *state.ui_sel_count.to_ref();
                let ui_sel = *state.ui_sel_index.to_ref();
                if ui_sel == ui_sel_count - 1 {
                    *state.ui_sel_index.to_mut() = 0;
                } else {
                    *state.ui_sel_index.to_mut() = (ui_sel + 1) % ui_sel_count;
                }
            }
            KeyCode::Char('k') => {
                let ui_sel_count = *state.ui_sel_count.to_ref();
                let ui_sel = *state.ui_sel_index.to_ref();
                if ui_sel == 0 {
                    *state.ui_sel_index.to_mut() = ui_sel_count - 1;
                } else {
                    *state.ui_sel_index.to_mut() = (ui_sel - 1) % ui_sel_count;
                }
            }
            KeyCode::Char('h') => {
                let mut inc = match key.ctrl {
                    true => 1,
                    false => 8,
                };
                match *state.ui_sel_index.to_ref() {
                    0 => {
                        let color = *state.user_color.to_ref();
                        inc = if color.r < inc { color.r } else { inc };
                        *state.bar_r.to_mut() = text_bar_u8_8wide(color.r - inc);
                        *state.user_color.to_mut() = Hex::from((color.r - inc, color.g, color.b));
                    }
                    1 => {
                        let color = *state.user_color.to_ref();
                        inc = if color.g < inc { color.g } else { inc };
                        *state.bar_g.to_mut() = text_bar_u8_8wide(color.g - inc);
                        *state.user_color.to_mut() = Hex::from((color.r, color.g - inc, color.b));
                    }
                    2 => {
                        let color = *state.user_color.to_ref();
                        inc = if color.b < inc { color.b } else { inc };
                        *state.bar_b.to_mut() = text_bar_u8_8wide(color.b - inc);
                        *state.user_color.to_mut() = Hex::from((color.r, color.g, color.b - inc));
                    }
                    _ => {}
                }
            }
            KeyCode::Char('l') => {
                let mut inc = match key.ctrl {
                    true => 1,
                    false => 8,
                };
                match *state.ui_sel_index.to_ref() {
                    0 => {
                        let color = *state.user_color.to_ref();
                        inc = if color.r >= 255 - inc {
                            255 - color.r
                        } else {
                            inc
                        };
                        *state.bar_r.to_mut() = text_bar_u8_8wide(color.r + inc);
                        *state.user_color.to_mut() = Hex::from((color.r + inc, color.g, color.b));
                    }
                    1 => {
                        let color = *state.user_color.to_ref();
                        inc = if color.g >= 255 - inc {
                            255 - color.g
                        } else {
                            inc
                        };
                        *state.bar_g.to_mut() = text_bar_u8_8wide(color.g + inc);
                        *state.user_color.to_mut() = Hex::from((color.r, color.g + inc, color.b));
                    }
                    2 => {
                        let color = *state.user_color.to_ref();
                        inc = if color.b >= 255 - inc {
                            255 - color.b
                        } else {
                            inc
                        };
                        *state.bar_b.to_mut() = text_bar_u8_8wide(color.b + inc);
                        *state.user_color.to_mut() = Hex::from((color.r, color.g, color.b + inc));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
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
        "pick_rgb",
        "src/pick_rgb.aml",
        || PickRgb {},
        || PickRgbState {
            user_color: Hex::from((0, 64, 64)).into(),
            ui_sel_count: 3u8.into(),
            ui_sel_index: 0u8.into(),
            bar_r: text_bar_u8_8wide(64).into(),
            bar_g: text_bar_u8_8wide(64).into(),
            bar_b: text_bar_u8_8wide(64).into(),
        },
    );

    runtime
        .register_component("main", "src/ui.aml", UIMain {}, UIMainState {})
        .unwrap();

    let mut runtime = runtime.finish().unwrap();
    runtime.run();
}
