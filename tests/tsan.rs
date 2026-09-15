#![deny(warnings)]

use std::thread;

use zero::RingBuffer;

#[test]
fn thread_spwan() {
    static RB: RingBuffer<i32, 6> = RingBuffer::new();

    let rb = &RB;

    let (producer, mut consumer) = rb.split();

    producer.enqueue(1).unwrap();

    thread::spawn(move || {
        producer.enqueue(2).unwrap();
    });

    thread::spawn(move || {
        consumer.dequeue().unwrap();
    });
}

#[test]
fn thread_scope() {
    static RB: RingBuffer<i32, 6> = RingBuffer::new();
    thread::scope(|scope| {
        scope.spawn(|| {
            let (producer, _) = RB.split();

            for i in 0..100 {
                while producer.enqueue(i).is_err() {}
            }
        });

        scope.spawn(|| {
            let (_, mut consumer) = RB.split();

            for _ in 0..100 {
                while consumer.dequeue().is_none() {}
            }
        });
    })
}

#[test]
fn contention() {
    const N: usize = 1024;

    let rb: RingBuffer<u8, N> = RingBuffer::new();

    {
        let (producer, mut consumer) = rb.split();

        thread::scope(move |scope| {
            scope.spawn(move || {
                let mut sum: u32 = 0;

                for i in 0..(2 * N) {
                    sum = sum.wrapping_add(i as u32);
                    while producer.enqueue(i as u8).is_err() {}
                }

                println!("producer: {sum}");
            });

            scope.spawn(move || {
                let mut sum: u32 = 0;

                for _ in 0..(2 * N) {
                    loop {
                        if let Some(v) = consumer.dequeue() {
                            sum = sum.wrapping_add(v as u32);
                            break;
                        }
                    }
                }

                println!("consumer: {sum}");
            });
        });
    }

    assert_eq!(rb.len(), 0);
}
