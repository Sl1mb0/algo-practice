Longest Palindromic Substring
============================

A Palindrome is a word whose original and reversed versions are identical; e.g. "hannah == hannah"

Given a a string `s` of arbitrary length `n`; Find the longest palindromic sub-string in `s`.

Overview 
========

This solution uses a memoization table in order to utilize a "greedy" dynamic programming approach.

Each `i,j` entry in the table is a boolean value that indicates whether or not the substring `s[i:j]` is a palindrome.
Since we know that all single characters are palindromes themselves, this means that all `i,i` entries in the table will be true.
Expanding further, if two consecutive characters are equal, the `i,i+1` entry in the table will also be true.

We can then use these base cases to check if the characters at index `i` and `j` of substring `s[i:j]` are equal;
if they are, check the table entry at `i+1,j-1` to see if the substring `s[i+1:j-1]` is a palindrome. If it is,
store whichever is greater between it's length and the current longest palindrome length; and the indices `i,j` as well.
