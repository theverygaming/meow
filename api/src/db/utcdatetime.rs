use rocket::serde::{Deserialize, Deserializer, Serialize, Serializer};

// I have no fucking clue what I'm doing I just consulted ChatGPT about all of this at fucking 4am it's so over

#[derive(Debug, Clone, Copy, diesel::AsExpression, diesel::FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Timestamp)]
pub struct UTCDateTime(chrono::NaiveDateTime);

impl<'de> Deserialize<'de> for UTCDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use chrono::Offset;
        let s = String::deserialize(deserializer)?;

        let parsed = chrono::DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;

        // enforce UTC
        if parsed.timezone() != chrono::Utc.fix() {
            return Err(serde::de::Error::custom("Time is not in UTC"));
        }

        Ok(UTCDateTime(parsed.naive_utc()))
    }
}

impl diesel::deserialize::FromSql<diesel::sql_types::Timestamp, diesel::pg::Pg> for UTCDateTime {
    fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let naive = <chrono::NaiveDateTime as diesel::deserialize::FromSql<
            diesel::sql_types::Timestamp,
            diesel::pg::Pg,
        >>::from_sql(bytes)?;
        Ok(UTCDateTime(naive))
    }
}

impl Serialize for UTCDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.0.and_utc().to_rfc3339())
    }
}

impl diesel::serialize::ToSql<diesel::sql_types::Timestamp, diesel::pg::Pg> for UTCDateTime {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, diesel::pg::Pg>,
    ) -> diesel::serialize::Result {
        <chrono::NaiveDateTime as diesel::serialize::ToSql<
            diesel::sql_types::Timestamp,
            diesel::pg::Pg,
        >>::to_sql(&self.0, out)
    }
}
