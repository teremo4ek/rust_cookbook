fn main() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        // `iter()` создает итератор массива (тип значения - `Some(&i32)`)
        // `cloned()` - создает новый итератор, клонирующий значения предыдущего итератора
        //  (`&T` преобразуется в `T`, типом значения становится - `Some(i32)`)
        // `max()` - возвращает максимальный элемент итератора
        return arr.iter().cloned().max();
    }

    // Делим массив пополам
    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        // Создаем параллельные потоки для обработки левой и правой частей массива
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        // Получаем максимальные значения из потоков
        // `join()` - ожидает завершения потока
        // (заставляет основной поток ждать завершения выделенного потока)
        // и возвращает его результат
        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        // `max()` сравнивает и возвращает максимальное из двух значений
        Some(max_l.max(max_r))
    })
    .unwrap()
}
