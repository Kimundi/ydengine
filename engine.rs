pub trait VideoDriver {
    static fn init() -> Self;
}

pub trait Platform {
    static fn init<D: VideoDriver>() -> Self;
}

struct AvgTimeSpan {
    accum: f64,
    min: f64,
    max: f64
}

struct AvgTimeSpanTuple {
    update: AvgTimeSpan,
    render: AvgTimeSpan,
    idle:   AvgTimeSpan,
}

pub fn run<P: Platform, D: VideoDriver>() {
    const MAX_DELTA_TIME: f64         = 1.0 / 30.0;
    const MAX_UPDATES_PER_FRAME: uint = 5;

    let platform: P = Platform::init::<P, D>();

    let mut running: bool;
    let mut time:    f64;



}
