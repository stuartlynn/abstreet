use crate::{text, Canvas, GfxCtx, InputResult, Text, UserInput, CENTERED};
use piston::input::{Button, Key, PressEvent};

// Stores some associated data with each choice
pub struct Menu<T: Clone> {
    prompt: String,
    choices: Vec<(String, T)>,
    current_idx: usize,
}

impl<T: Clone> Menu<T> {
    pub fn new(prompt: &str, choices: Vec<(String, T)>) -> Menu<T> {
        if choices.is_empty() {
            panic!("Can't create a menu without choices for \"{}\"", prompt);
        }
        Menu {
            prompt: prompt.to_string(),
            choices,
            current_idx: 0,
        }
    }

    pub fn event(&mut self, input: &mut UserInput) -> InputResult<T> {
        let ev = input.use_event_directly().clone();

        if let Some(Button::Keyboard(Key::Escape)) = ev.press_args() {
            return InputResult::Canceled;
        }

        if let Some(Button::Keyboard(Key::Return)) = ev.press_args() {
            // TODO instead of requiring clone, we could drain choices to take ownership of the
            // item. but without consuming self here, it's a bit sketchy to do that.
            let (name, item) = self.choices[self.current_idx].clone();
            return InputResult::Done(name, item);
        }

        if let Some(Button::Keyboard(Key::Up)) = ev.press_args() {
            if self.current_idx > 0 {
                self.current_idx -= 1;
            }
        }
        if let Some(Button::Keyboard(Key::Down)) = ev.press_args() {
            if self.current_idx < self.choices.len() - 1 {
                self.current_idx += 1;
            }
        }

        InputResult::StillActive
    }

    pub fn draw(&self, g: &mut GfxCtx, canvas: &Canvas) {
        let mut txt = Text::new();
        txt.add_styled_line(
            self.prompt.clone(),
            text::TEXT_FG_COLOR,
            Some(text::TEXT_QUERY_COLOR),
        );

        // TODO Silly results from doing this:
        // - The menu width changes as we scroll
        // - Some off-by-one / usize rounding bugs causing menu height to change a bit

        // How many lines can we fit on the screen?
        let can_fit = {
            // Subtract 1 for the prompt, and an additional TODO hacky
            // few to avoid the bottom OSD and stuff.
            let n =
                (f64::from(canvas.window_size.height) / text::LINE_HEIGHT).floor() as isize - 1 - 6;
            if n <= 0 {
                // Weird small window, just display the prompt and bail out.
                canvas.draw_text(g, txt, CENTERED);
                return;
            }
            n as usize
        };

        let low_idx = if self.choices.len() <= can_fit {
            0
        } else {
            let middle = can_fit / 2;
            if self.current_idx >= middle {
                (self.current_idx - middle).min(self.choices.len() - (middle * 2))
            } else {
                0
            }
        };

        for (idx, (line, _)) in self.choices.iter().enumerate() {
            if idx < low_idx || idx > low_idx + can_fit {
                continue;
            }
            if self.current_idx == idx {
                txt.add_styled_line(
                    line.clone(),
                    text::TEXT_FG_COLOR,
                    Some(text::TEXT_FOCUS_COLOR),
                );
            } else {
                txt.add_line(line.clone());
            }
        }

        canvas.draw_text(g, txt, CENTERED);
    }

    pub fn current_choice(&self) -> &T {
        &self.choices[self.current_idx].1
    }
}
