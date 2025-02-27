# The nth Finonacci number

This is a specific number Fibonacci sequence, which is a famous sequence of numbers in mathematics. 


# Definition of the Finonacci Sequence 

1. The first Fibonacci number is 0
2. The second Fibonacci number is 1
3. Every Fibonacci number is the sum of the two preceeding ones

Mathemiatically it's F_n = F_n-1 + F_n-2

# Fibonacci function

> This takes in an unsigned 32 bit integer n, as the input parameter, and returns a 64 bit unsigned integer as the type of the return value
> Within the function handles for when n is 0 or 1, since the first two Fibonacci numbers is 0 and 1
> the for .. in loop contains an inclusive range, eg 2..=5 will iterate over numbers "2, 3, 4, 5". the _ basically means "I don't care about the value of the loop variable, I just want to run the loops a certain number of times". 
> Now the for _ in 2..=n, n = 0 and 1 are handled as base cases, so the loop starts from 2 since we know the values for n = 0 and n = 1

# main function

> ref comments in code



?It didnt work??
or rather, it gives the error message "error: process didn't exit successfully: `target\debug\nth-fibonacci-number.exe` (exit code: 259)" if the number isn't provided early
