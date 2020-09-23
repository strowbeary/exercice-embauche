use rocket::request::FromFormValue;
use chrono::{Date, Utc, TimeZone, NaiveDate};
use rocket::http::RawStr;

pub struct DateForm(pub Date<Utc>);

impl<'v> FromFormValue<'v> for DateForm {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, Self::Error> {
        println!("DATE : {}", form_value.as_str());

        let date = NaiveDate::parse_from_str(
            form_value.as_str(),
            "%Y-%m-%d",
        );
        match date {
            Ok(parsed_date) => {
                Ok(DateForm(Utc.from_utc_date(&parsed_date)))
            },
            Err(_) => {
                Err(RawStr::from_str("Date format invalid"))
            },
        }


    }
}