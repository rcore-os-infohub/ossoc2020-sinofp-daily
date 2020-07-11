# 15道练习题

> 要求用rust语言重新实现15道以上的编程练习题……可以选择类似Leecode的题目，用Rust实现，但需要在README中给出题目的出处和相关描述信息，并给出采用Rust语言和非Rust语言的实现代码。

出处、描述请点折叠块（否则太长了）。

1. add two numbers

    <details>
    <summary>题目描述</summary>
    给出两个 非空 的链表用来表示两个非负的整数。其中，它们各自的位数是按照 逆序 的方式存储的，并且它们的每个节点只能存储 一位 数字。
    
    如果，我们将这两个数相加起来，则会返回一个新的链表来表示它们的和。
    
    您可以假设除了数字 0 之外，这两个数都不会以 0 开头。
    
    示例：
    
    输入：(2 -> 4 -> 3) + (5 -> 6 -> 4)
    输出：7 -> 0 -> 8
    原因：342 + 465 = 807
    
    来源：力扣（LeetCode）
    链接：https://leetcode-cn.com/problems/add-two-numbers
    著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
    </details>
    
2. Esolang Interpreters #1

    <details>
    <summary>题目描述</summary>
    For the rest of this Kata, I would recommend considering "fuck" to be non-profane.
    Esolang Interpreters #1 - Introduction to Esolangs and My First Interpreter (MiniStringFuck)
    About this Kata Series
    
    "Esolang Interpreters" is a Kata Series that originally began as three separate, independent esolang interpreter Kata authored by @donaldsebleung which all shared a similar format and were all somewhat inter-related. Under the influence of a fellow Codewarrior, these three high-level inter-related Kata gradually evolved into what is known today as the "Esolang Interpreters" series.
    
    This series is a high-level Kata Series designed to challenge the minds of bright and daring programmers by implementing interpreters for various esoteric programming languages/Esolangs, mainly Brainfuck derivatives but not limited to them, given a certain specification for a certain Esolang. Perhaps the only exception to this rule is the very first Kata in this Series which is intended as an introduction/taster to the world of esoteric programming languages and writing interpreters for them.
    What is an esoteric programming language?
    
    An esoteric programming language, otherwise known as an Esolang, is an informal computer programming language that is generally not designed for serious practical use. There are a few main aims/themes among the vast majority of such languages:
    
        Achieve Turing-completeness in as few commands (instructions) as possible. There are currently a number of implemented Esolangs that have been proven to be Turing-complete, Brainfuck being the most popular of them all, comprised of no more than 8 distinct commands. Despite having only 8 commands, it has been objectively proven to be Turing-complete. However, Brainfuck is not the Turing-complete programming language with the fewest commands. Boolfuck, a derivative of Brainfuck which operates on bits (0s and 1s) and contains 7 commands only, has also been proven to be Turing-complete through reduction from Brainfuck. Another less-known Esolang called Etre contains as few as 3 commands yet has been proven to be Turing-complete through the translation of a Minsky Machine to Etre.
        To be as hard to program in as possible. The famous Brainfuck Esolang is well known as a Turing tarpit - that is, a Turing-complete programming language where it is very hard to write a useful program in reality. However, Brainfuck is most definitely not the hardest Esolang to program in. For example, its close cousin, Boolfuck, which operates on bits (mentioned above) is even harder to program in. There have also been a small number of implemented Esolangs which are so difficult to program in that no one has ever successfully written a single program in it from scratch - the only programs generated from these languages came from computers!
        As a joke. Many Esolangs out there have been invented solely as a joke. Examples include Ook! and Bitxtreme.
    
    Although there is no clear-cut definition as to when a programming language is esoteric (or not), Esolangs can generally be identified by the following features/traits:
    
        Minimalistic - having as few instructions as possible
        Plays with new concepts - for example, Befunge, another very popular Esolang, is interpreted in two dimensions as opposed to the usual linear way of interpreting code
        Themed - this is a trait of many joke Esolangs. For example, some may be fashioned like Shakespearean plays and others like cooking recipes
        Not clearly documented - Many Esolangs out there have not been described in great detail with perhaps only a few code examples on the entire Internet. Some Esolangs have not even been implemented yet!
        Contain incomplete specs - New Esolangs are being invented every day. Some Esolangs on the Internet are still a work-in-progress and their commands and behaviour have not been finalised yet.
    
    Nevertheless, Esolangs are generally fun to program in, experiment with and write interpreters for. A great deal can be learned about certain concepts and theories in Computer Science just by studying and programming in a well-designed Esolang such as Brainfuck or Befunge.
    
    Next off, I will introduce you to a simple, minimalistic Esolang called MiniStringFuck.
    The Language
    
    MiniStringFuck is a derivative of the famous Brainfuck which contains a memory cell as its only form of data storage as opposed to a memory tape of 30,000 cells in Brainfuck. The memory cell in MiniStringFuck initially starts at 0. MiniStringFuck contains only 2 commands as opposed to 8:
    
        + - Increment the memory cell. If it reaches 256, wrap to 0.
        . - Output the value of the memory cell as a character with code point equal to the value
    
    For example, here is a MiniStringFuck program that outputs the string "Hello, World!":
    
    ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++.+++++++..+++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++.+++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.
    
    And here is another program that prints the uppercase English alphabet:
    
    +++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.
    
    Any characters in a MiniStringFuck program other than + or . are simply non-command characters (no-ops, i.e. do nothing) and therefore can serve as comments in the program.
    The Task
    
    Time to write your first Esolang interpreter :D
    
    Your task is to implement a MiniStringFuck interpreter myFirstInterpreter()/my_first_interpreter()/Interpreter()/interpret() (depending on your language) which accepts exactly 1 required argument code/$code which is the MiniStringFuck program to be executed. The output of the program should then be returned by your interpreter as a string.
    
    Since this is the first time you are about to write an interpreter for an Esolang, here are a few quick tips:
    
        If you are afraid that your interpreter will be confused by non-command characters appearing in the MiniStringFuck program, you can try to remove all non-command characters from the code input before letting your interpreter interpret it
        The memory cell in MiniStringFuck only ever contains a single integer value - think of how it can be modelled in your interpreter
        If you are stuck as to how to interpret the string as a program, try thinking of strings as an array of characters. Try looping through the "program" like you would an array
        Writing an interpreter for an Esolang can sometimes be quite confusing! It never hurts to add a few comments in your interpreter as you implement it to remind yourself of what is happening within the interpreter at every stage
    
    NOTE: If you would not like to name your interpreter as myFirstInterpreter()/my_first_interpreter(), you can optionally rename it to either miniStringFuck()/mini_string_fuck() or interpreter() instead - the test cases will handle the rest for you. Not available in Java, Go, Swift, TypeScript, Haskell, Elixir, C++, C#, Rust, R, Erlang, F# and NASM; irrelevant to Brainfuck solvers ;)
    
    Good luck :D
    
    出自：https://www.codewars.com/kata/586dd26a69b6fd46dd0000c0
    </details>
    
3. game of life

    <details>
    <summary>题目描述</summary>
    根据 百度百科 ，生命游戏，简称为生命，是英国数学家约翰·何顿·康威在 1970 年发明的细胞自动机。

    给定一个包含 m × n 个格子的面板，每一个格子都可以看成是一个细胞。每个细胞都具有一个初始状态：1 即为活细胞（live），或 0 即为死细胞（dead）。每个细胞与其八个相邻位置（水平，垂直，对角线）的细胞都遵循以下四条生存定律：

        如果活细胞周围八个位置的活细胞数少于两个，则该位置活细胞死亡；
        如果活细胞周围八个位置有两个或三个活细胞，则该位置活细胞仍然存活；
        如果活细胞周围八个位置有超过三个活细胞，则该位置活细胞死亡；
        如果死细胞周围正好有三个活细胞，则该位置死细胞复活；

    根据当前状态，写一个函数来计算面板上所有细胞的下一个（一次更新后的）状态。下一个状态是通过将上述规则同时应用于当前状态下的每个细胞所形成的，其中细胞的出生和死亡是同时发生的。

     

    示例：

    输入： 
    [
      [0,1,0],
      [0,0,1],
      [1,1,1],
      [0,0,0]
    ]
    输出：
    [
      [0,0,0],
      [1,0,1],
      [0,1,1],
      [0,1,0]
    ]

     

    进阶：

        你可以使用原地算法解决本题吗？请注意，面板上所有格子需要同时被更新：你不能先更新某些格子，然后使用它们的更新后的值再更新其他格子。
        本题中，我们使用二维数组来表示面板。原则上，面板是无限的，但当活细胞侵占了面板边界时会造成问题。你将如何解决这些问题？

    来源：力扣（LeetCode）
    链接：https://leetcode-cn.com/problems/game-of-life
    著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
    </details>

4. generate parentheses

    <details>
    <summary>题目描述</summary>
    数字 n 代表生成括号的对数，请你设计一个函数，用于能够生成所有可能的并且 有效的 括号组合。
    
    


    示例：
    
    输入：n = 3
    输出：[
           "((()))",
           "(()())",
           "(())()",
           "()(())",
           "()()()"
         ]
    
    来源：力扣（LeetCode）
    链接：https://leetcode-cn.com/problems/generate-parentheses
    著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
    </details>

5. h index

    <details>
    <summary>题目描述</summary>
    给定一位研究者论文被引用次数的数组（被引用次数是非负整数）。编写一个方法，计算出研究者的 h 指数。

    h 指数的定义：h 代表“高引用次数”（high citations），一名科研人员的 h 指数是指他（她）的 （N 篇论文中）总共有 h 篇论文分别被引用了至少 h 次。（其余的 N - h 篇论文每篇被引用次数 不超过 h 次。）

    例如：某人的 h 指数是 20，这表示他已发表的论文中，每篇被引用了至少 20 次的论文总共有 20 篇。

     

    示例：

    输入：citations = [3,0,6,1,5]
    输出：3 
    解释：给定数组表示研究者总共有 5 篇论文，每篇论文相应的被引用了 3, 0, 6, 1, 5 次。
         由于研究者有 3 篇论文每篇 至少 被引用了 3 次，其余两篇论文每篇被引用 不多于 3 次，所以她的 h 指数是 3。

     

    提示：如果 h 有多种可能的值，h 指数是其中最大的那个。

    来源：力扣（LeetCode）
    链接：https://leetcode-cn.com/problems/h-index
    著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
    </details>

6. h index II

    <details>
    <summary>题目描述</summary>
    给定一位研究者论文被引用次数的数组（被引用次数是非负整数），数组已经按照升序排列。编写一个方法，计算出研究者的 h 指数。

    h 指数的定义: “h 代表“高引用次数”（high citations），一名科研人员的 h 指数是指他（她）的 （N 篇论文中）总共有 h 篇论文分别被引用了至少 h 次。（其余的 N - h 篇论文每篇被引用次数不多于 h 次。）"

     

    示例:

    输入: citations = [0,1,3,5,6]
    输出: 3 
    解释: 给定数组表示研究者总共有 5 篇论文，每篇论文相应的被引用了 0, 1, 3, 5, 6 次。
         由于研究者有 3 篇论文每篇至少被引用了 3 次，其余两篇论文每篇被引用不多于 3 次，所以她的 h 指数是 3。

     

    说明:

    如果 h 有多有种可能的值 ，h 指数是其中最大的那个。

     

    进阶：

        这是 H指数 的延伸题目，本题中的 citations 数组是保证有序的。
        你可以优化你的算法到对数时间复杂度吗？

    来源：力扣（LeetCode）
    链接：https://leetcode-cn.com/problems/h-index-ii
    著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
    </details>

7. jump game

    <details>
    <summary>题目描述</summary>
    给定一个非负整数数组，你最初位于数组的第一个位置。

    数组中的每个元素代表你在该位置可以跳跃的最大长度。

    判断你是否能够到达最后一个位置。

    示例 1:

    输入: [2,3,1,1,4]
    输出: true
    解释: 我们可以先跳 1 步，从位置 0 到达 位置 1, 然后再从位置 1 跳 3 步到达最后一个位置。

    示例 2:

    输入: [3,2,1,0,4]
    输出: false
    解释: 无论怎样，你总会到达索引为 3 的位置。但该位置的最大跳跃长度是 0 ， 所以你永远不可能到达最后一个位置。

    来源：力扣（LeetCode）
    链接：https://leetcode-cn.com/problems/jump-game
    著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
    </details>

8. merge intervals

    <details>
    <summary>题目描述</summary>
    给出一个区间的集合，请合并所有重叠的区间。

    示例 1:

    输入: [\[1,3],[2,6],[8,10],[15,18]]
    输出: [\[1,6],[8,10],[15,18]]
    解释: 区间 [1,3] 和 [2,6] 重叠, 将它们合并为 [1,6].

    示例 2:

    输入: [\[1,4],[4,5]]
    输出: [\[1,5]]
    解释: 区间 [1,4] 和 [4,5] 可被视为重叠区间。

    来源：力扣（LeetCode）
    链接：https://leetcode-cn.com/problems/merge-intervals
    著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
    </details>

9. permutations

    <details>
    <summary>题目描述</summary>
    给定一个 没有重复 数字的序列，返回其所有可能的全排列。

    示例:

    输入: [1,2,3]
    输出:
    [
      [1,2,3],
      [1,3,2],
      [2,1,3],
      [2,3,1],
      [3,1,2],
      [3,2,1]
    ]

    来源：力扣（LeetCode）
    链接：https://leetcode-cn.com/problems/permutations
    著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
    </details>

10. Persistent Bugger

    <details>
    <summary>题目描述</summary>
    Write a function, persistence, that takes in a positive parameter num and returns its multiplicative persistence, which is the number of times you must multiply the digits in num until you reach a single digit.
    
    For example:
    
     persistence 39 -- returns 3, because 3*9=27, 2*7=14, 1*4=4
                    -- and 4 has only one digit
    
     persistence 999 -- returns 4, because 9*9*9=729, 7*2*9=126,
                     -- 1*2*6=12, and finally 1*2=2
    
     persistence 4 -- returns 0, because 4 is already a one-digit number
    
    出自 https://www.codewars.com/kata/55bf01e5a717a0d57e0000ec
    </details>

11. powx n

     <details>
     <summary>题目描述</summary>
     实现 pow(x, n) ，即计算 x 的 n 次幂函数。
    
     示例 1:
    
     输入: 2.00000, 10
     输出: 1024.00000
    
     示例 2:
    
     输入: 2.10000, 3
     输出: 9.26100
    
     示例 3:
    
     输入: 2.00000, -2
     输出: 0.25000
     解释: 2-2 = 1/22 = 1/4 = 0.25
    
     说明:
    
         -100.0 < x < 100.0
         n 是 32 位有符号整数，其数值范围是 [−231, 231 − 1] 。
    
     来源：力扣（LeetCode）
     链接：https://leetcode-cn.com/problems/powx-n
     著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
     </details>

12. rotate image

    <details>
    <summary>题目描述</summary>
    给定一个 n × n 的二维矩阵表示一个图像。

    将图像顺时针旋转 90 度。

    说明：

    你必须在原地旋转图像，这意味着你需要直接修改输入的二维矩阵。请不要使用另一个矩阵来旋转图像。

    示例 1:

    给定 matrix = 
    [
    [1,2,3],
    [4,5,6],
    [7,8,9]
    ],

    原地旋转输入矩阵，使其变为:
    [
    [7,4,1],
    [8,5,2],
    [9,6,3]
    ]

    示例 2:

    给定 matrix =
    [
    [ 5, 1, 9,11],
    [ 2, 4, 8,10],
    [13, 3, 6, 7],
    [15,14,12,16]
    ], 

    原地旋转输入矩阵，使其变为:
    [
    [15,13, 2, 5],
    [14, 3, 4, 1],
    [12, 6, 8, 9],
    [16, 7,10,11]
    ]

    来源：力扣（LeetCode）
    链接：https://leetcode-cn.com/problems/rotate-image
    著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
    </details>

13. sort colors

     <details>
     <summary>题目描述</summary>
     给定一个包含红色、白色和蓝色，一共 n 个元素的数组，原地对它们进行排序，使得相同颜色的元素相邻，并按照红色、白色、蓝色顺序排列。

     此题中，我们使用整数 0、 1 和 2 分别表示红色、白色和蓝色。

     注意:
     不能使用代码库中的排序函数来解决这道题。

     示例:

     输入: [2,0,2,1,1,0]
     输出: [0,0,1,1,2,2]

     进阶：

         一个直观的解决方案是使用计数排序的两趟扫描算法。
         首先，迭代计算出0、1 和 2 元素的个数，然后按照0、1、2的排序，重写当前数组。
         你能想出一个仅使用常数空间的一趟扫描算法吗？

     来源：力扣（LeetCode）
     链接：https://leetcode-cn.com/problems/sort-colors
     著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
     </details>

14. spiral matrix

     <details>
     <summary>题目描述</summary>
     给定一个包含 m x n 个元素的矩阵（m 行, n 列），请按照顺时针螺旋顺序，返回矩阵中的所有元素。
    
     示例 1:
    
     输入:
     [
      [ 1, 2, 3 ],
      [ 4, 5, 6 ],
      [ 7, 8, 9 ]
     ]
     输出: [1,2,3,6,9,8,7,4,5]
    
     示例 2:
    
     输入:
     [
       [1, 2, 3, 4],
       [5, 6, 7, 8],
       [9,10,11,12]
     ]
     输出: [1,2,3,4,8,12,11,10,9,5,6,7]
    
     来源：力扣（LeetCode）
     链接：https://leetcode-cn.com/problems/spiral-matrix
     著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
     </details>

15. zigzag-conversion

    <details>
    <summary>题目描述</summary>
    将一个给定字符串根据给定的行数，以从上往下、从左到右进行 Z 字形排列。
    
    比如输入字符串为 "LEETCODEISHIRING" 行数为 3 时，排列如下：
    
    <pre>
    L   C   I   R
    E T O E S I I G
    E   D   H   N
    </pre>
    
    之后，你的输出需要从左往右逐行读取，产生出一个新的字符串，比如："LCIRETOESIIGEDHN"。
    
    请你实现这个将字符串进行指定行数变换的函数：
    
    string convert(string s, int numRows);
    
    示例 1:
    
    输入: s = "LEETCODEISHIRING", numRows = 3
    输出: "LCIRETOESIIGEDHN"
    
    示例 2:
    
    输入: s = "LEETCODEISHIRING", numRows = 4
    输出: "LDREOEIIECIHNTSG"
    解释:
    
    <pre>
    L     D     R
    E   O E   I I
    E C   I H   N
    T     S     G
    </pre>
    

    来源：力扣（LeetCode）
    链接：https://leetcode-cn.com/problems/zigzag-conversion
    著作权归领扣网络所有。商业转载请联系官方授权，非商业转载请注明出处。
    </details>