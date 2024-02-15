fn main() {
    let numbers: Vec<_> = vec![2, 4, 6, 8];

    let _plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();

    let _new_numbers: Vec<_> = numbers.iter().filter(|num| num > &&2).collect();

    let _find_number = numbers.iter().find(|num| num == &&9);

    let _count = numbers.iter().count();

    let _last = numbers.iter().last();

    let _min = numbers.iter().min();
    let _max = numbers.iter().max();

    let _take: Vec<&i32> = numbers.iter().take(3).collect();

    let _chained_filter = numbers.iter().map(|num| num + 1).filter(|num| num > &1);
}
