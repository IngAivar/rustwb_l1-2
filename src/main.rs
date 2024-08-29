use std::thread;

fn main() {
    let n: i32 = 10; // Размер массива
    let _vec_n: Vec<i32> = (1..=n).collect();  // Создайте вектор, содержащий числа от 1 до n

    let mut handles = vec![]; // Вектор для хранения дескрипторов потоков
    for num_1_to_n in _vec_n {
        let handle = thread::spawn(move || { // Создайте новый поток и переместите значение num_1_to_n в его область действия
            println!("{}^2 = {}", num_1_to_n, num_1_to_n * num_1_to_n); // Выведите квадрат этого числа
        });
        handles.push(handle); // Добавьте дескриптор потока к вектору
    }

    for handle in handles {
        handle.join().unwrap(); // Дождитесь завершения потока и верните его результат (если таковой имеется)
    }
}