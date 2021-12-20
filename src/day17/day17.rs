// 19.12.2021 18:06
//target area: x=156..202, y=-110..-69

#[derive(Debug, Copy, Clone, PartialEq)]
struct Step {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn trajectory((vx, vy): (i32, i32)) -> impl Iterator<Item = Step> {
    use std::cmp::Ordering::*;
    std::iter::successors(Some(Step { x: 0, y: 0, vx, vy }), |prev| {
        Some(Step {
            x: prev.x + prev.vx,
            y: prev.y + prev.vy,
            vx: match prev.vx.cmp(&0) {
                Less => prev.vx + 1,
                Equal => 0,
                Greater => prev.vx - 1,
            },
            vy: prev.vy - 1,
        })
    })
}

pub fn run_trajectory ( target: &((i32,i32),(i32,i32)), initial_velocity : (i32, i32) ) -> Option<i32> {
    let mut y_max = i32::MIN;
    let mut counter = 0;
    for step in trajectory(initial_velocity) {
        if step.x > target.0.1 || step.y < target.1.0 {
            return None;
        }
        if step.x >= target.0.0 && step.x <= target.0.1 && step.y >= target.1.0 && step.y <= target.1.1 { 
            break;
        }
        y_max = y_max.max(step.y);
    }
    Some(y_max)
}

pub fn find_best_start_vel ( target: &((i32,i32),(i32,i32)) ) -> i32 {
    let mut y_max = i32::MIN;
    for vx in 1..target.0.1 {
        for vy in target.1.0 .. target.0.1.abs() {
            if let Some(y) = run_trajectory(target, (vx, vy)) {
                if y_max < y { y_max = y; }
            }
        }
    }
    y_max
}

pub fn count_pos_vels ( target: &((i32,i32),(i32,i32)) ) -> i32 {
    let mut y_max = i32::MIN;
    let mut counter = 0;
    for vx in 1..target.0.1+1 {
        for vy in target.1.0 .. target.0.1.abs()  {
            if run_trajectory(target, (vx, vy)).is_some() {
                counter += 1;
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    //target area: x=156..202, y=-110..-69

    static TARGETAREA : ((i32,i32),(i32,i32)) = ( (156,202), (-110,-69) );

    #[test]
    fn trajectory_test() {
        //x=20..30, y=-10..-5
        let target_area = ( (20,30), (-10, -5) );
        assert_eq!(run_trajectory(&target_area,(7,2)), Some(3) );
    }

    #[test]
    fn trajectory_test_2() {
        //x=20..30, y=-10..-5
        let target_area = ( (20,30), (-10, -5) );
        assert_eq!(run_trajectory(&target_area,(6,3)), Some(6) );
    }

    #[test]
    fn trajectory_test_3() {
        //x=20..30, y=-10..-5
        let target_area = ( (20,30), (-10, -5) );
        assert_eq!(run_trajectory(&target_area,(17,-4)), None );
    }

    #[test]
    fn test_velocities () {
        //x=20..30, y=-10..-5
        let target_area = ( (20,30), (-10, -5) );
        assert_eq!(find_best_start_vel(&target_area), 45 );
    }

    #[test]
    fn test_count_velocities () {
        //x=20..30, y=-10..-5
        let target_area = ( (20,30), (-10, -5) );
        assert_eq!(count_pos_vels(&target_area), 112 );
    }

    #[test]
    fn test_trajectories () {
        //x=20..30, y=-10..-5
        let target_area = ( (20,30), (-10, -5) );
        assert_eq!(run_trajectory(&target_area,(30,-9)), Some(0) );
    }

    #[test]
    fn riddle_1() {
        assert_eq!(find_best_start_vel(&TARGETAREA), 5995 );
    }

    #[test]
    fn riddle_2() {
        assert_eq!(count_pos_vels(&TARGETAREA), 3202 );
    }

}

