#include <iostream>
#include <vector>
#include <algorithm>
#include <functional>

using namespace std;

class Solution {
public:
    vector<string> generateParenthesis(int n) {
        vector<string> res;
        // c++ 太酷了
        function<void(string, int, int)> dfs;
        dfs = [&dfs, &res, &n](const string &str, int l, int r) {
            if (r > l or r > n or l > n)
                return;
            if (r == l and r == n) {
                res.push_back(str);
                return;
            }
            dfs(str + "(", l + 1, r);
            dfs(str + ")", l, r + 1);
        };
        dfs("", 0, 0);
        return res;
    }
};

int main() {
    cout << "ok" << endl;
    return 0;
}