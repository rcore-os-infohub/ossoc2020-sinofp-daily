#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int hIndex(vector<int> &citations) {
        sort(citations.begin(), citations.end());
        int h = 0;
        int cnt = 0;
        for (auto i = citations.rbegin(); i != citations.rend(); ++i) {
            ++cnt;
            if (*i >= cnt) {
                h = cnt;
            }
        }
        return h;
    }
};

int main() {
    cout << "ok" << endl;
    return 0;
}
