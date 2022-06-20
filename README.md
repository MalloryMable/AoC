# AoC
Advent of Code is a yearly exercise done around christmas. I have decided to work on different languages and build a portfolio of little projects with this. Each Program so far is a command line tool that can take any text file and defaults to ./file.txt.

**2015**

**Day 1**

Using parathesisis we track "floors" as well as marking the first time we "reach the basement"(pass a count of -1).

**Day 2**

Takes the dimensions of a rectanglular prisim to find area plus the size of the smallest side as well as the shortest length around said rectangular rectangular prism, and the volume of the object, all tracked by amount of wrapping paper and ribbons required.

**Day 3**

Move through a 2d grid of arbitrary dimensions one space at a time based on the directional intructions '>' = right, '<' = left, '^' = up, and 'v' = down. 

Part 1

Tracks the number of positions a single person vists following each instruction

Part 2

Splits instructions between 2 people and tracks tne number of houses tackled colaberatively taking turns to execute movements.

**Day 4**

NOTE: This tool takes an argument of the starting condition and NOT a file. Although the default ./file.txt still works if no arguments are called
A brute force method of cracking an MD5 hash given some required number of digits in base 16.

Part 1

Finds the value for a hash that produces 5 leading zeros(~1 min).

Part 2

Finds the value for a has that produces 6 leading zeros(~15 min).

**Day 5**

A pair of text matching programs that count up the number of lines that meet a certain condition.

Part 1

Condtions are 3 vowels at any position, any pair of letters, and Not containing "ab", "cd", "pq", or "xy"

Part 2

Conditions are any pair of letters repeats, any letter repeats with exactly one letter between the instances

**Day 6**

Reading instructions formatted "[INSTRUCTION] (x,y) through (x,y)" from a file to control a 1000x1000 grid of lights. The instructions are "turn on", "turn off", "toggle"

Part 1

Turns lights on and off based on instructions, and returns the total number of turned on lights

Part 2 

Turns up and down lights where 0 is off, turn off turns down, turn on turns up by a factor of 1, and toggle turns up by a factor of two. Returns the total amount of light(lumens?) produced 


**Day 7**

Models a curcuit and after propogating a 16 bit signal through memory returns the value being passed through wire A, then runs the same curcuit where the value being passed through wire B is reset to whatever the output from wire A was.

**Day 8**

Part 1

Removes escape characters to read back plaintext and returns the length of this new array

Part 2

Add proper escape characters for subsequent users and returns the length of this program

**Day 9**

[N/A]

**Day 10**

Takes a single line as an argument or reads from ./file.txt

Implements a look and say program that returns the length for a given input at both 40 and 50 cycles.
