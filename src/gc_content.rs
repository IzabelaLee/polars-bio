use std::sync::Arc;
use arrow_array::{ArrayRef, StringArray};
use arrow_schema::{DataType, Field, Schema};
use datafusion::datasource::MemTable;
use datafusion::execution::context::SessionContext;
use datafusion::physical_plan::udaf::create_udaf;
use datafusion::prelude::*;
use pyo3::prelude::*;
use tokio::runtime::Runtime;
 
use crate::context::PyBioSessionContext;
use crate::dataframe::PyDataFrame;
 
use super::accumulators::GcContentAccumulator;
 
#[pyfunction]
#[pyo3(signature = (py_ctx,))]
pub fn gc_content(
    py: Python<'_>,
    py_ctx: &PyBioSessionContext,
) -> PyResult<PyDataFrame> {
    py.allow_threads(|| {
        let rt = Runtime::new().unwrap();
        let ctx = &py_ctx.ctx.session;
 
        let df = rt.block_on(async {
            let sequences = vec![
                "GCGCGC",
                "ATATAT",
                "GATTACA",
            ];
            let array = Arc::new(StringArray::from(sequences)) as ArrayRef;
            let schema = Arc::new(Schema::new(vec![Field::new("sequence", DataType::Utf8, false)]));
            let batch = RecordBatch::try_new(schema.clone(), vec![array]).unwrap();
 
            let gc_udaf = create_udaf(
                "gc_content",
                vec![DataType::Utf8],
                Arc::new(DataType::Float64),
                Volatility::Immutable,
                Arc::new(|_| Ok(Box::new(GcContentAccumulator::new()))),
                Arc::new(vec![DataType::Utf8]),
            );
 
            ctx.register_udaf(gc_udaf);
 
            let table = MemTable::try_new(batch.schema(), vec![vec![batch]])?;
            ctx.register_table("sequences", Arc::new(table));
 
            let df = ctx.sql("SELECT gc_content(sequence) AS gc_percent FROM sequences").await?;
            Ok(df)
        })?;
 
        Ok(PyDataFrame::new(df))
    })
}