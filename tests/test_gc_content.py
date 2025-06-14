import time
import pytest
import pandas as pd
import polars_bio as pb

from _expected import EXPECTED_SQL_GC_DF, DF_INPUT_GC_CONTENT_PATH

class TestSQLGCContent:
    @classmethod
    def setup_class(cls):
        cls.df = pb.read_fastq(DF_INPUT_GC_CONTENT_PATH).collect()
        pb.sql("select * from reads").collect()

        cls.result = pb.sql("select sequence, gc_content(sequence) as gc from reads").collect()

        cls.result_pd = cls.result.to_pandas()
        cls.expected = EXPECTED_SQL_GC_DF

    def test_gc_content_shape(self):
        assert self.result_pd.shape == self.expected.shape

    def test_gc_content_columns(self):
        assert list(self.result_pd.columns) == ["sequence", "gc"]

    def test_gc_content_dtypes(self):
        dtypes = self.result_pd.dtypes.to_dict()
        assert dtypes["sequence"] == "object"
        assert pd.api.types.is_float_dtype(dtypes["gc"])

    def test_gc_content_values(self):
        result_sorted = self.result_pd.sort_values(by="sequence").reset_index(drop=True)
        expected_sorted = self.expected.sort_values(by="sequence").reset_index(drop=True)

        expected_sorted = expected_sorted.astype(result_sorted.dtypes.to_dict())

        pd.testing.assert_frame_equal(result_sorted, expected_sorted)
