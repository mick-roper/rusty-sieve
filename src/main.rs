use std::{io::stdin, time::Instant};

fn main() {
    loop {
        println!("pick a number or type 0 to exit");
        let n = read_number();

        if n == 0 {
            break;
        }

        println!("finding primes numbers up to {}", n);

        let now = Instant::now();
        let resutls = sieve(n);
        let elapsed = now.elapsed();

        let mut c: usize = 0;
        for i in 2..n + 1 {
            if resutls[i] {
                // println!("{} is prime", i);
                c += 1;
            }
        }

        println!("it took {}ms to find {} primes", elapsed.as_millis(), c);
    }

    println!("bye bye!");
}

fn sieve(n: usize) -> Vec<bool> {
    let mut arr = vec![true; n + 1];
    arr[0] = false;
    arr[1] = false;
    for i in 2..n + 1 {
        if arr[i] {
            for j in (2 * i..n + 1).step_by(i) {
                arr[j] = false;
            }
        }
    }

    return arr;
}

fn read_number() -> usize {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}