mod stats;
use self::stats::{
    AvgTimeStat,
    TimeStatsGroup
};

/// Maximum time span an logic update may handle.
/// If more time passed, multible calls ensure.
const MAX_DELTA_TIME:        f64  = 1.0 / 30.0;

/// Maximum number of update iterations per frame. If to many happen,
/// logic time starts lagging behind system time.
const MAX_UPDATES_PER_FRAME: uint = 5;

pub trait VideoDriver {
    static fn init() -> Self;
}

pub trait Platform {
    static fn init<D: VideoDriver>() -> Self;
    fn get_time(&self)               -> f64;
}

macro_rules! time_stat (
    (start, var = $deltavar:ident) => (
        let $deltavar = self.platform.get_time()
    );
    (end in $statname:ident, var = $deltavar:ident) => (
        self.time_stats.$statname.add_data_point(
            self.platform.get_time() - $deltavar
        )
    )
)

/// Struct containing the runtime state of the ydengine.
struct EngineState<P, D> {
    platform:   P,
    running:    bool,
    time:       f64,
    time_sys:   f64,
    time_stats: TimeStatsGroup,
}

impl<P: Platform, D: VideoDriver> EngineState<P, D> {
    /// Initializes the engine state, including
    /// associated platform and videodriver.
    static fn new() -> EngineState<P, D> {
        let platform = Platform::init::<P, D>();

        let running    = false;

        let time: f64  = 0.0;
        let time_sys   = platform.get_time();
        let time_stats = TimeStatsGroup::new();

        EngineState {
            platform:   platform,
            running:    running,
            time:       time,
            time_sys:   time_sys,
            time_stats: time_stats,
        }
    }

    /// Main engine loop.
    /// Responsible for logic, rendering and screen refresh.
    fn run(&mut self) {
        while self.is_running() {
            time_stat!(start, var = time_iter_active);
            self.logic();
            self.render();
            time_stat!(end in iteration_active, var = time_iter_active);
            self.refresh();
        }
    }

    fn is_running(&mut self) -> bool {
        false
    }

    /// Do all gamelogic related tasks.
    /// Calls `update()` at least once if time passsed since last
    /// iteration, and more than once if more than `MAX_DELTA_TIME` time
    /// passed, to ensure time-based calculations don't have to handle
    /// potentially infinite large time deltas.
    fn logic(&mut self) {
        let time_sys_new   = self.platform.get_time();
        let mut time_delta = time_sys_new - self.time_sys;
        self.time_sys      = time_sys_new;

        let mut update_iterations: uint = 0;
        while   time_delta > 0.0 &&
                update_iterations < MAX_UPDATES_PER_FRAME {
            let time_delta_remaining =
                f64::min(time_delta, MAX_DELTA_TIME);

            self.update(time_delta_remaining);

            time_delta -= time_delta_remaining;
            self.time  += time_delta_remaining;

            update_iterations += 1;
        }
    }

    fn update(&mut self, time_delta: f64) {

    }

    fn render(&mut self) {

    }

    fn refresh(&mut self) {

    }

}

/// Starts the engine by constructing it's state, handling controll
/// to it's methods.
pub fn run<P: Platform, D: VideoDriver>() {
    let mut state = EngineState::new::<P, D>();
    state.run();
}


