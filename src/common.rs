//! Common operations
//!

#[cfg(not(target_arch = "wasm32"))]
use polars::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
/// Converts a Dataframe into a String, this is expensive but useful for small results.
pub fn convert_df_to_json_data(df: &DataFrame) -> Result<String, crate::error::Error> {
    let mut buf = Vec::new();
    JsonWriter::new(&mut buf)
        .with_json_format(JsonFormat::Json)
        .finish(&mut df.clone())?;
    Ok(String::from_utf8(buf)?)
}
