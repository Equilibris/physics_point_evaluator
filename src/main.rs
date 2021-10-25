use std::collections::HashSet;
use std::io::stdin;

// mod config;
// mod operator;
// mod symbol;
// mod tree;

fn get_sets() -> (
    HashSet<&'static str>,
    HashSet<&'static str>,
    HashSet<&'static str>,
    HashSet<&'static str>,
    HashSet<&'static str>,
    HashSet<&'static str>,
) {
    let vel = "v = a * t + v_0";
    let s_1 = "s = t*(v + v_0)/2";
    let s_2 = "s = 1/2 a * t^2 + t * v_0";
    let timeless = "v^2 - v_0^2 = 2 a * s";

    let mut all_set = HashSet::new();
    let mut v_set = HashSet::new();
    let mut a_set = HashSet::new();
    let mut t_set = HashSet::new();
    let mut s_set = HashSet::new();
    let mut v_0_set = HashSet::new();

    all_set.insert(vel);
    all_set.insert(s_1);
    all_set.insert(s_2);
    all_set.insert(timeless);

    v_set.insert(vel);
    v_set.insert(s_1);
    v_set.insert(timeless);

    a_set.insert(vel);
    a_set.insert(s_2);
    a_set.insert(timeless);

    t_set.insert(vel);
    t_set.insert(s_1);
    t_set.insert(s_2);

    s_set.insert(s_1);
    s_set.insert(s_2);
    s_set.insert(timeless);

    v_0_set.insert(vel);
    v_0_set.insert(s_1);
    v_0_set.insert(s_2);
    v_0_set.insert(timeless);

    (all_set, v_set, a_set, t_set, s_set, v_0_set)
}

macro_rules! diff_eq {
    ($goal: expr, $target:expr, $subr:expr) => {
        for i in $goal.clone().difference(&$target) {
            $goal.remove(*i);
        }
        $subr -= 1;
    };
}

fn mode_get() {
    let (all_set, v_set, a_set, t_set, s_set, v_0_set) = get_sets();

    println!("Num of knowns");

    let mut s = String::new();

    stdin().read_line(&mut s).unwrap();

    match s.trim().parse::<usize>() {
        Ok(num) => {
            let mut num = num;

            let mut current_set = all_set;

            while num > 0 {
                println!("Enter name of known: v, v_0, a, t, s");
                println!("Your current options are {:?}", current_set);

                let mut s = String::new();
                stdin().read_line(&mut s).unwrap();

                let s = s.to_lowercase();

                if s.contains("v") {
                    if s.contains("0") {
                        diff_eq!(current_set, v_0_set, num);
                    } else {
                        diff_eq!(current_set, v_set, num);
                    }
                } else if s.contains("s") {
                    diff_eq!(current_set, s_set, num);
                } else if s.contains("t") {
                    diff_eq!(current_set, t_set, num);
                } else if s.contains("a") {
                    diff_eq!(current_set, a_set, num);
                }
            }

            println!("Your final options are {:?}", current_set);
        }
        Err(_) => {
            println!("Please provide valid usize");
            return mode_get();
        }
    }
}

fn main() {
    loop {
        println!("Enter mode (g)et, (e)xit");

        let mut s = String::new();

        stdin().read_line(&mut s).unwrap();

        let s = s.chars().next();

        if let Some('g') = s {
            mode_get();
        } else if let Some('e') = s {
            return;
        }
    }
}
