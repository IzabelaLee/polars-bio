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
                continue;
            }

            let s = string_array.value(i);
            let mut gc = 0;
            let mut total = 0;

            for b in s.bytes().filter(|&b| b != b'\n') {
                match b {
                    b'G' | b'g' | b'C' | b'c' => {
                        gc += 1;
                        total += 1;
                    }
                    b'N' | b'n' => {}
                    _ => total += 1,
                }
            }

            let gc_content = if total > 0 {
                (gc as f64) / (total as f64) * 100.0
            } else {
                0.0
            };

            results.push(ScalarValue::Float64(Some(gc_content)));
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
