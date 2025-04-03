# lab 3 报告

## 编程作业部分

第一个实验需要去获取 id 的地址的数据，因为不涉及用户空间的地址转换，直接用裸指针找数据就可以了 [解引用裸指针 - Comprehensive Rust 🦀](https://google.github.io/comprehensive-rust/zh-CN/unsafe-rust/dereferencing.html) -

data 直接用 u8 转换保留最低位

但是这是啥意思呢，怎么又说这是 const 又想写入？

```
- 如果 `trace_request` 为 1，则 `id` 应被视作 `*const u8` ，表示写入 `data` （作为 `u8`，即只考虑最低位的一个字节）到该用户程序 `id` 地址处。返回值应为0。
```

为了记录 syscall 数量，可以给每个进程定一个结构体，然后给 TaskManager 设计一个函数，调用 syscall 的时候增加 count。

这里 syscall 比较乱，没有统一的管理结构，暂且写一个固定长度的数组吧。

## 简答作业部分

运行 bad 测试，rustsbi-0.3.0-alpha.2。运行命令 `make run TEST=2`

在 U 模式下对于 bad address 触发访问异常的中断后，会 match 上 Exception::StoreFault。

```rust
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
            exit_current_and_run_next();
        }
```


在 U 模式不能执行 S 模式的指令，也不能操作 S 模式的寄存器，属于 `Exception::IllegalInstruction`

```
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next();
        }
```

### trap.S
#### L40 ：刚进入 `__restore` 时，`sp` 代表了什么值。请指出 `__restore` 的两种使用情景。

表示内核栈，一个是用来在中断的时候返回用户态，另一个在用户程序开始的时候进行必要的 context 布置。

#### L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

```
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
```
`32*8(sp)` sstatus 表示 S、U 模式

`33*8(sp)` spec (Supervisor exception program counter) 当 Trap 是一个异常的时候，记录 Trap 发生之前执行的最后一条指令的地址

`2*8(sp)` 用户的 sp

#### L50-L56：为何跳过了 `x2` 和 `x4`？

根据注释，需要跳过 sp/tp，因为用户态的 sp 不是存在栈上，而是在寄存器 sscratch 里面。而 tp 寄存器，除非我们手动出于一些特殊用途使用它，否则一般不会被用到。

```
skip sp(x2), we will save it later
skip tp(x4), application does not use it
```

#### L60：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？

```
csrrw sp, sscratch, sp
```

sp 是 user 栈顶，sscratch 是 kernel 栈顶。

#### `__restore`：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

`sret` 根据手册，会进行 privilege level 的切换。

```
When an SRET instruction (see Section 3.2.1) is executed to return from the trap handler, the privilege level is set to user mode if the SPP bit is 0, or supervisor mode if the SPP bit is 1; SPP is then set to 0.
```
#### L13 ：该指令之后，`sp` 和 `sscratch` 中的值分别有什么意义？
```
csrrw sp, sscratch, sp
```    

sp 是 kernel 栈顶，sscratch 是 user 栈顶

#### 从 U 态进入 S 态是哪一条指令发生的？

根据手册，`ecall` 指令会发出一个 service request 请求。

```
The ECALL instruction is used to make a service request to the execution environment. The EEI will define how parameters for the service request are passed, but usually these will be in defined locations in the integer register file
```

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：


[riscv 手册](https://www.scs.stanford.edu/~zyedidia/docs/riscv/)

[解引用裸指针 - Comprehensive Rust 🦀](https://google.github.io/comprehensive-rust/zh-CN/unsafe-rust/dereferencing.html) 

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。