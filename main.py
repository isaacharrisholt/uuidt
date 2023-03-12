import timeit

import uuidt
import pyimpl


rs = timeit.timeit('uuidt.new("Test")', number=1_000_000, globals=globals())
py = timeit.timeit('pyimpl.new("Test")', number=1_000_000, globals=globals())

print(f'rs: {rs}')
print(f'py: {py}')
