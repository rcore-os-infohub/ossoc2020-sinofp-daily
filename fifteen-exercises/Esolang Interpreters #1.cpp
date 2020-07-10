#include <iostream>
#include <vector>
#include <algorithm>
#include <functional>

std::string my_first_interpreter(const std::string &code) {
    int memory_cell = 0;
    std::string output;
    for (char c: code) {
        switch (c) {
            case '+':
                memory_cell = (memory_cell + 1) % 256;
                break;
            case '.':
                output.push_back(static_cast<char>(memory_cell));
                break;
        }
    }
    return output;
}


int main() {
    std::cout << my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.") << std::endl;
    std::cout << my_first_interpreter("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.+.") << std::endl;
    return 0;
}
