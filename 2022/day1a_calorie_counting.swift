var best = 0
var acc = 0
while let line = readLine() {
    if line == "" {
        if acc > best {
            best = acc
        }
        acc = 0
        continue
    }
    acc += Int(line)!
}
if acc > best {
    best = acc
}

print("\(best)");
