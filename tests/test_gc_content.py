import pandas as pd
from polars_bio.polars_bio import fastq_gc_dataframe

EXPECTED_PATH = "tests/data/gc_content/target.csv"
INPUT_FASTQ = "tests/data/gc_content/reads.fastq"

class TestGCContent:
    @classmethod
    def setup_class(cls):
        cls.df = fastq_gc_dataframe(INPUT_FASTQ).to_pandas()
        cls.expected = pd.read_csv(EXPECTED_PATH)

    def test_gc_content_shape(self):
        assert self.df.shape == self.expected.shape

    def test_gc_content_columns(self):
        assert list(self.df.columns) == [
            "id", "G_count", "C_count", "GC_count", "len", "GC_content"
        ]

    def test_gc_content_schema(self):
        dtypes = self.df.dtypes.to_dict()
        assert dtypes["id"] == "object"

        assert pd.api.types.is_unsigned_integer_dtype(dtypes["G_count"])
        assert pd.api.types.is_unsigned_integer_dtype(dtypes["C_count"])
        assert pd.api.types.is_unsigned_integer_dtype(dtypes["GC_count"])
        assert pd.api.types.is_unsigned_integer_dtype(dtypes["len"])
        assert pd.api.types.is_float_dtype(dtypes["GC_content"])

    def test_gc_content_values(self):
        df_sorted = self.df.sort_values(by="id").reset_index(drop=True)
        expected_sorted = self.expected.sort_values(by="id").reset_index(drop=True)

        expected_sorted = expected_sorted.astype(df_sorted.dtypes.to_dict())

        pd.testing.assert_frame_equal(df_sorted, expected_sorted)
