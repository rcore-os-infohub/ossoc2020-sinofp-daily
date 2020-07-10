#include <iostream>
#include <vector>
#include <algorithm>
#include <functional>

using namespace std;

class Solution {
public:
    void rotate(vector<vector<int>> &matrix) {
        std::size_t n = matrix.size();
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < i + 1; ++j) {
                swap(matrix[i][j], matrix[j][i]);
            }
        }
        for (int i = 0; i < n; ++i) {
            for (int j = 0; j < n / 2; ++j) {
                swap(matrix[i][j], matrix[i][n - 1 - j]);
            }
        }
    }
};

int main() {
    cout << "ok" << endl;
    return 0;
}
