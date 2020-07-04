# otsoc
https://github.com/rcore-os/rCore/wiki/os-tutorial-summer-of-code

## 200704

- 完成rustlings

    之前本着“实用编程语言都大同小异”的想法，看完rust的[语法概览](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)后断断续续做过几个rustlings。正好这次也要求做它，索性就在今天做完了。

    <details>
    <summary>初见rust的一些感觉</summary>

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
