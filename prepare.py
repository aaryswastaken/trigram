import re

output = []

with open("./blogtext.csv", "r", encoding='iso-8859-1') as source:
    s = source.read().split("\n")

    l = len(s)

    for (i, line) in enumerate(s):
        buffer = re.sub(r'.*,"           ', "", line)
        buffer = re.sub(r"[^a-zA-Z]", "", buffer)
        output.append(buffer.lower())

        if i%1000 == 0:
            print(f'{i/l * 100:.3f}% {i} / {l}')

with open("temp.txt", "w") as out:
    out.write("".join(output))

