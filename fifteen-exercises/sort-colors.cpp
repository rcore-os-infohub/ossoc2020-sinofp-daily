#include <iostream>
#include <vector>
#include <algorithm>
#include <functional>
#include <iterator>

using namespace std;

class Solution {
public:
    void sortColors(vector<int> &nums) {
        int l = 0, r = nums.size() - 1, i = 0;
        while (i <= r) {
            switch (nums[i]) {
                case 0:
                    swap(nums[i], nums[l]);
                    ++l;
                    ++i;
                    break;
                case 2:
                    swap(nums[i], nums[r]);
                    if (r > 0) {
                        --r;
                    } else {
                        return;
                    }
                    break;
                default:
                    ++i;
            }
        }
    }
};

int main() {
    cout << "ok" << endl;
    return 0;
}
