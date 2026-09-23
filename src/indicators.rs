use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressBarWrapper {
    bar: ProgressBar,
}

impl ProgressBarWrapper {
    pub fn inc(&self, n: u64) {
        self.bar.inc(n);
    }

    pub fn finish(&self) {
        self.bar.finish();
    }
}

pub fn progress_bar(total: u64, prefix: &str) -> ProgressBarWrapper {
    let bar = ProgressBar::new(total);
    let bar_prefix = prefix.to_string();
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:12.cyan.bold} [{elapsed_precise}] [{bar:30}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("=> "),
    );
    bar.set_prefix(bar_prefix);
    ProgressBarWrapper { bar }
}
