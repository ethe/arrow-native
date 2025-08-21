use arrow_array::types::{
    IntervalDayTime as IntervalDayTimeNative, IntervalMonthDayNano as IntervalMonthDayNanoNative,
};
use arrow_schema::{DataType, IntervalUnit};
use typed_arrow::{bridge::ArrowBinding, IntervalDayTime, IntervalMonthDayNano, IntervalYearMonth};

#[test]
fn interval_year_month() {
    assert_eq!(
        <IntervalYearMonth as ArrowBinding>::data_type(),
        DataType::Interval(IntervalUnit::YearMonth)
    );
    let mut b = <IntervalYearMonth as ArrowBinding>::new_builder(2);
    <IntervalYearMonth as ArrowBinding>::append_value(&mut b, &IntervalYearMonth::new(12));
    <IntervalYearMonth as ArrowBinding>::append_null(&mut b);
    let a = <IntervalYearMonth as ArrowBinding>::finish(b);
    assert_eq!(a.len(), 2);
}

#[test]
fn interval_day_time() {
    assert_eq!(
        <IntervalDayTime as ArrowBinding>::data_type(),
        DataType::Interval(IntervalUnit::DayTime)
    );
    let mut b = <IntervalDayTime as ArrowBinding>::new_builder(2);
    let v = IntervalDayTime::new(IntervalDayTimeNative {
        days: 0,
        milliseconds: 0,
    });
    <IntervalDayTime as ArrowBinding>::append_value(&mut b, &v);
    <IntervalDayTime as ArrowBinding>::append_null(&mut b);
    let a = <IntervalDayTime as ArrowBinding>::finish(b);
    assert_eq!(a.len(), 2);
}

#[test]
fn interval_month_day_nano() {
    assert_eq!(
        <IntervalMonthDayNano as ArrowBinding>::data_type(),
        DataType::Interval(IntervalUnit::MonthDayNano)
    );
    let mut b = <IntervalMonthDayNano as ArrowBinding>::new_builder(2);
    let v = IntervalMonthDayNano::new(IntervalMonthDayNanoNative {
        months: 0,
        days: 0,
        nanoseconds: 0,
    });
    <IntervalMonthDayNano as ArrowBinding>::append_value(&mut b, &v);
    <IntervalMonthDayNano as ArrowBinding>::append_null(&mut b);
    let a = <IntervalMonthDayNano as ArrowBinding>::finish(b);
    assert_eq!(a.len(), 2);
}
