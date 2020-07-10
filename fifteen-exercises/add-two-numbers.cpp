#include <iostream>

struct ListNode {
    int val;
    ListNode *next;

    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    ListNode *addTwoNumbers(ListNode *l1, ListNode *l2) {
        int carry = 0;
        auto *head = new ListNode(0);
        ListNode *cur = head;
        while (l1 != nullptr or l2 != nullptr or carry) {
            int sum = (l1 != nullptr ? l1->val : 0) + (l2 != nullptr ? l2->val : 0) + carry;
            carry = sum / 10;
            cur->next = new ListNode(sum % 10);
            cur = cur->next;
            l1 = l1 != nullptr ? l1->next : nullptr;
            l2 = l2 != nullptr ? l2->next : nullptr;
        }
        return head->next;
    }
};

int main() {
    auto *l1 = new ListNode(2);
    l1->next = new ListNode(4);
    l1->next->next = new ListNode(3);
    auto *l2 = new ListNode(5);
    l2->next = new ListNode(6);
    l2->next->next = new ListNode(4);

    Solution sln = Solution();
    ListNode *res = sln.addTwoNumbers(l1, l2);
    std::cout << res->val << std::endl;
    std::cout << res->next->val << std::endl;
    std::cout << res->next->next->val << std::endl;
    return 0;
}