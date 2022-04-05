# rust 学习 --- 进度4.2
缺点：难学？
入门视频（白嫖）：https://www.bilibili.com/video/BV1hp4y1k7SV?spm_id_from=333.337.search-card.all.click
基础入门课程
### 安装
[网址](https://www.rust-lany.org/)：https://www.rust-lany.org/

linux or mac
- curl https://sh.rustup.rs-sSf | sh

window sbsystemm for Linux:
- curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

更新rust 
- rustup update
卸载rust
- rustup self uninstall

版本号
- rustc --version
  
查看本地rust文档
- rustup doc

开发工具vscode
安装rust插件

运行前需要编译，编译生成二进制文件，
双击执行
rust是ahead-time 编译语言
编译：
rustc main.rs
rustc只适合编译简单的rust程序

cargo
 - 构建系统和包管理工具
- cargo --version

cargo 创建项目
cargo new xxx
cargo 构建系统
cargo build 
cargo 运行cargo项目
cargo  run（根目录） = 编译 + 执行 
cargo check 
- 监测代码，确保通过率
- 速度比 cargo build 快

cargo build --release;

rust 里面，变量和引用默认是不可变的，（可变条件，前面+mut `${age}`)
println! 宏里面输出变量，一个大括号对应一个变量
pritln!("变量 {}{}",a,b)
(& mut a)地址引用

随机数库： rand

loop 循环关键词

常见问题，跟新Cargo.toml后，执行cargo buid/run 没有下载更新依赖包，<https://blog.csdn.net/duapple/article/details/105512402>

3.1 变量和可变性
let mut
常量不可变const 声明所有字母大小
- 隐藏 后续替换前面同名变量，类型合一和以前的不一样

3.2 数据类型
- 标量类型
  - 整数  75u8
    - 有符号 u8
    - 没有符号 i8
    - isize/usize
    - 整数益处
  - 浮点 IEEE - 754
    - f32 单精度
    - f64 双精度
  - boolean 占用一个字节大小
  - 字符类型 char
  

for .iter 正向，.rev反向