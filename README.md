
# Bindings

* Python
* Rust
* C++

# Components

* Pattern
  ```
  #     Padding 4
  @     Padding 1
  
  /// Ranges read up till last frame
  1-n   Range
  1,4,6 Non-contiguous frames
  1-5x2 Skip frame 1,3
  1-5y3 Fill-in frame 1,2,4
  ```
  
* FrameSet: Has compiled frames from `pattern`  
* Parser: Converts a string `pattern` into frameset
* Streak: Owns path, frameset

# Layout

```

struct Pattern {
    static void parse(const std::string& pattern);
    bool is_skip();
    bool is_fill();
    bool is_contiguous();
}

template<typename T>
struct FrameSet {
    FrameSet(const Pattern& pattern);
    T frames();
    Pattern m_pattern;
}

struct Streak {
    std::string m_name;
    std::string m_extension;
    FrameSet m_frames;
}

```

# streaker
