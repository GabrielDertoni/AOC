
@discardableResult
func minHeapPop<T: Comparable>(_ array: inout [T]) -> T? {
    if array.count == 0 {
        return nil
    }
    array.swapAt(0, array.count - 1)
    let element = array.popLast()
    var i = 0
    while i * 2 + 1 < array.count {
        let child: Int
        if i * 2 + 2 >= array.count || array[i * 2 + 1] < array[i * 2 + 2] {
            child = i * 2 + 1
        } else {
            child = i * 2 + 2
        }
        if array[i] > array[child] {
            array.swapAt(i, child)
        } else {
            break
        }
        i = child
    }
    return element
}

func minHeapPush<T: Comparable>(_ array: inout [T], _ val: T) {
    array.append(val)

    var i = array.count - 1
    while i > 0 && array[i] < array[(i - 1) / 2] {
        let parent = (i - 1) / 2
        array.swapAt(i, parent)
        i = parent
    }
}

var best = Array(repeating: 0, count: 3)
var acc = 0
while let line = readLine() {
    if line == "" {
        if acc > best[0] {
            minHeapPop(&best)
            minHeapPush(&best, acc)
        }
        acc = 0
        continue
    }
    acc += Int(line)!
}

if acc > best[0] {
    minHeapPop(&best)
    minHeapPush(&best, acc)
}

print("\(best.reduce(0, +))")
