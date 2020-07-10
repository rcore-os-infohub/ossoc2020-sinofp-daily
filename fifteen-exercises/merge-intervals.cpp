#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>> &intervals) {
        sort(intervals.begin(), intervals.end());
        vector<vector<int>> new_intervals;
        for (int i = 0; i < intervals.size();) {
            int l = intervals[i][0];
            int r = intervals[i][1];
            int j = i + 1;
            for (; j < intervals.size(); ++j) {
                if (intervals[j][0] <= r) {
                    r = max(intervals[j][1], r);
                } else {
                    break;
                }
            }
            new_intervals.emplace_back(vector<int>{l, r});
            i = j;
        }
        return new_intervals;
    }
};

int main() {
    cout << "ok" << endl;
    return 0;
}
