/// Generate a kick-ass beat
///
/// This macro creates a `main` function. Just put nothing but the import and a
/// macro call in a file and `cargo run` it:
///
/// ```
/// #[macro_use] extern crate macromusic;
/// sick_beat!(400 4, 460 2, 460 1, 400 4, 360 3);
/// ```
#[macro_export]
macro_rules! sick_beat {
    ($($freq:tt $duration:tt),+) => {
        extern crate rodio;
        use std::time::Duration;
        use rodio::Source;

        fn main() {
            let endpoint = rodio::default_endpoint().unwrap();

            let sink = rodio::Sink::new(&endpoint);

            $(
                sink.append(
                    rodio::source::SineWave::new($freq).take_duration(Duration::from_millis($duration * 75))
                );
            )*

            sink.sleep_until_end();
        }
    };
}
