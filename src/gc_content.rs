use std::sync::Arc;

use arrow_array::{Array, StringArray};
use arrow_schema::DataType;

use datafusion::common::{DataFusionError, Result, ScalarValue};
use datafusion::logical_expr::{create_udf, ColumnarValue, ScalarUDF, Volatility};

pub fn create_gc_content_udf() -> ScalarUDF {
    let func = |args: &[ColumnarValue]| -> Result<ColumnarValue> {
        let string_array = match &args[0] {
            ColumnarValue::Array(array) => array
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or_else(|| DataFusionError::Execution("Expected StringArray".to_string()))?,
            ColumnarValue::Scalar(_) => {
                return Err(DataFusionError::Execution(
                    "Expected array input, got scalar".to_string(),
                ))
            }
        };

        let mut results = Vec::with_capacity(string_array.len());

        for i in 0..string_array.len() {
            if string_array.is_null(i) {
                results.push(ScalarValue::Float64(None));
            } else {
                let s = string_array.value(i);
                let sequence: Vec<u8> = s
                    .lines()
                    .flat_map(|line| line.as_bytes().to_vec())
                    .collect();

                let gc = sequence.iter()
                    .filter(|&&b| b == b'G' || b == b'g' || b == b'C' || b == b'c')
                    .count() as f64;

                let without_n = sequence.iter()
                    .filter(|&&b| b != b'N' && b != b'n')
                    .count() as f64;

                let gc_content = if without_n > 0.0 {
                    gc / without_n * 100.0
                } else {
                    0.0
                };

                results.push(ScalarValue::Float64(Some(gc_content)));
            }
        }

        let array = ScalarValue::iter_to_array(results.into_iter())?;
        Ok(ColumnarValue::Array(array))
    };

    create_udf(
        "gc_content",
        vec![DataType::Utf8],
        DataType::Float64,
        Volatility::Immutable,
        Arc::new(func),
    )
}
