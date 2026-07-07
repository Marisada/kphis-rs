use derive_demo::Demo;
use serde_derive::{Deserialize, Serialize};
use time::{Date, Duration, Time};
use utoipa::ToSchema;

pub struct Shift {
    pub shift: NurseShift,
    pub duration: String,
    pub detail: String,
}

/// Nursing shift
#[derive(Clone, Debug, Demo, Deserialize, Serialize, PartialEq, ToSchema)]
#[schema(example = json!(NurseShift::demo_day()))]
pub enum NurseShift {
    Day,
    Evening,
    Night,
}

impl NurseShift {
    pub fn short(&self) -> &'static str {
        match self {
            NurseShift::Day => "ช.",
            NurseShift::Evening => "บ.",
            NurseShift::Night => "ด.",
        }
    }
    pub fn long(&self) -> &'static str {
        match self {
            NurseShift::Day => "เช้า",
            NurseShift::Evening => "บ่าย",
            NurseShift::Night => "ดึก",
        }
    }
    /// return ('date of shift', shift)
    /// start time is `the end of previous shift`. Ex shift_day_start is 08:00 mean night shift ended 08:00, day shift start 08.00:01
    /// ex: if night shift start after 22:00 then at 24-01-2024 23:45 is night-shift of 25-01-2024
    /// so generate(24-01-2024,23:45) -> (25-01-2024, Night)
    pub fn generate(shift_day_start: Time, shift_evening_start: Time, shift_night_start: Time, date: Date, time: Time) -> (Date, Self) {
        let one_day = Duration::days(1);
        let noon = Time::from_hms(12, 0, 0).unwrap();

        // ex: | 0-8 | 8-16 | 16-24 |
        if shift_night_start == Time::MIDNIGHT {
            if time == Time::MIDNIGHT {
                (date - one_day, Self::Evening)
            } else if time > shift_evening_start {
                (date, Self::Evening)
            } else if time > shift_day_start {
                (date, Self::Day)
            } else {
                (date, Self::Night)
            }
        // ex: | 22-6 | 6-14 | 14-22 |
        } else if shift_night_start > noon {
            if time > shift_night_start {
                (date + one_day, Self::Night)
            } else if time > shift_evening_start {
                (date, Self::Evening)
            } else if time > shift_day_start {
                (date, Self::Day)
            } else {
                (date, Self::Night)
            }
        // ex: | 2-10 | 10-18 | 18-2 |
        } else if time > shift_evening_start {
            (date, Self::Evening)
        } else if time > shift_day_start {
            (date, Self::Day)
        } else if time > shift_night_start {
            (date, Self::Night)
        } else {
            (date - one_day, Self::Evening)
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
pub mod tests {

    use time::macros::{date, time};
    use super::*;

    #[test]
    fn test_generate() {
        //                        START-TIME:  DAY         EVENING        NIGHT  |  GEN:   DATE            TIME      TO       DATE                  SHIFT
        assert_eq!(NurseShift::generate(time!(06:00), time!(14:00), time!(22:00), date!(2024-01-09), time!(23:00)), (date!(2024-01-10), NurseShift::Night));
        assert_eq!(NurseShift::generate(time!(06:00), time!(14:00), time!(22:00), date!(2024-01-10), time!(07:00)), (date!(2024-01-10), NurseShift::Day));
        assert_eq!(NurseShift::generate(time!(06:00), time!(14:00), time!(22:00), date!(2024-01-10), time!(15:00)), (date!(2024-01-10), NurseShift::Evening));

        assert_eq!(NurseShift::generate(time!(08:00), time!(16:00), time!(00:00), date!(2024-01-10), time!(01:00)), (date!(2024-01-10), NurseShift::Night));
        assert_eq!(NurseShift::generate(time!(08:00), time!(16:00), time!(00:00), date!(2024-01-10), time!(09:00)), (date!(2024-01-10), NurseShift::Day));
        assert_eq!(NurseShift::generate(time!(08:00), time!(16:00), time!(00:00), date!(2024-01-10), time!(17:00)), (date!(2024-01-10), NurseShift::Evening));

        assert_eq!(NurseShift::generate(time!(10:00), time!(18:00), time!(02:00), date!(2024-01-10), time!(01:00)), (date!(2024-01-09), NurseShift::Evening));
        assert_eq!(NurseShift::generate(time!(10:00), time!(18:00), time!(02:00), date!(2024-01-10), time!(09:00)), (date!(2024-01-10), NurseShift::Night));
        assert_eq!(NurseShift::generate(time!(10:00), time!(18:00), time!(02:00), date!(2024-01-10), time!(17:00)), (date!(2024-01-10), NurseShift::Day));

        // at start time
        assert_eq!(NurseShift::generate(time!(08:00), time!(16:00), time!(00:00), date!(2024-01-10), time!(00:00)), (date!(2024-01-09), NurseShift::Evening));
        assert_eq!(NurseShift::generate(time!(08:00), time!(16:00), time!(00:00), date!(2024-01-10), time!(08:00)), (date!(2024-01-10), NurseShift::Night));
        assert_eq!(NurseShift::generate(time!(08:00), time!(16:00), time!(00:00), date!(2024-01-10), time!(16:00)), (date!(2024-01-10), NurseShift::Day));
    }
}
