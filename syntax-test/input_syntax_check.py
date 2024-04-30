'''
Helper script to check the syntax of the input.
It will print the characters and the number of times they are repeated sequentially.
'''

from itertools import tee, zip_longest

def pair(iterable):
    a, b = tee(iterable)
    next(b, None)
    return zip_longest(a, b)


available_days = ['01', '02', '03', '04', '05', '06']
print('Available days:', available_days)

day = input("Enter the day number: ")
if day not in available_days:
    print('Invalid day number')
    exit()
    
with open(f'syntax-test\\days\\{day}.txt', 'r') as file:
    chars = []
    counter = 1
    for c, nc in pair(file.read()):
        if c == nc and not c.isalpha() and not c.isnumeric():
            counter += 1
        else:
            chars.append((c, counter))
            counter = 1

    for ct in chars:
        if ct[1] > 1:
            print(f' [{repr(ct[0])}({ct[1]}x)] ', end='')
            continue
        if ct[0] == '\n':
            print(repr(ct[0]))
            continue
        if ct[0].isalpha() or ct[0].isnumeric() :
            print(ct[0], end='')
            continue
        print(ct[0], end='')
