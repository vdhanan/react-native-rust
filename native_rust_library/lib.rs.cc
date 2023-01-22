extern "C" {
double cxxbridge1$rust_multiply(double a, double b) noexcept;
} // extern "C"

double rust_multiply(double a, double b) noexcept {
  return cxxbridge1$rust_multiply(a, b);
}
