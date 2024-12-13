


def prime_numbers(s):
    nums = []
    for i in range(2,s+1):
        check = True
        for i2 in range(2,i+1):
            if i %i2 == 0 and i != i2:
                check = False
                
        if check:
            nums.append(i)

    return nums

print(prime_numbers(10))


def func(L):
 return [i /3  for i in L if i % 3 ==0 ]
assert func([3, 4, 2, 7, 9, 15]) == [1, 3, 5]


def reverse_string(s):
    if len(s) == 0:
        return ""
    else:
        return reverse_string(s[1:]) + s[0]
        

print(reverse_string("rtwr"))