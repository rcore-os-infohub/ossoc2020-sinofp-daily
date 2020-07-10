#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int hIndex(vector<int> &citations) {
        if (citations.empty()) return 0;

        std::size_t l = 0;
        std::size_t r = citations.size() - 1;
        int h = 0;
        while (l <= r) {
            std::size_t mid = (r + l) / 2;
            int cnt = citations.size() - mid;
            if (citations[mid] >= cnt) {
                h = cnt;
                if (!mid) {
                    break;
                }
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        return h;
    }
};

int main() {
    cout << "ok" << endl;
    return 0;
}
