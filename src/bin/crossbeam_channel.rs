use crossbeam_channel::bounded;
use std::thread;
use std::time::Duration;

fn main() {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(1);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // Поток производителя
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }
            // Закрываем канал - это необходимо для выхода
            // из цикла `for` в воркере
            drop(snd1);
        });

        // Параллельная обработка двумя потоками/воркерами
        for _ in 0..n_workers {
            // Отправляем в приемник, получаем из источника
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            // Создаем воркеров в отдельных потоках
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));
                // Получаем сообщения до закрытия канала
                for msg in recvr.iter() {
                    println!("Worker {:?} received {}", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        // Закрываем канал, иначе приемник никогда не выйдет из цикла `for`
        drop(snd2);

        // Приемник
        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap();
}
