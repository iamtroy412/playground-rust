fn main() {
    // Like structs, the compiler will implement things like == for you,
    // but you have to ask.
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum TimeUnit {
        Seconds,
        Minutes,
        Hours,
        Days,
        Months,
        Years,
    }

    // Enums can have methods like structs
    impl TimeUnit {
        // Return the plural noun for this time unit.
        fn plural(self) -> &'static str {
            match self {
                TimeUnit::Seconds => "seconds",
                TimeUnit::Minutes => "minutes",
                TimeUnit::Hours => "hours",
                TimeUnit::Days => "days",
                TimeUnit::Months => "months",
                TimeUnit::Years => "years",
            }
        }

        // Return the singular noun for this time unit.
        fn singular(self) -> &'static str {
            self.plural().trim_end_matches('s')
        }
    }

    // How to get values out of an enum with data
    enum RoughTime {
        InThePast(TimeUnit, u32),
        JustNow,
        InTheFuture(TimeUnit, u32),
    }

    fn rough_time_to_english(rt: RoughTime) -> String {
        match rt {
            RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
            RoughTime::JustNow => format!("just now"),
            RoughTime::InTheFuture(units, count) => {
                format!("{} {} from now", count, units.plural())
            }
        }
    }
}
