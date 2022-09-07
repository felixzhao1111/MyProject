"""给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。
abcabcbb
"""

class SSolution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        hashtable = dict()
        for i, ss in enumerate(s):
            if ss not in hashtable:
                hashtable[ss] = {"index": i, "len": 1, "count": 1}  # [i, 1]
            else:
                hashtable[ss]["len"] = i - hashtable[ss]["index"]
                hashtable[ss]["index"] = i
                hashtable[ss]["count"] += 1
        longest_str = {"str": "", "len": 0, "count": 0}  # ["", 0, 0]
        for k in hashtable:
            if hashtable[k]["len"] > longest_str["len"]:
                longest_str["str"] = k
                longest_str ["len"] = hashtable[k]["len"]
                longest_str["count"] = hashtable[k]["count"]
        if longest_str["len"] == 1:
            if longest_str["count"] == 1:
                return len(s)
            else:
                return 1
        else:
            return longest_str["len"]

class Solution:
    def lengthOfLongestSubstring(self, s) -> int:
        s_len = len(s)
        hashtable = dict()
        for i, ss in enumerate(s):
            if ss not in hashtable:
                hashtable[ss] = {"index": i, "len": 1, "count": 1, "to_head": i, "to_end": s_len-i+1}
            else:
                hashtable[ss]["len"] = i - hashtable[ss]["index"]
                hashtable[ss]["index"] = i
                hashtable[ss]["count"] += 1
                hashtable[ss]["to_end"] = s_len-i+1
        longest_str = {"str": "", "len": 0, "count": 0}  # ["", 0, 0]
        for k in hashtable:
            if hashtable[k]["len"] < hashtable[k]["to_head"]:
                hashtable[k]["len"] = hashtable[k]["to_head"]
            if hashtable[k]["len"] < hashtable[k]["to_end"]:
                hashtable[k]["len"] = hashtable[k]["to_end"]
            if hashtable[k]["len"] > longest_str["len"]:
                longest_str["str"] = k
                longest_str["len"] = hashtable[k]["len"]
                longest_str["count"] = hashtable[k]["count"]
        if longest_str["len"] == 1:
            if longest_str["count"] == 1:
                return len(s)
            else:
                return 1
        else:
            return longest_str["len"]


s = Solution()
a = s.lengthOfLongestSubstring("aab")
print("长度：", a)
