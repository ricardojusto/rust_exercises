require 'benchmark'
require 'ffi'

# ruby only approach (slowest performance without optimizations)
# def fibonacci(n)
#   return n if n <= 1
#   fibonacci(n - 1) + fibonacci(n - 2)
# end

# puts(Benchmark.measure { puts fibonacci(42) })

# module that import rust function
module Fibo
  extend FFI::Library
  ffi_lib 'fibonacci/target/release/libfibonacci.so'
  attach_function :fibonacci, [:int], :int
end

puts(Benchmark.measure { puts Fibo.fibonacci(42) })
