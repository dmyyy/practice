class Solution(object):
    def twoSum(self, nums, target):
        """
        :type nums: List[int]
        :type target: int
        :rtype: List[int]
        """

        # trivial n^2 soln
        # for (i, n1) in enumerate(nums):
        #     for (j, n2) in enumerate(nums):
        #         if (n1 + n2) == target:
        #             return (i, j)

        # create map: num -> index
        num_map = {}
        for (i, n) in enumerate(nums):
            complement = target - n
            # if we've already seen the complement, return the pair
            if complement in num_map:
                return (num_map[complement], i)
            # map from num to its index
            num_map[n] = i
            
solution = Solution()
print(solution.twoSum([2, 7, 11, 15], 9))
