# demo-rust-ffi

演示 rust 绑定外部函数接口

`c_main.rs`: 演示调用 c 的函数

`git_main.rs`: 演示调用 [libgit2](https://github.com/libgit2/libgit2) 的库函数

```shell
# 克隆 libgit2
git clone git@github.com:libgit2/libgit2.git

# 编译 libgit2
mkdir build && cd build
cmake ..
cmake --build .

# 配置环境变量

# macos 
export DYLD_LIBRARY_PATH=./libgit2/build/:$DYLD_LIBRARY_PATH

# linux
export LD_LIBRARY_PATH=./libgit2/build/:$LD_LIBRARY_PATH

# 执行
cargo run
```