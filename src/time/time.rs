use crate::utils::constants::DAY_S;

#[derive(Debug, Clone)]
pub struct Time {
    pub whole_days: u64,
    pub whole_seconds: u64,
    pub fractional_seconds: f64,
}

impl Time {
    pub fn new(whole_days: u64, whole_seconds: u64, fractional_seconds: f64) -> Self {
        Self {
            whole_days,
            whole_seconds,
            fractional_seconds,
        }
    }

    pub fn set_whole_days(&mut self, whole_days: u64) {
        self.whole_days = whole_days;
    }

    pub fn set_whole_seconds(&mut self, whole_seconds: u64) {
        self.whole_seconds = whole_seconds;
    }

    pub fn set_fractional_seconds(&mut self, fractional_seconds: f64) {
        self.fractional_seconds = fractional_seconds;
    }

    pub fn set_julian_days(&mut self, jd: f64) {
        let (days_whole, seconds_whole, seconds_fraction) = from_jd_to_tuple(jd);
        self.whole_days = days_whole;
        self.whole_seconds = seconds_whole;
        self.fractional_seconds = seconds_fraction;
    }
}

pub fn from_jd_to_tuple(jd: f64) -> (u64, u64, f64) {
    let (days, fractional_days) = compute_quotient_and_remainder(jd, 1.0);
    let (seconds, seconds_fraction) = compute_quotient_and_remainder(fractional_days * DAY_S, 1.0);
    return (days, seconds, seconds_fraction);
}

pub fn compute_quotient_and_remainder(numerator: f64, denominator: f64) -> (u64, f64) {
    ((numerator / denominator) as u64, numerator % denominator)
}