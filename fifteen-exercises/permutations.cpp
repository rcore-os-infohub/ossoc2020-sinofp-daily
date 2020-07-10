#include <iostream>
#include <vector>
#include <algorithm>
#include <functional>

using namespace std;

class Solution {
public:
    vector<vector<int>> permute(vector<int> &nums) {
        sort(nums.begin(), nums.end());
        vector<vector<int>> perm;
        do {
            perm.emplace_back(nums);
        } while (next_permutation(nums.begin(), nums.end()));
        return perm;
    }
};

int main() {
    std::cout << "ok" << std::endl;
    return 0;
}
