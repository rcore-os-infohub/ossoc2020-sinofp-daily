#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    string convert(string s, int numRows) {
        if (1 == numRows) return s;
        std::size_t len = s.size();
        std::size_t md = 2 * numRows - 2;
        string s_out;
        for (int i = 0; i < len; i += md) {
            s_out.push_back(s[i]);
        }
        for (int j = 1; j < md / 2 + 1; ++j) {
            for (int i = 0; i < len; i += md) {
                int l = i + j;
                int r = i + md - j;
                if (l < len) {
                    s_out.push_back(s[l]);
                }
                if (r < len and r != l) {
                    s_out.push_back(s[r]);
                }
            }
        }
        return s_out;
    }
};

int main() {
    cout << "ok" << endl;
    return 0;
}
