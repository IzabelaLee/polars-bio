from .context import ctx
from polars_bio import py_gc_content

def gc_content(path: str):
    return py_gc_content(ctx, path)
