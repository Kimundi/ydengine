mod stats;
use self::stats::{
    AvgTimeStat,
    TimeStatsGroup
};

pub trait VideoDriver {
    static fn init() -> Self;
}

pub trait Platform {
    static fn init<D: VideoDriver>() -> Self;
    fn foo(&self);
}

pub fn run<P: Platform, D: VideoDriver>() {
    let mut state = EngineState::new::<P, D>();
    state.run();
}

const MAX_DELTA_TIME: f64         = 1.0 / 30.0;
const MAX_UPDATES_PER_FRAME: uint = 5;

struct EngineState<P, D> {
    platform:   P,
    running:    bool,

    time:       f64,
    time_stats: TimeStatsGroup,
}
impl<P: Platform, D: VideoDriver> EngineState<P, D> {
    static fn new() -> EngineState<P, D> {
        let running    = false;

        let time: f64  = 0.0;
        let time_stats = TimeStatsGroup::new();

        let platform = Platform::init::<P, D>();

        EngineState {
            platform: platform,
            running: running,
            time: time,
            time_stats: time_stats,
        }
    }

    fn run(&mut self) {
        while self.is_running() {
            self.logic();
            self.render();
            self.refresh();
        }
    }

    fn stuff(&mut self) {
        self.platform.foo();
    }

    fn logic(&mut self) {

    }

    fn render(&mut self) {

    }

    fn refresh(&mut self) {

    }

    fn is_running(&mut self) -> bool {
        false
    }


}




