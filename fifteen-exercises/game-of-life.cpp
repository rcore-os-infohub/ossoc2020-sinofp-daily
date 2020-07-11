#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    void gameOfLife(vector<vector<int>> &board) {
        auto last_board{board};
        const int dx[] = {-1, 0, 1, -1, 1, -1, 0, 1};
        const int dy[] = {-1, -1, -1, 0, 0, 1, 1, 1};
        std::size_t x_max = board[0].size();
        std::size_t y_max = board.size();
        for (int y = 0; y < y_max; ++y) {
            for (int x = 0; x < x_max; ++x) {
                int living_neighbours = 0;
                for (int i = 0; i < 8; ++i) {
                    int nx = x + dx[i];
                    int ny = y + dy[i];
                    if (nx > -1 and nx < x_max and ny > -1 and ny < y_max) {
                        living_neighbours += last_board[ny][nx];
                    }
                }
                if (last_board[y][x] == 1) {
                    board[y][x] = living_neighbours == 2 || living_neighbours == 3 ? 1 : 0;
                } else {
                    board[y][x] = living_neighbours == 3 ? 1 : 0;
                }
            }
        }
    }
};

int main() {
    cout << "ok" << endl;
    return 0;
}
