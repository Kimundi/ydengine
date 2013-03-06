pub struct AvgTimeStat {
    average: f64,
    min: f64,
    max: f64,

    priv accum: f64,
    priv count: uint,
}
pub impl AvgTimeStat {
    static fn new() -> AvgTimeStat {
        AvgTimeStat {
            average: 0.0,
            min:    -1.0,
            max:    -1.0,

            accum:   0.0,
            count:   0,
        }
    }

    fn reset(&mut self) {
        self.average =  0.0;
        self.min     = -1.0;
        self.max     = -1.0;

        self.accum   =  0.0;
        self.count   =  0;
    }

    fn add_data_point(&mut self, data: f64) {
        self.accum += data;
        self.count += 1;
        self.min =
            if self.min < 0.0 { data } else { f64::min(data, self.min) };
        self.max =
            if self.max < 0.0 { data } else { f64::max(data, self.max) };
        self.average = self.accum / (self.count as f64);
    }
}

pub struct TimeStatsGroup {
    logic:            AvgTimeStat,
    logic_update:     AvgTimeStat,
    render:           AvgTimeStat,
    iteration_active: AvgTimeStat,
    iteration_idle:   AvgTimeStat,
}
pub impl TimeStatsGroup {
    static fn new() -> TimeStatsGroup {
        TimeStatsGroup {
            logic:            AvgTimeStat::new(),
            logic_update:     AvgTimeStat::new(),
            render:           AvgTimeStat::new(),
            iteration_active: AvgTimeStat::new(),
            iteration_idle:   AvgTimeStat::new(),
        }
    }
}
