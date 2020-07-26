# 实验报告

## lab0

这个实验主要是搭建一个可以编译到裸机系统的helloword，主要有以下几点：

1. 禁用rust标准库、实现简单的panic_handler、禁用C语言运行时
2. 使用链接脚本、汇编代码让openSBI再开机后可以执行我们的rust代码

核心是这些，其他还有细碎的交叉编译、Makefile、封装openSBI代码实现println!、panic!什么的，这个实验只是引导，跟着做就好了。

另外感叹一点rust的构建真的很方便，不像C那样要写很——长的Makefile，cargo很能干

### lab0总结

这是我当时写完lab0准备用画图画出整个rcore示例时画出的图，不过并没有更新了，放在这里当做lab0的总结倒是挺不错

下图使用plantuml在线服务渲染，个别字体可能会有问题。

![图](http://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/sinofp/otsoc/master/rcore/rcore.puml)


## lab1

指导里把中断、异常、陷阱用放学、老师找家长、学生自己找家长来解释，我是没想到的。

这一部分有使用rust的riscv库，riscv库封装了对中断相关寄存器的访问、写入，还封装了Exception类型

### 处理中断的准备工作

使用`#[repr(C)]`+`struct`可以定义C那样的结构体，或者说，可以顺序放在内存里的结构体。这样我们可以用汇编代码讲寄存器按顺序压入栈中，将第一个寄存器被压入的内存地址当做结构体的地址。

rcore定义了Context结构体用来存储32个通用寄存器、与中断处理相关的sstatus、sepc寄存器

- 这里其实我有个疑惑：为什么寄存器要用usize来存储？rust的数组下标只能用usize类型的变量表示，而理论上数组长度应该是无限大，所以我以为usize是特殊的无符号数，可以用任意长度内存空间表示任意大小的数字（我记得Erlang是有这样的数字类型的），但查了下文档，usize是“The pointer-sized unsigned integer type”，所以……指针有多长？我记得还有所谓比别的指针长的“胖指针”啊？文档接着说：64位机上占8字节——我们用的编译目标是`riscv64imac-unknown-none-elf`，64位目标，8字节正好是64位riscv的寄存器长度，所以我们可以使用usize来存储寄存器的值

- 后面很多实验也用到了usize的封装类型来表示地址

接着说进入中断处理前保存寄存器的事，其实就是需要用到的寄存器统统压栈，执行完再弹出。搁到x86_64架构的CPU，一对PUSHAD、POPAD即可，但riscv的r告诉我们：“不要搞特殊化！”。所以我们还是得一个一个push，一个一个pop

riscv汇编的stack pointer不会自己增长。x86中可以直接push到当前栈顶，但riscv往往是手动给sp减一个值，用store（这里是sd）+相对sp偏移量把值放入内存正确位置。而sp本身也是32个通用寄存器之一，所以在保存sp（sp是通用寄存器x2）之前，我们必须变动sp的值（sp - 34*8，给其他通用寄存器指示位置）。

解决方法也很简单，sp原值被减了34*8，存储原值前再加回来就好了，所以有interrupt.asm中的如下代码：

```asm
addi sp, sp, -34*8
sd x1, 1*8(sp)
addi x1, sp, 34*8 # let x1 = old x2
sd x1, 2*8(sp) # save old x2
```

（代码展开了宏，我并不喜欢这里的宏，使用它反而丧失了riscv的清晰感），在恢复值的部分，只要最后再ld x2就好了。

- 我当时想：既然用sp定位会造成压入、弹出x2的特殊处理，那不用sp（stack pointer）定位，改用fp（frame pointer）不就好了？其实x86上好多相对寻址都是相对ebp的（ebp对应fp），因为esp总是会变。好，我简直是天才——然后想起fp也只是x8的别名，这样做仍然要特殊处理，不能一水相似的sd、ld

interrupt.asm里还有用到读取、保存CSR（Control and Status Registers）寄存器的专用操作csrr、csrw来获取、更改sstatus、sepc、scause、stval的值。csr{r,w}是为了原子读写而设计的指令，不过这个实验里只把它们当成load、store来用。这些不是重点，重点是`jal handle_interrupt`这句，它让我de了好久bug

`jal handle_interrupt`是压栈完寄存器，传入handle_interrupt的参数后跳转到handle_interrupt函数的汇编代码，jal意为“jump and link”。我在写这段时就想着要跳转到这个函数，所以直接写了个`j handle_interrupt`。在实验1结束，本应该会处理断点中断、时钟中断，但我的代码只能打印断点中断的信息，时钟中断总是出不来，会报错（未解决的Exception，handle_interrupt的那个match块的默认语句）。找了半天bug在哪，gdb都快被我的“si”听烦了，最后才想起是不是这个“无条件跳转”出了问题——是的！`jal`会添加返回地址，这样执行完handle_interrupt还会再回来执行interrupt.asm里剩下的汇编代码，但是`j`不是，jump完就完了，再也不见了，处理器去执行handle_interrupt之后的汇编代码，执行流程就超出了我的掌控范围，也因此总是打印不了时钟。

好，执行中断处理程序前的汇编程序（__interrupt）说完了，但是我们怎么告诉处理器，出了问题（收到中断信号）要到这里来执行？

实验指导里有提到，“stvec”是用来存放中断处理程序入口（__interrupt）地址的寄存器。它分为两种模式：

1. 遇到中断就到指定地址的direct模式
2. 遇到中断要提供中断编号，跳到指定地址+4×编号地址的vectored模式

在这个实验中，中断处理的逻辑是进入__interrupt保存上下文，再跳到handle_interrupt，handle_interrupt中使用match块根据scause（记录中断原因的寄存器）内容分发到不同中断类型的处理程序。所以我们在这里要使用direct模式，把__interrupt的地址写入stvec寄存器。

最后的中断处理程序（__interrupt跳转到的函数）就是一个match：

```rust
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    match scause.cause() {
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        _ => fault(context, scause, stval),
    }
}
```

问题来了，__interrupt的地址是什么？我们怎么写入stvec寄存器？

__interrupt的函数体我们用汇编写好了，不过其实没声明，像C代码一样，声明一下函数签名，就可以得到函数地址；写入stvec寄存器，可以用前面提到的riscv crate的封装，所以有了如下代码：

```rust
unsafe {
    extern "C" {
        fn __interrupt();
    }
    //                          ↓ 前面说过usize就是寄存器大小
    stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
}
```

### 断点中断

断点中断直接使用汇编`ebreak`即可触发，这里的处理是直接修改sepc（触发中断的指令的地址），跳过ebreak这句汇编执行下一个。我理解这个断点中断只是用来实验中断处理有没有起作用的，所以直接跳过了。

### 时钟中断

好，openSBI大哥又出现了！它替我们实现了set_timer的C函数，我们用rust封装一下（这个之前没有说，是使用汇编ecall+extension_id来执行openSBI的C函数的）。set_timer在规定周期后会触发`Interrupt::SupervisorTimer`中断，每次中断我们都给TICK加1，每到100就打印一次100TICKS。

至此，我们就支持了两种中断：断点和时钟（主要是时钟）。“幼儿园级kernel”就告成了。

## 实验1

1. 原理：在 rust_main 函数中，执行 ebreak 命令后至函数结束前，sp 寄存器的值是怎样变化的？

既然有gdb，为什么不用它呢？

因为有部分是汇编，所以不能n、n、n、n，得si、si、si、si……

```中文
ebreak前：80216650
    进入__interrupt -> 80216540
        跳转到handle_interrupt，上来addi sp, sp, -128 -> 802164c0
            进入riscv::register::scause::Scause::cause -> 80216470
                进入riscv::register::scause::Scause::code -> 80216430
            退出code，回到cause -> 80216470
        回到handle_interrupt -> 802164c0
            进入breakpoint -> 80216440
                println!宏，很多变化
        回到handle_interrupt -> 802164c0
    回到interrupt.asm，不过不是__interrupt，是__restore（这也是__interrupt最后调用的，不过因为手动用汇编写的，其实sp相比__inerrupt没变），哗啦哗啦pop完（其实是ld）寄存器后变成80216650
回到rust_main，sp不变，结束
```

2. 分析：如果去掉 rust_main 后的 panic 会发生什么，为什么？

```shell
src/interrupt/handler.rs:66: 'Unresolved interrupt: Exception(LoadFault)
Context { registers: [3, 80200044, 80216580, 0, 8001de00, 1, 82200000, 82200000, 8001dd70, 8000000000006800, 0, 3f, 3f, 0, 0, 802163a4, 4, 1, 1, 8000000000006800, 80200000, 82200000, 0, 0, 2000, 0, 0, 0, 80200000, 0, 0, 0], sstatus: Sstatus { bits: 8000000000006120 }, sepc: 80201a42 }
stval: 0'
```

rust_main返回后回到entry.asm，entry.asm在调用rust_main之后没有写代码。这里的LoadFualt应该是二进制文件里下一条（总之要有一条）请求了不存在的地址0 → 见stval 0
       
那我觉得entry.asm在rust_main之后可以ecall终止，做个双保险

3. 实验

    1. 如果程序访问不存在的地址，会得到 Exception::LoadFault。模仿捕获 ebreak 和时钟中断的方法，捕获 LoadFault（之后 panic 即可）。

    ```rust
    #[no_mangle]
    pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
        match scause.cause() {
            Trap::Exception(Exception::Breakpoint) => breakpoint(context),
            Trap::Interrupt(Interrupt::SupervisorTimer) => timer::tick(),
            Trap::Exception(Exception::LoadFault) => panic!("Load Fault @ 0x{:x}", stval),
            _ => panic!(
                "Unresolved interrupt: {:?}\n\t{:x?}\n\t{:x}",
                scause.cause(),
                context,
                stval
            ),
        }
    }
    ```

    2. 在处理异常的过程中，如果程序想要非法访问的地址是 0x0，则打印 SUCCESS!

    在`Trap::Exception(Exception::LoadFault) => panic!("Load Fault @ 0x{:x}`那句加个if-else，用stval判断等不等于0

    2. 添加或修改少量代码，使得运行时触发这个异常，并且打印出 SUCCESS!

    去掉rust_main最后的painc就可以直接请求地址0了，见2.分析。去掉后疯狂输出SUCCESS！

### lab1总结

这个实验相对简单，只是调用openSBI、riscv库的代码。不过这样真的很舒服，我觉得，会不会未来同一世界的操作系统就像乐高积木一样，只是提供这些底层的库，跨平台，然后用户自己用它们定制自己的系统？那时，写操作系统可能像是写配置文件一样简单～

## lab2

栈的访问速度比堆快（因为访问堆时要先访问放在栈上的指针），但是os总会遇到必须使用堆的时候（比如放一些持久性的数据，或者根据情况创造、销毁数据结构），这一指导就是告诉我们如何实现“堆”（加引号是因为实现了两个堆，而不仅仅是一般意义上的用户用的堆）。

### 动态内存分配

rust给了我们自定义堆的权利，但是要实现`GlobalAlloc`trait（alloc与dealloc）并用`#[global_allocator]`标记，还要实现alloc失败的处理函数，用`#[alloc_error_handler]`（有点像禁用标准库时强迫我们实现panic_handler一样，这就是rust骨子里的安全性吧）

实验中使用了`Buddy System Allocator`crate，直接就有了`#[global_allocator]`

这部分先预留了一部分“堆”，给操作系统本身使用，大小是`static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];`，这里不能用Vec申请，因为堆还没实现，不能用动态长度的手段申请空间。在init中告诉`Buddy System Allocator`crate的LockedHeap要管理这么多的空间

然后就可以使用Box、Vec了，印象里不用添加太多别的代码

### 物理内存探测

这部分主要是为下一步分配内核之外的内存空间做准备，要分配内核之外的剩余空间，首先要知道内核把内存占到哪了。

链接脚本里指定了kernel_end放在内存最后面，我们可以仿照之前得到__interrupt函数的地址那样得到kernel_end的地址。但实际不行，“因为rust语言的限制”。

我猜测这个“限制”可能是因为kernel_end的位置由链接脚本指定，而不是__interrupt那样随意在代码中声明，所以没法在编译时得到位置。不过没关系，可以用lazy_static在运行时得到kernel_end的地址（说起来，在《rust编程之道》里看到过它）。这里指导把地址做了额外的封装（__interrupt就是直接用usize写入寄存器的）

```rust
pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);
```

如上，使用了PhysicalAddress包装usize（本质就是个元祖，只有一个usize的值）。当时做到这里的时候我觉得很麻烦，好好的usize不用非得自己封装一个类。再看一眼示例代码，哇还专门写了好长一个宏给它实现本来usize就有的方法，更坚定了我觉得它麻烦的决心。但是之后又看了看后面的实验，还要VirutalAddress、VirtualPageNumber、PhysicalPageNumber，都只是usize。如果只用usize表示的话，的确很方便，但就和C一样了，什么类型的数据都不过时内存中某种长度的空间罢了，然后它们之间可以随意混淆编译器也不报警。所以最后我觉得，要不还是按指导的来，我也包装吧（不过我不写很长的宏，我把它分开成很多小的宏，比较清晰）

还有，这部分最后打印了KERNEL_END_ADDRESS，我一开始做直接用usize，打印没问题，但后来改成PhysicalAddress就有得实现输出专用的trait，哦，rust好死板，不如直接.0访问usize。

### 物理内存管理

- 先是把物理地址、页号从usize的别名换成单独的类，所以先要在memory/address中去掉data，换成struct（当然还有各种#\[\]）
- 示例代码里直接写了两个宏去生成包装类（原谅我，我还是喜欢叫它类）的方法，不过我觉得这样太乱，所以决定先编译，报错需要哪个再补上相应代码
- 首先按照“分配与回收”填上FrameTracker的代码。FrameTracker有些类似于页框在操作系统中的描述
- FrameTracker需要用into把页号转成物理地址，所以我们得`impl From<PhysicalPageNumber> for PhysicalAddress`（不过我记得这个报错是很靠后的，具体报错顺序记不清了）
- 按指导，接着是frame/allocator，这个要加的代码就多了。

    - 首先是Mutex，这个需要spin包，所以得在Cargo.toml里加上一句
    - FrameAllocatorx只是包装了AllocatorImpl的一些方法，而AllocatorImpl在示例代码中放到了单独的仓库algorithm下，不过俺觉得这有些分离了，所以直接在memory文件夹里又开了一个allocator文件夹放这些代码。
    - 然后FrameAllocator::alloc需要用到物理页号+usize的操作，返回的也是物理页号，直接self.start_ppn.0+usize，再用PhysicalPageNumer新建一个有些累赘了，所以回到adress里实现`core::ops::Add<usize> for PhysicalPageNumber`（实现方法就是前面说的累赘代码）
    - dealloc还需要页号-usize得到usize的，这里直接访问页号内部的usize即可，无需添加新的代码。
    - 然后说一下FrameAllocator和AllocatorImpl的关系，有点像经理和打工仔：一开始FrameAllocator告诉AllocatorImpl有多少空间可分配，然后真正需要空间时，再由打工仔看看到底可不可行，并且记录当前状态。o
    - 所以经理怎么知道有多少空间可用？config中得到了kernel在内存中结束的地址，并设定了内存结束地址，它俩相减即可得到可用空间，这个范围由memory::Range表示
    - 其实我是觉得和core的Range重名不太好的，但它的确是个Range，再没有更合适的名字了，总之，我们还得去实现Range
    - Range也没啥，就记着俩数，需要from、len就实现好了。不过rcore的Range想做成能装换成usize且能从usize转回来的类型都能用的泛型，所以还得回头往address里补上和usize之间的转换

### lab2总结

这个实验主要思想是

1. 留一块内核专用的空间，用来分配堆内存，分配好之后内核就可以执行需要动态分配内存的代码了。
2. 直接把可用的物理内存按PAGE_SIZE分成物理页
3. 使用某种方法（AllocatorImpl可切换）以页为单位（FrameAllocator）去申请、释放内存（也就是页框）
4. 交给其他代码使用页分配器时，又加了一层Mutex防止冲突
