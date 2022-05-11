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
            RoughTime::InThePast(TimeUnit::Hours, 1) => format!("an hour ago"),
            RoughTime::InThePast(units, 1) => format!("a {} ago", units.singular()),
            RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
            RoughTime::JustNow => format!("just now"),
            RoughTime::InTheFuture(TimeUnit::Hours, 1) => format!("an hour from now"),
            RoughTime::InTheFuture(units, 1) => format!("a {} from now", units.singular()),
            RoughTime::InTheFuture(units, count) => {
                format!("{} {} from now", count, units.plural())
            }
        }
    }

    assert_eq!(
        "2 months from now",
        rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Months, 2))
    );
    assert_eq!(
        "a month from now",
        rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Months, 1))
    );
    assert_eq!(
        "an hour from now",
        rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Hours, 1))
    );
    assert_eq!(
        "an hour ago",
        rough_time_to_english(RoughTime::InThePast(TimeUnit::Hours, 1))
    );
    assert_eq!(
        "a year ago",
        rough_time_to_english(RoughTime::InThePast(TimeUnit::Years, 1))
    );
    println!(
        "{}",
        rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Months, 2))
    );

    // Tuple patterns match tuples. They're useful any time you want to get multiple
    // pieces of data involved in a single match
    fn describe_point(x: i32, y: i32) -> &'static str {
        use std::cmp::Ordering::*;
        match (x.cmp(&0), y.cmp(&0)) {
            (Equal, Equal) => "at the origin",
            (_, Equal) => "on the x axis",
            (Equal, _) => "on the y axis",
            (Greater, Greater) => "in the first quadrant",
            (Less, Greater) => "in the second quadrant",
            _ => "somewhere else",
        }
    }

    assert_eq!("somewhere else", describe_point(-100, -42));
    println!("{}", describe_point(42, 42));
}
