#ifndef UNIQUE_PTR_H
#define UNIQUE_PTR_H

namespace MvrSTL {
template <typename T> class unique_ptr {
  public:
    explicit unique_ptr(T *p = nullptr) noexcept : ptr(p){};
    unique_ptr(unique_ptr &&other) noexcept : ptr(other.ptr) { other.ptr = nullptr; };
    ~unique_ptr() { reset(); }

  private:
    T *ptr;
};

} // namespace MvrSTL

#endif // UNIQUE_PTR_H
