try:
  from cffi import FFI
except ImportError:
  print("pip install cffi, include with pypy")
  exit()

ffi = FFI()
lib = ffi.dlopen("./librust_math.dylib")

print(lib)

ffi.cdef("int treble(int);")

print("math from rust ! %s" % lib.treble(10))
