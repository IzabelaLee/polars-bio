from .context import ctx
from polars_bio.polars_bio import py_gc_content
 
 
def gc_content():
    return py_gc_content(ctx)