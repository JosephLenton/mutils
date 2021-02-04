use crate::{Line, Point};
use ::std::iter::Iterator;

pub struct LineIterator<'a> {
    points: &'a [f32],
}

impl<'a> LineIterator<'a> {
    pub fn new(points: &'a [f32]) -> Self {
        assert!(
            (points.len() % 4) == 0,
            "length of points given should be divisible by 4, instead divisible by {}",
            points.len()
        );

        Self { points }
    }
}

impl<'a> Iterator for LineIterator<'a> {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if self.points.len() > 0 {
            let start = Point(self.points[0], self.points[1]);
            let end = Point(self.points[2], self.points[3]);
            let line = Line(start, end);

            self.points = &self.points[4..];

            return Some(line);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_return_an_empty_list_if_no_points() {
        let iterator = LineIterator::new(&[]);

        let lines: Vec<Line> = iterator.collect();
        assert_eq!(lines, []);
    }

    #[test]
    fn it_should_return_the_lines_given() {
        let iterator = LineIterator::new(&[
            0.0, 1.0, 10.0, 13.0, 20.0, 21.0, 210.0, 213.0, 0.0, 1.0, 10.0, 13.0, 20.0, 21.0,
            210.0, 213.0, 0.0, 1.0, 10.0, 13.0, 20.0, 21.0, 210.0, 213.0,
        ]);

        let lines: Vec<Line> = iterator.collect();
        assert_eq!(
            lines,
            [
                Line(Point(0.0, 1.0), Point(10.0, 13.0)),
                Line(Point(20.0, 21.0), Point(210.0, 213.0)),
                Line(Point(0.0, 1.0), Point(10.0, 13.0)),
                Line(Point(20.0, 21.0), Point(210.0, 213.0)),
                Line(Point(0.0, 1.0), Point(10.0, 13.0)),
                Line(Point(20.0, 21.0), Point(210.0, 213.0)),
            ]
        );
    }
}
