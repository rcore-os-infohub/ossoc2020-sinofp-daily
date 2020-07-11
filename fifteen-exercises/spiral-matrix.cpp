#include <iostream>
#include <vector>
#include <algorithm>
#include <functional>
#include <iterator>

using namespace std;

class Solution {
public:
    vector<int> spiralOrder(vector<vector<int>> &matrix) {
        if (matrix.empty()) return vector<int>();
        const int x_max = matrix[0].size();
        const int y_max = matrix.size();
        int x = -1, y = 0;
        const int dx[] = {1, 0, -1, 0};
        const int dy[] = {0, 1, 0, -1};
        int dir = 0;
        vector<int> res;
        while (true) {
            while (true) {
                x = x + dx[dir];
                y = y + dy[dir];
                if (x < 0 or x >= x_max or y < 0 or y >= y_max or matrix[y][x] == INT_MAX) {
                    x -= dx[dir];
                    y -= dy[dir];
                    break;
                }
                res.emplace_back(matrix[y][x]);
                matrix[y][x] = INT_MAX;
            }
            if (res.size() == x_max * y_max) {
                break;
            }
            dir = (dir + 1) % 4;
        }
        return res;
    }
};

int main() {
    Solution sln = Solution();
    vector<vector<int>> test = {{1, 2, 3},
                                {4, 5, 6},
                                {7, 8, 9}};
    vector<int> xs = sln.spiralOrder(test);
    copy(xs.begin(), xs.end(), ostream_iterator<int>(cout, ", "));
    cout << endl;
    return 0;
}
