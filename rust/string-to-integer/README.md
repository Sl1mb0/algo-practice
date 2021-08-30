String -> Integer
=================

Implement the `myAtoi(string s)` function that converts a string to a 32-bit signed integer.

Overview
========

I wrote two functions:


## `my_atoi_small()`

While the amount of memory allocated is significantly reduced, having to find the signed symbol (`+` or `-`) multiple times 
instead of finding it once and storing it's index means things are going to take longer. An improvement could be made by storing the index instead.
Finding the index of the symbol is necessary regardless; since computing a digit's decimal value is based on it's position relative to other digits.
We can't count how many digits there are without knowing where the symbol is, because there could be any number of whitespaces before the number,
and we won't always know which number to look for to begin counting.

## `my_atoi_fast()`

On the other hand, the optimized for time version uses checks to store the symbol once we've found it; and storing a reversed version of the string
eliminates the need to find the index for a symbol, and allows us to compute the decimal value of each digit by using the "reversed index".
While reversing the string does add processing overhead, It is only one additional loop as opposed to the worst-case 3 additional loops in `my_atoi_small()`.
