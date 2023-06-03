use chrono::NaiveDate;

use crate::error::Error;

pub struct MyNaiveDate(NaiveDate);

impl TryFrom<&str> for MyNaiveDate {
    type Error = Error;

    /*
            fn string_to_naive_date(string:&str) -> Result<NaiveDate, Error>{
            let date: NaiveDate;
            if string.contains("/") {
                date = NaiveDate::parse_from_str(string, "%d/%m/%Y")?;
            } else if string.contains("-") {
                date = NaiveDate::parse_from_str(string, "%d-%m-%Y")?;
            } else {
                date = NaiveDate::parse_from_str(string, "%d%m%Y")?;
            }
            Ok(date)
        }
     */

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let date: NaiveDate;
        if value.contains("/") {
            date = NaiveDate::parse_from_str(value, "%d/%m/%Y")?;
        } else if value.contains("-") {
            date = NaiveDate::parse_from_str(value, "%d-%m-%Y")?;
        } else {
            date = NaiveDate::parse_from_str(value, "%d%m%Y")?;
        }
        Ok(MyNaiveDate(date))
    }
}

impl From<MyNaiveDate> for NaiveDate {
    fn from(date: MyNaiveDate) -> Self {
        date.0
    }
}
