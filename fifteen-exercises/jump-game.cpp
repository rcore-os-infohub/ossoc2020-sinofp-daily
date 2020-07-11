#include <iostream>
#include <vector>
#include <algorithm>
#include <functional>
#include <iterator>

using namespace std;

class Solution {
public:
    bool canJump(vector<int> &nums) {
        int dis = 1;
        for (int i = nums.size() - 2; i >= 0; --i) {
            if (nums[i] >= dis) {
                dis = 1;
            } else {
                ++dis;
            }
            if (i == 0 and dis > 1) {
                return false;
            }
        }
        return true;
    }
};

int main() {
    vector<int> test{3, 2, 1, 0, 4};
    Solution sln = Solution();
    cout << sln.canJump(test) << endl;
    return 0;
}
