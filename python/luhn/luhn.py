from typing import Tuple


class Luhn:
    def __init__(self, card_num: str):
        self.card_num = None
        self._check_sum = None
        def _dubling(args: Tuple[int, int]) -> int:
            (index, num) = args
            if index % 2:
                num += num
                if num > 9:
                    num -= 9
            return num

        try:
            nums = [ int(num) for num in card_num if not num.isspace() ]
            if len(nums) > 1:
                nums.reverse()
                self._check_sum = sum(map(_dubling, enumerate(nums)))
                self.card_num = nums
        except:
            pass

    def valid(self) -> bool:
        return bool(self.card_num and not self._check_sum % 10)
