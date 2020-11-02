// This code is from https://stackoverflow.com/questions/56384447/how-do-i-transform-special-values-into-optionnone-when-using-serde-to-deserial
use serde::de::Deserializer;
use serde::Deserialize;

pub fn deserialize_maybe_nan<'de, D, T: Deserialize<'de>>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
{
    // we define a local enum type inside of the function
    // because it is untagged, serde will deserialize as the first variant
    // that it can
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum MaybeNA<U> {
        // if it can be parsed as Option<T>, it will be
        Value(Option<U>),
        // otherwise try parsing as a string
        NAString(String),
    }

    // deserialize into local enum
    let value: MaybeNA<T> = Deserialize::deserialize(deserializer)?;
    match value {
        // if parsed as T or None, return that
        MaybeNA::Value(value) => Ok(value),

        // otherwise, if value is string an "n/a", return None
        // (and fail if it is any other string)
        MaybeNA::NAString(string) => {
            if string == "n/a" {
                Ok(None)
            } else {
                Err(serde::de::Error::custom("Unexpected string"))
            }
        }
    }
}
