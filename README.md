# otsoc
https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code

## 200704

- 完成rustlings

    之前本着“实用编程语言都大同小异”的想法，看完rust的[语法概览](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)后断断续续做过几个rustlings。正好这次也要求做它，索性就在今天做完了。

    <details>
    <summary>初见rust的一些感觉（单击以展开）</summary>

    - if let有点像java的try-with-resources，或者说，python的with

    - enum可以只是字面量，也可以像Haskell的ADT一样。rust的错误处理把enum写成返回值，并不把错误当成别的什么东西，感觉挺有趣，但写起来不方便（一定要处理错误，不处理要像java一样一直向上传，当然这样的确安全）

    - 这个match貌似只是代替了频繁的if-else + isinstance 根据某变量是哪种类型来做出不同动作。match x块的判断条件不能用x，有点鸡肋。

    - 个人感觉rust隐式return、?的写法让函数结束位置看起来不够清晰

    - 所有权、借用像是下一代引用，低级语言还是得靠人和内存搏斗，编译不通过主要是因为它

    - Arc是引用计数，这缩写太晦涩了乍一看以为和数学有关

    - iter类似java的stream，同样的变量collect会根据类型不同collect出不一样的结果（比如result包着vec，或者vec包着result）

    - 不知道为啥泛型时impl后面也要接着\<T\>，理论上说只有结构体有\<T\>也可以吧

    - 把方法放在单独的trait里有个问题：名称对不上，想用parse得实现FromStr，这很反直觉。在class-based语言里只要在类中实现parse就好了——kotlin甚至可以让你在类定义之外实现某种方法，这就很方便。
    </details>

    exercises我会commit两次，一次初始一次完成，这样方便看改了哪里。

    之后会着重看所有权、迭代器（主要看看有哪些方法可用）、多线程、错误处理的部分。

## 200705

- 大致读完了《Rust编程之道》二三四五九章

    > 不要过早迷失于细节。略读即可 —— 本书作者

    <details>
    <summary>学到了不少新东西（甚至感觉对C++更理解了）</summary>

    - Rust名字和标志来源于锈菌，惊了

    - 所有权转移就是当位置表达式出现在值上下文中时，内存地址被转移给另外一个位置表达式

    - 引用也被称为借用，&x是x的借用

    - move || xxx 其实是将捕获到的变量所有权转移给匿名函数，以防止某变量在外面被销毁出现悬垂指针。我昨天以为中间的||是或，疑惑了好久这是什么诡异判断……昨天说“着重看多线程”其实就是不明白这啥意思，现在懂了

    - match用法并不少，不过貌似还是没有把match的结果用匿名函数得到布尔值的方法

    - Box\<T\>其实是将T放到堆上的意思，使用时可以自动解引用（实现Deref），像使用T一样（我喜欢这种透明感）

    - &str的目的是在编译器得到确定大小的字符串，为什么可以得到确定大小？因为引用来引用去第一个被引用的总会是固定的字面量，大小可以由它推出来

    - 用let v:() = xxx的报错信息查看xxx的类型——好，简单朴实，但我选择clion转插件

    - 孤儿规则是为了防止某些类型的行为被破坏性改写（像是方法的所有权）

    - 空impl块代表默认实现，冒号表示继承其他trait，trait可以用韦恩图画出

    - 存放在堆上的数据要通过其放在栈上的指针进行访问——长久以来我一直没意识到这一点，思维里直接把栈上的指针忽略了……

    - 智能指针是实现了Drop和Defef的结构体

    - RAII也有一个别名——作用域界定的资源管理——我一直以为RAII真的只是字面意思获取资源时初始化，忽略了析构函数也是RAII的一部分，所以一直不知道一直强调RAII到底有啥意思

    - 内存泄漏不在内存安全的概念，rust也会内存泄漏

    - enum就是带了标签指明成员的union

    - copy👉值语义、move👉引用语义。

    - 标注生命周期不改变引用的生命周期长短，只用于编译器的检查

    - box只在Rust源码内使用——这个才是“魔法”

    - 可以对Option\<T\>使用map，直接对T进行计算，类似的还有map_or等，这些就是“组合子”——哦，原来组合子就是这些啊，那也不遥远么

    - and_then的返回值不像map那样包装了一层Some
    
    - 可以使用type定义类型别名，讲话函数签名（想起了python的type hint）
    </details>

- 做了两道leetcode

    我也不明白意义不明的“15道笨方法学XXX练习题”是个啥意思，既然说可以用算法题代替，就选了leetcode了。放在fifteen-exercises里。

    和昨天做人家挖好空的rustlings不同，自己写更加感受到了什么叫和编译器格斗。以及对着Option使用map真好。rust的map不止可以处理迭代器，有点通用，都是举起包装直接对里面的货物进行操作。我记得Haskell有个fmap，感觉可能是类似的。

## 200706

今天有事

- 看了一部分《Rust编程之道》unsafe章节
- 做了一道leetcode一道codewars

    主要是leetcode很多题目不适合rust，看看codewars

## 200707

今天有事

- 大致看完《Rust编程之道》第十三章
    
    > 学习 Unsafe Rust 才能对 Safe Rust 有更深的理解 —— 本章小结

    <details>
    <summary>unsafe这章内容可太多了……</summary>

    - unsafe在解引用裸指针、调用unsafe函数、访问修改该可变静态变量、实现unsafe trait、读写union中的字段时不会进行安全检查

    - 用#\[repr(C)\]告诉编译器使用C语言内存布局

    - 解引用优先级低于方法，高于as

    - as_ptr指向的是存放数据堆/栈的指针，引用是对本身的引用

    - 型变（variance）

        Cat是Animal子类型

        - 协变（covariant）：List\<Cat\>是List\<Animal\>子类型

            Rust大部分结构是协变的（感觉其他语言也是）

            Rust的协变以忘记原始生命周期为代价

        - 逆变（contravariant）：List\<Animal\>是List\<Cat\>子类型

        - 协变（invariant）：List\<Cat\>和List\<Animal\>没关系

        - 长生命周期是短生命周期的子类型

        这玩意前所未闻见所未见……

        - 当协变不会引起ub时，可以用协变，否则保证该类型为不变或逆变
    
    - 有时要改变变量声明顺序，方便编译器推导drop顺序

    - ABI包括调用约定、内存布局、处理器指令集、二进制格式——老听说C ABI，在学rust时知道是啥了

    - C中可以用结构体模拟元组，传递更复杂类型可以用Opaque和Box\<T\>对应

    - 还写了和各种其他语言互动，甚至讲了wasm基本概念，这书快赶上百科全书了
    </details>

- 写了一道codewars，两道leetcode

## 200708

今天有事

- 看了一部分《Rust编程之道》模块化章节
- 写了两道leetcode

    - leetcode竟然有生命游戏，我以为都是奇怪的算法题

    - rust的字符串很麻烦，不能`string[index]`获取字符，要`string[index..].chars().next().unwrap()`。我的天啊，这就是安全么？还是说我用错了。

## 200709

今天有事

- 把《Rust编程之道》模块化章节10.3节之外的内容看完了

    貌似“完成《Rust 编程之道》第十章的完整示例代码”指的是10.3，明天看

- 写了两道leetcode

    rust也可以在函数里定义函数，很好，我喜欢

## 200710

- 完成了《Rust编程之道》第十章的示例代码
- 看完了《计算机组成与设计 RISC-V版》第一章
- 补充了15道练习题其他语言的实现

    写了几天rust去写c++感觉c++可太爽了

- 写了一道leetcode

## 200711

- 把《计算机组成与设计 RISC-V版》第二章看了一半

    有很多学过的内容，但就在这些内容里也能获得新的认知。为什么我们不用这本当教材？

- 写了三道leetcode，至此15道练习题写完了

## 200712

- 看完了《计算机组成与设计 RISC-V版》第二章
- 开会

    > 操作系统有好多哲学原理 —— 陈向群教授

    我惊了，有个大佬只有大二，“好几年前在公司做游戏学会了rust”。那不就是高中开始实习了么，然后人家高考也没落下，上了南方著名985。

- 写（看）完了lab0

    - 接下来要看看内联汇编，后面应该还会经常用到
    - OpenSBI是个好东西，不仅实现了bootloader，还提供一些实用过程，棒！

## 200713

今天有事

- 看了llvm_asm!
    
    昨天说要看是因为我的lsp客户端提示应该用asm!，写完日志发现俺的客户端有问题，的确应该用llvm_asm!，那没事了……

- 写（复制）完了lab1

    - 我不知道这是咋回事啊，rcore是直接给代码的么？我以为也是会挖空的，但这个复制下来解决解决编译错误就能跑。明天不看新的了，画画图（趁刚开始）、整理整理代码
    - rcore fork了rust-embedded/riscv，但我看貌似也没有巨大变化，所以直接用了原版，之后有问题再换回来
    - 挺喜欢rcore tutorial直接用OpenSBI、riscv crate的，这样源码里不会出现大量工具代码（不过可能之后的lab会有？）

## 200714

今天有事，很累

- 画了简单的流程图，并且标注了一些细节

    - 可以在rcore目录下的README中看到（如果网没问题的话）
    - 感觉还有好多地方没有说，但不知道该把文字放在图的哪个位置
    - 另外plantuml貌似画不出想要的样子，它的活动图太结构化了，而我想要箭头随便指
    - 画图有助于关注细节，因为得想这部分代码有什么作用
    - 得知rcore也是有题目的，就在“实验题”部分，只有两个的原因是还没写完。明天写一个看看，顺带完善一下图

## 200715

今天有事，回来很晚，摸了

## 200716

> 休息，休息一下 —— 一休

- 尝试了下用gdb调试rcore

    感觉无论是便携性、调试代码的方便程度，的确gdb-dashboard比pwnedbg好些
    
    用tmux一边看qemu一边看gdb的确是个好想法，之前没怎么用过tmux，从来都是用konsole的分页（不过貌似tmux最主要用途也不是分屏？）
    
    为什么我的显示器不能竖过来？gdb-dashboard可太长了

## 200717

- 写了实验一

    <details>
    <summary>点击展开</summary>

    1. 简述

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

    2. 回答

        ```shell
        src/interrupt/handler.rs:66: 'Unresolved interrupt: Exception(LoadFault)
        Context { registers: [3, 80200044, 80216580, 0, 8001de00, 1, 82200000, 82200000, 8001dd70, 8000000000006800, 0, 3f, 3f, 0, 0, 802163a4, 4, 1, 1, 8000000000006800, 80200000, 82200000, 0, 0, 2000, 0, 0, 0, 80200000, 0, 0, 0], sstatus: Sstatus { bits: 8000000000006120 }, sepc: 80201a42 }
        stval: 0'
        ```

        rust_main返回后回到entry.asm，entry.asm在调用rust_main之后没有写代码。这里的LoadFualt应该是二进制文件里下一条（总之要有一条）请求了不存在的地址0 → 见stval 0
       
        那我觉得entry.asm在rust_main之后可以ecall终止，做个双保险
    
    3. 实验
    
        rcore-tutorial通过切换分支改实验，而我这本来就是一个仓库了，不知道怎么把代码放上来比较好……方法和答案一样

        不过触发那里，我去掉rust_main最后的painc就可以直接请求地址0了，见上题。去掉后疯狂输出SUCCESS！

    </details>

- 感觉之前可能会错意了，应该是让我们跟着指导写自己的操作系统？

    重写了指导0、1的内容，不过因为看过了再写，而且这两个指导都是基础设施（Makefile、boot的汇编、切换上下文的汇编、println、SBI相关……），也不好写成别的样子，所以内容没啥大变动。

- 快速浏览了《RISC-V手册：一本开源指令集的指南》第十章

    页数不多内容还挺多，以为只是异常、中断的内容，竟然还有分页。以后还得看。

## 200718

- 看了指导2、实验2

    - 之前我奇怪rcore-tutorial为啥复制代码就能跑，原来只是指导0、1这样啊。这个指导2代码不全，checkout到lab2还多了好多其他代码。
    - 哇，什么叫线段树，原谅我没打过ACM，去查了查。这东西貌似还挺多hack，那我具体应该做成什么样？感觉这个实验题目有些没说清楚，也可能是我不懂，之后再说吧。

- 看了看其他人的笔记

    为啥之前没想到呢……

    有些已经停更了，有些大佬已经飞速进展快做完了，还有些大大佬已经完全在表演艺术了。

    我看有一些人是先整体看完所有指导再回头写代码、报告的，是个好方法，明天要么完善指导2代码，要么开始浏览后面的。

## 200719

- 做指导2前两个

    <details>
    <summary>点击展开</summary>

    - 动态内存分配

        既然说要用写好的crate，首先要在依赖中加入`buddy_system_allocator = "0.4.0"`

        - error[E0433]: failed to resolve: use of undeclared type or module `alloc`

            在main.rs里加`extern crate alloc;`，rust 2015曾经要求全部外部模块都要用它，2018自动为dependencies里的模块插入，但是在no_std下要用到rust core中的模块时，还是要手动加入这句extern crate

        - error[E0412]: cannot find type `LockedHeap` in this scope

            这应该就是指导里说的调用细节之一，加个use：`use buddy_system_allocator::LockedHeap;`

        - error[E0425]: cannot find value `KERNEL_HEAP_SIZE` in this scope

            加个`use crate::memory::config::KERNEL_HEAP_SIZE;`，感觉吧，rust有点傻

        - error[E0658]: the `#[alloc_error_handler]` attribute is an experimental feature

            这个超棒，rust直接把解决方法说了：main.rs加个`#![feature(alloc_error_handler)]`

        到这里，Rust的“malloc”和“free”就全部支持了（当然还有找不到对应的alloc_error_handler）

        复制指导里的Box、Vec测试，可以看到“heap test passed”的输出

    </details>

    前两个步骤我直接把自定义的类型用`type`定义为usize的别名，这样用起来方便，但是担心在之后的指导里这会成为累赘，所以打算先往后看，再考虑改不改回来。

- 开会

    - 我惊了国科大操作系统直接上板子
    - 我惊了有人中断准备考研做这件事
## 200720

今天有事

粗略看了指导3

## 200721

今天有事

粗略看了指导4

## 200722

今天有事

粗略看了指导5

## 200723

粗略看了指导6

感觉真的可以把指导2里那两个usize包装结构体换成usize，不过还是算了吧，有点回到C的感觉

## 200724

- 补完指导2的剩余代码

<details>
<summary>报告底稿</summary>

接着7.19的说，也就是，这一部分只说《物理内存管理》

- 首先是之前决定还是把物理地址、页号从usize的别名换成单独的类，所以先要在memory/address中去掉data，换成struct（当然还有各种#\[\]）
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
- 然后应当没什么问题了，这个实验主要思想是

    1. 留一块内核专用的空间，用来分配堆内存，分配好之后内核就可以执行需要动态分配内存的代码了。
    2. 直接把可用的物理内存按PAGE_SIZE分成物理页
    3. 使用某种方法（AllocatorImpl可切换）以页为单位（FrameAllocator）去申请、释放内存（也就是页框）
    4. 交给其他代码使用页分配器时，又加了一层Mutex防止冲突
</details>
