with open("1.input") as f:
    lines = f.readlines()

input = []

for line in lines:
	input.append((line[:1], int(line[1:])))

dial = 50
zero_count = 0

for direction, turns in input:
	if direction == "R":
		dial += turns
	else:
		dial -= turns
	dial = (dial % 100)
	if dial == 0:
		zero_count += 1
    

print(zero_count)
