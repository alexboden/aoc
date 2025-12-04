import sys

lines = sys.stdin.read()

input = []

for line in lines.split('\n'):
	input.append((line[:1], int(line[1:])))

dial = 50
zero_count = 0

for direction, turns in input:
	complete_turns = turns // 100
	zero_count += complete_turns
	turns -= complete_turns * 100

	prev = dial
	if direction == "R":
		dial = (dial + turns) % 100
		if dial < prev:
			zero_count += 1
	else:
		dial = (dial - turns) % 100
		if (dial > prev and prev != 0) or dial == 0:
			zero_count += 1
	
print(zero_count)
