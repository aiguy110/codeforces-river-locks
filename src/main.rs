use std::io::BufRead;

#[derive(PartialEq, Debug)]
struct Puzzle {
    lock_vols: Vec<f32>,
    queries: Vec<f32>
}

fn read_puzzle<R>(input: &mut R) -> Puzzle 
    where R: BufRead
{
    // Burn the first line of input
    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();

    // Read in the lock volumes
    buf.clear();
    input.read_line(&mut buf).unwrap();
    let mut puzzle = Puzzle {
        lock_vols: buf.trim().split(' ').map(|s| s.parse().unwrap()).collect(),
        queries: Vec::new()
    };

    // Find out how many queries we're dealing with
    buf.clear();
    input.read_line(&mut buf).unwrap(); 
    let q = buf.trim().parse().unwrap();

    // Read in q queries
    puzzle.queries = Vec::with_capacity(q);
    for _ in 0..q {
        buf.clear();
        input.read_line(&mut buf).unwrap();
        puzzle.queries.push( buf.trim().parse().unwrap() )
    }

    puzzle
}

/// Only works if `vals` is sorted in assending order
fn find_first_above(vals: &[f32], x: f32) -> usize {
    let mut bottom = 0;
    let mut top    = match vals.len() { 0 => 0, _ => vals.len()-1};
    while top - bottom > 1 {
        let i = bottom + (top - bottom) / 2;
        if vals[i] <= x {
            bottom = i;
        } else {
            top = i;
        }
    }
    top
}

/// Only works if `vals` is sorted in decending order
fn find_first_at_or_below(vals: &[f32], x: f32) -> usize {
    let mut bottom = match vals.len() { 0 => 0, _ => vals.len()-1};
    let mut top    = 0;
    while bottom - top > 1 {
        let i = top + (bottom - top) / 2;
        if vals[i] <= x {
            bottom = i;
        } else {
            top = i;
        }
    }
    bottom
}

fn time_to_fill_vol(vol: f32, in_rate_incrament_times: &[f32], start_rate: f32) -> f32{
    let mut current_level = 0.0;
    let mut current_time  = 0.0;
    let mut current_rate  = start_rate;
    let mut current_rate_incrament_ind = 0;

    // Simulate water flowing into this volume until it is full.
    loop {
        let time_to_fill_at_current_rate = current_time + (vol - current_level) / current_rate;

        if current_rate_incrament_ind == in_rate_incrament_times.len() {
            return time_to_fill_at_current_rate; 
        }
        
        let next_rate_incrament_time = in_rate_incrament_times[current_rate_incrament_ind];
        current_rate_incrament_ind += 1;
        if next_rate_incrament_time >= time_to_fill_at_current_rate {
            return time_to_fill_at_current_rate; 
        } 

        current_level += current_rate * (next_rate_incrament_time - current_time);
        current_time = next_rate_incrament_time;
        current_rate += 1.0;
    };    
}

fn total_fill_and_out_rate_incrament_times(
    in_rate_incrament_times: Vec<f32>,
    lock_vols: &[f32],
    lock_ind: usize
) -> (f32, Vec<f32>) {
    let time_to_fill_this = time_to_fill_vol(lock_vols[lock_ind], &in_rate_incrament_times, 1.0);
    
    let mut out_rate_incrament_times = in_rate_incrament_times;
    let insert_ind = find_first_above(&out_rate_incrament_times, time_to_fill_this);
    out_rate_incrament_times.insert(insert_ind, time_to_fill_this);
    for i in 0..insert_ind {
        out_rate_incrament_times[i] = time_to_fill_this;
    }

    let remaining_volume: f32 = lock_vols[lock_ind+1..].iter().sum();
    let time_to_fill_rest = time_to_fill_vol(remaining_volume, &out_rate_incrament_times, 0.0);

    let mut time_to_fill_all = time_to_fill_rest;
    let last_rate_incrament_time = out_rate_incrament_times[out_rate_incrament_times.len()-1];
    if last_rate_incrament_time > time_to_fill_rest {
        time_to_fill_all = last_rate_incrament_time;
    }
    (time_to_fill_all, out_rate_incrament_times)
}

fn solve_puzzle(puzzle: &Puzzle) -> Vec<i32> {
    // Pre-process puzzle
    let mut finish_times = vec![0.0; puzzle.lock_vols.len()];
    let mut rate_incrament_times = vec![];
    for i in 0..finish_times.len() {
        //println!("rate_incrament_times: {:?}", rate_incrament_times);
        (finish_times[i], rate_incrament_times) = total_fill_and_out_rate_incrament_times(rate_incrament_times, &puzzle.lock_vols, i);
    }
    //println!("finish_times: {:?}", finish_times);
    //println!("queries     : {:?}", puzzle.queries);

    // Get answers to queres
    let mut answers = Vec::with_capacity(puzzle.queries.len());
    for &t in puzzle.queries.iter() {
        if finish_times[finish_times.len()-1] > t {
            answers.push(-1);
        } else {
            answers.push( find_first_at_or_below(&finish_times, t) as i32 + 1)
        }
    }
    
    answers
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;
    use std::time::{Instant, Duration};

    #[test]
    fn read_puzzle_works() {
        let mut input = Cursor::new("2\n1 2\n3\n3\n2\n1");
        assert_eq!(read_puzzle(&mut input), Puzzle {
            lock_vols: vec![1.0, 2.0],
            queries:   vec![3.0, 2.0, 1.0]
        });
    }

    #[test]
    fn tfaorit_works_easy() {
        assert_eq!(
            total_fill_and_out_rate_incrament_times(vec![], &vec![1.0, 2.0], 0),
            (3.0, vec![1.0])
        )
    }

    #[test]
    fn sample_1() {
        let mut input = Cursor::new(include_str!("../test_input_1.txt"));
        let puzzle = read_puzzle(&mut input);
        assert_eq!(solve_puzzle(&puzzle), vec![-1, 3,-1,-1, 4, 3]);
    }

    #[test]
    fn sample_2() {
        let mut input = Cursor::new(include_str!("../test_input_2.txt"));
        let puzzle = read_puzzle(&mut input);
        assert_eq!(solve_puzzle(&puzzle), vec![-1,-1, 4, 4,-1, 5]);
    }

    fn speed_test(n: usize) {
        let puzzle = Puzzle {
            lock_vols: vec![1.0; n],
            queries: vec![1.0; n]
        };

        let start = Instant::now();
        solve_puzzle(&puzzle);
        let solve_duration = Instant::now() - start;
        assert!(solve_duration < Duration::from_secs(2));
    }

    #[test]
    fn speed_test_1k() {
        speed_test(1000);
    }
    
    #[test]
    fn speed_test_10k() {
        speed_test(10_000);
    }
    
    #[test]
    fn speed_test_200k() {
        speed_test(200_000);
    }
}