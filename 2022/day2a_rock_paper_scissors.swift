
enum Choice: Equatable {
    case rock
    case paper
    case scissors
}

func score(_ choice: Choice) -> Int {
    switch choice {
    case .rock:     return 1
    case .paper:    return 2
    case .scissors: return 3
    }
}

func getChoice(_ s: Substring) -> Choice {
    switch s {
    case "A", "X": return .rock
    case "B", "Y": return .paper
    case "C", "Z": return .scissors
    default: unreachable()
    }
}

func unreachable() -> Never {
    fatalError("unreachable")
}

var total = 0
while let line = readLine() {
    let parts = line.split(separator: " ")
    let opponent = parts[0]
    let your = parts[1]

    let opponent_choice = getChoice(opponent)
    let your_choice = getChoice(your)

    switch (opponent_choice, your_choice) {
    case (.rock, .paper),
         (.paper, .scissors),
         (.scissors, .rock):
        total += 6
    case let (a, b) where a == b:
        total += 3
    default: break
    }

    total += score(your_choice)
}

print("\(total)")
