class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


"""
ListNode{
    val: 2, 
    next: ListNode{
        val: 4,
        next: ListNode{
            val: 3, 
            next: None
        }
    }
}
ListNode{val: 5, next: ListNode{val: 6, next: ListNode{val: 4, next: None}}}
输入：l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
输出：[8,9,9,9,0,0,0,1]
"""


class Solution:
    def list_len(self, l=ListNode()):
        ll = 1
        while True:
            if l.next:
                l = l.next
                ll += 1
            else:
                return ll

    def add(self, l1=ListNode(), l2=ListNode(), next_val=0):
        current_val = l1.val + l2.val + next_val
        if current_val >= 10:
            next_val = 1
            current_val = current_val - 10
        else:
            next_val = 0
        if l1.next:
            if l2.next:
                return ListNode(current_val, self.add(l1.next, l2.next, next_val))
            else:
                return ListNode(current_val, self.add(l1.next, ListNode(), next_val))
        else:
            if next_val == 1:
                return ListNode(current_val, ListNode(1, None))
            else:
                return ListNode(current_val, None)

    def add_two_numbers(self, l1, l2):
        # 使得函数add接收的l1参数长度总是大于等于l2参数长度
        if self.list_len(l1) > self.list_len(l2):
            return self.add(l1, l2)
        else:
            return self.add(l2, l1)


s = Solution()
a = s.add_two_numbers(ListNode(2, ListNode(4, ListNode(3, None))), ListNode(5, ListNode(6, ListNode(4, None))))
print(a.val, a.next.val, a.next.next.val)
