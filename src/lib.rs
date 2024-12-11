pub use term_size;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub enum LoadingBarType {
    Standard,
    Wave,
}

pub struct LoadingBarBuilder {
    len: usize,
    should_clear: bool,
    bar_type: LoadingBarType,
}

impl LoadingBarBuilder {
    pub fn should_clear(self) -> Self {
        Self {
            should_clear: true,
            ..self
        }
    }

    pub fn of_type(self, bar_type: LoadingBarType) -> Self {
        Self { bar_type, ..self }
    }

    pub fn init(self) -> LoadingBar {
        let l = LoadingBar {
            len: self.len,
            step: 0,
            should_clear: self.should_clear,
            bar_type: self.bar_type,
        };
        l.render();
        l
    }
}

pub struct LoadingBar {
    len: usize,
    step: usize,
    should_clear: bool,
    bar_type: LoadingBarType,
}

impl LoadingBar {
    pub fn new(len: usize) -> LoadingBarBuilder {
        LoadingBarBuilder {
            len,
            should_clear: false,
            bar_type: LoadingBarType::Standard,
        }
    }

    pub fn step(&mut self) {
        self.step += 1;
        self.render();
    }

    fn render(&self) {
        if self.should_clear {
            print!("\x1b[2J");
            print!("\x1b[H");
        }
        if let Some((w, h)) = term_size::dimensions() {
            match self.bar_type {
                LoadingBarType::Standard => self.render_standard((w, h)),
                LoadingBarType::Wave => self.render_wave((w, h)),
            }
        };
    }

    fn render_standard(&self, (w, h): (usize, usize)) {
        let finished = self.step as f64 / self.len as f64;
        let len = w - 30;
        let bar_len = (len as f64 * finished) as usize;
        println!(
            "loading {}/{} |{:█<4$}{:<5$}|",
            self.step,
            self.len,
            "",
            "",
            bar_len,
            len - bar_len
        );
    }

    fn render_wave(&self, (w, h): (usize, usize)) {
        let finished = self.step as f64 / self.len as f64;
        let loading_string = self.loading_string(Some("loading"));
        let len = w - loading_string.len() - 2;
        let bar_len = (len as f64 * finished) as usize;
        let mut lines = vec![String::new(); 5];
        for (i, line) in lines.iter_mut().enumerate() {
            if i == 2 {
                line.push_str(&format!("{}|", loading_string));
            } else {
                line.push_str(&format!("{:<1$}|", "", loading_string.len()));
            }
            for x in 0..len {
                if i == ((((x as f32 / 5. + self.step as f32 * 6. / (self.len as f32)).sin()) * 2.5
                    + 2.5) as usize)
                    .min(5)
                    && x < bar_len
                {
                    line.push('█');
                } else {
                    line.push(' ');
                };
            }
            line.push('|');
            println!("{}", line);
        }
    }

    fn loading_string(&self, with_text: Option<&str>) -> String {
        if let Some(x) = with_text {
            format!("{} {}/{} ", x, self.step, self.len)
        } else {
            format!(" {}/{} ", self.step, self.len)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::{self, sleep};
    use std::time::Duration;

    // #[test]
    // fn standard() {
    //     let len = 100;
    //     let mut loader = LoadingBar::new(len).should_clear().init();
    //     for i in 0..100 {
    //         loader.step();
    //         thread::sleep_ms(10);
    //     }
    // }

    #[test]
    fn wave() {
        let len = 100;
        let mut loader = LoadingBar::new(len)
            .should_clear()
            .of_type(LoadingBarType::Wave)
            .init();

        for i in 0..100 {
            loader.step();
            thread::sleep_ms(10);
        }
    }
}
