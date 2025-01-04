use crate::geom::Line;
use crate::geom::Point;
use crate::num::Num;

#[derive(Clone, Debug)]
pub struct LineIterator<N: Num = f32> {
    current_n: Point<N>,
    current: Point<f32>,
    end: Point<f32>,
    step: Point<f32>,
    iteration_type: IterationType,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum IterationType {
    Exclusive,
    Inclusive,
    NoMovement,
    Done,
}

impl<N: Num> LineIterator<N> {
    pub fn new(line: Line<N>, step: N, is_exclusive: bool) -> Self {
        let line_f32 = line.to_f32();
        let current = line_f32.start();
        let end = line_f32.end();
        let step_f32 = step.to_rounded();
        let mut step = line.direction().to_point() * step_f32;

        if current.x() == end.x() {
            step.set_x(0.0);
        }

        if current.y() == end.y() {
            step.set_y(0.0);
        }

        if step_f32 == 0.0 {
            panic!("Zero step given");
        }

        if step_f32 < 0.0 {
            panic!("Negative step given");
        }

        Self {
            current_n: current.from_f32(),
            current,
            end,
            step,
            iteration_type: calculate_iteration_type(line, is_exclusive),
        }
    }
}

fn calculate_iteration_type<N: Num>(line: Line<N>, is_exclusive: bool) -> IterationType {
    if is_exclusive {
        if line.is_empty() {
            IterationType::Done
        } else {
            IterationType::Exclusive
        }
    } else {
        if line.is_empty() {
            IterationType::NoMovement
        } else {
            IterationType::Inclusive
        }
    }
}

impl<N: Num> Iterator for LineIterator<N> {
    type Item = Point<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration_type == IterationType::Done {
            return None;
        }

        let last = self.current_n;
        loop {
            if has_moved_to_final_iteration(self.current, self.end, self.step) {
                let is_exclusive = self.iteration_type == IterationType::Exclusive;
                self.iteration_type = IterationType::Done;

                if is_exclusive {
                    return None;
                } else {
                    return Some(self.end.from_f32());
                }
            }

            self.current = self.current + self.step;

            let next = self.current.from_f32();
            if next != last {
                self.current_n = next;
                break;
            }
        }

        Some(last)
    }
}

fn has_moved_to_final_iteration(current: Point<f32>, end: Point<f32>, step: Point<f32>) -> bool {
    if current == end {
        return true;
    }

    if step.x() < 0.0 && current.x() < end.x() {
        return true;
    }

    if 0.0 < step.x() && end.x() < current.x() {
        return true;
    }

    if step.y() < 0.0 && current.y() < end.y() {
        return true;
    }

    if 0.0 < step.y() && end.y() < current.y() {
        return true;
    }

    false
}
