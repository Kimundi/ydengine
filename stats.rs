pub struct AvgTimeStat {
    accum: f64,
    min: f64,
    max: f64
}
pub impl AvgTimeStat {
    static fn new() -> AvgTimeStat {
        AvgTimeStat {
            accum: 0.,
            min:  -1.,
            max:  -1.
        }
    }

    fn reset(&mut self) {
        self.accum =  0.;
        self.min   = -1.;
        self.max   = -1.;
    }
}

pub struct TimeStatsGroup {
    update: AvgTimeStat,
    render: AvgTimeStat,
    idle:   AvgTimeStat,
}
pub impl TimeStatsGroup {
    static fn new() -> TimeStatsGroup {
        TimeStatsGroup {
            update: AvgTimeStat::new(),
            render: AvgTimeStat::new(),
            idle:   AvgTimeStat::new(),
        }
    }
}
