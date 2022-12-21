
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
    case "A": return .rock
    case "B": return .paper
    case "C": return .scissors
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
    let your_choice: Choice

    switch (opponent_choice, your) {
    case (.rock, "X"):
        your_choice = .scissors
    case (.paper, "X"):
        your_choice = .rock
    case (.scissors, "X"):
        your_choice = .paper
    case let (a, "Y"):
        your_choice = a
    case (.rock, "Z"):
        your_choice = .paper
    case (.paper, "Z"):
        your_choice = .scissors
    case (.scissors, "Z"):
        your_choice = .rock
    default: unreachable()
    }

    switch (your) {
    case "Y":
        total += 3
    case "Z":
        total += 6
    default: break
    }

    total += score(your_choice)
}

print("\(total)")
