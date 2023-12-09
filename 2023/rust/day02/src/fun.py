# rapid script in python fun for fun
import time

start = time.perf_counter()
with open("src/input.txt", "r") as fichier:
    data = fichier.readlines()

games = {}
for line in data:
    game = int(line.split(":")[0].split(" ")[1])
    hands = []
    for hand in line.split(":")[1].split(";"):
        dic = {}
        for part in hand.split(","):
            parts = part.strip().split(" ")
            dic[parts[1]] = int(parts[0])
        hands.append(dic)
    games[game] = hands

s = 0
color_max = {"red": 12, "green": 13, "blue": 14}
for num, hands in games.items():
    for hand in hands:
        for color, dices in hand.items():
            if dices > color_max[color]:
                break
        else:
            continue
        break
    else:
        s += num

perfo = time.perf_counter() - start
print(f"result: {s}")
print(f"Done in: {perfo*1000}ms")
