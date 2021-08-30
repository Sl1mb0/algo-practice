String -> Integer
=================

Implement the `myAtoi(string s)` function that converts a string to a 32-bit signed integer.

Overview
========

I wrote two functions; each optimizing for either space, or time. 

In the optimized for space version, while the amount of memory allocated is significantly reduced,
Having to find the signed symbol (`+` or `-`) multiple times instead of finding it once and storing it's index introduces more time complexity.
Finding the index of the symbol is necessary; since computing a digit's decimal value is based on it's position relative to other digits,
and we don't know  how many digits there are since a string can contain an arbitrary amount of whitespace as well.

On the other hand, the optimized for time version uses checks to store the symbol once we've found it; and storing a reversed version of the string
eliminates the need to find the index for a symbol, and allows us to compute the decimal value of each digit by using the "reversed index".
