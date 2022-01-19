use chrono::NaiveDateTime;
use serde::{de::Visitor, Deserializer, Serializer};

pub fn serialize<S: Serializer>(date_time: &NaiveDateTime, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&i64::to_string(&date_time.timestamp_millis()))
}

pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<NaiveDateTime, D::Error> {
    struct V;

    impl Visitor<'_> for V {
        type Value = NaiveDateTime;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing a millisecond timestamp")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            let milliseconds = v.parse::<i64>().map_err(E::custom)?;
            let seconds = milliseconds / 1000;
            let milliseconds = (milliseconds % 1000) as u32;
            let nanos = milliseconds * 1_000_000;

            Ok(NaiveDateTime::from_timestamp(seconds, nanos))
        }
    }

    d.deserialize_str(V)
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDateTime, Utc, TimeZone, DateTime};
    use serde::{Deserialize, Serialize};
    use serde_json::{from_str, from_value, json, to_string};

    #[derive(Deserialize, Serialize)]
    struct Foo {
        #[serde(with = "crate::ts_milliseconds_str")]
        time: NaiveDateTime,
    }

    const TIMESTAMP_MS: i64 = 1640995200000;


    #[test]
    fn can_deserialize() {
        let date_time = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0);
        let foo: Foo = from_value(json!({"time": TIMESTAMP_MS.to_string()})).unwrap();
        let time = Utc.from_utc_datetime(&foo.time);
        assert_eq!(time, date_time);
    }

    #[test]
    fn round_trip() {
        let foo: Foo = from_value(json!({"time": TIMESTAMP_MS.to_string()})).unwrap();
        let s = to_string(&foo).unwrap();
        let foo_again: Foo = from_str(&s).unwrap();
        assert_eq!(foo.time, foo_again.time);
    }
}
