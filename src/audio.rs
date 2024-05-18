use rodio::{source::SineWave, Sink};

pub fn play_sound() {
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Reproduce un sonido utilizando una onda sinusoidal
    let source = SineWave::new(440).clone().take_duration(std::time::Duration::from_millis(100));
    sink.append(source);

}
