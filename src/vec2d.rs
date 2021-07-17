use ::std::cmp::PartialEq;
use ::std::fmt;
use ::std::iter::IntoIterator;
use ::std::iter::Iterator;
use ::std::ops::Index;
use ::std::ops::IndexMut;

use crate::geom::Point;
use crate::geom::Rect;
use crate::geom::Size;

/// Holds the data for a Vec2D.
///
/// A Vec2D has a fixed size and cannot be resized.
#[derive(Clone)]
pub struct Vec2D<V: Copy> {
    /// The width and height of this Vec2D.
    width: usize,

    /// The raw data inside.
    /// It's size matches the size of this Vec2D.
    data: Vec<V>,
}

impl<V: Copy> Vec2D<V> {
    /// Creates a new Vec2D of the size given.
    ///
    /// It is filled with the default value.
    pub fn new(size: Size<usize>, default: V) -> Self {
        Vec2D {
            width: size.width(),
            data: vec![default; size.area()],
        }
    }

    /// In Debug mode this will panic! if it is given a vector with different shaped arrays.
    pub fn new_from_vecs(vec: Vec<Vec<V>>) -> Self {
        let width = vec.get(0).map(|v| v.len()).unwrap_or(0);

        #[cfg(debug_assertions)]
        {
            for row in &vec {
                assert_eq!(row.len(), width);
            }
        }

        Self {
            data: vec.into_iter().flatten().collect(),
            width,
        }
    }

    /// Returns the tile at the position given.
    pub fn get(&self, pos: Point<usize>) -> Option<&V> {
        let index = map_index(pos, self.width);

        self.data.get(index)
    }

    /// Sets a tile at the position given.
    pub fn set(&mut self, pos: Point<usize>, value: V) -> () {
        let index = map_index(pos, self.width);

        self.data[index] = value;
    }

    /// Returns the size of this Vec2D as a Rect.
    /// This is for convenience, as the bottom left corner is always Point(0, 0).
    pub fn rect(&self) -> Rect<usize> {
        Rect(Point(0, 0), self.size())
    }

    /// Returns the size of this Vec2D.
    pub fn size(&self) -> Size<usize> {
        Size(self.width(), self.height())
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.width
    }

    /// Returns a slice which encompasses the entire map.
    pub fn iter(&self) -> Vec2DIterator<V> {
        self.iter_of(self.rect())
    }

    /// Allows you to iterate over a sub section of this map.
    pub fn iter_of(&self, area: Rect<usize>) -> Vec2DIterator<V> {
        let data_rect = self.rect();
        let iterate_area = data_rect
            .intersect_rect(area)
            .unwrap_or(Rect(Point(0, 0), Size(0, 0)));

        Vec2DIterator {
            data: &self.data,
            data_width: self.width(),

            iterate_area: iterate_area,
            pos: area.bottom_left(),
        }
    }

    /// Returns the underlying raw data.
    pub fn raw_data<'a>(&'a self) -> &'a [V] {
        &self.data
    }

    fn row<'a>(&'a self, y: usize) -> Vec2DRow<'a, V> {
        Vec2DRow::new(self, y)
    }
}

impl<V: Copy> Index<Point<usize>> for Vec2D<V> {
    type Output = V;

    fn index(&self, pos: Point<usize>) -> &V {
        let index = map_index(pos, self.width);

        &self.data[index]
    }
}

impl<V: Copy> IndexMut<Point<usize>> for Vec2D<V> {
    fn index_mut(&mut self, pos: Point<usize>) -> &mut V {
        let index = map_index(pos, self.width);

        &mut self.data[index]
    }
}

impl<'a, V: Copy> IntoIterator for &'a Vec2D<V> {
    type Item = (V, Point<usize>);
    type IntoIter = Vec2DIterator<'a, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// @TODO, This should be an IterMut.
// impl<'a, V: Copy> IntoIterator for &'a mut Vec2D<V> {
//     type Item = (V, Point<usize>);
//     type IntoIter = Vec2DIterator<'a, V>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

/// An iterator for the `Vec2D`.
pub struct Vec2DIterator<'a, V: 'a> {
    /// The raw data we are iterating over.
    data: &'a Vec<V>,

    /// The size of the data when 2D.
    /// Needed for translating index.
    data_width: usize,

    /// The size of the area we are iterating over.
    iterate_area: Rect<usize>,

    /// Current index in the `Vec2D`.
    pos: Point<usize>,
}

impl<'a, V: Copy> Iterator for Vec2DIterator<'a, V> {
    type Item = (V, Point<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos.y() >= self.iterate_area.top_right().y() {
            return None;
        }

        let index = map_index(self.pos, self.data_width);
        let data = self.data[index];

        let result = Some((data, self.pos));

        // Increment across the x axis.
        if self.pos.x() < self.iterate_area.top_right().x() - 1 {
            self.pos.move_x(1);

        // We've wrapped over the x position.
        } else {
            self.pos.set_x(self.iterate_area.bottom_left().x());
            self.pos.move_y(1);
        }

        result
    }
}

impl<V: Copy + PartialEq> PartialEq for Vec2D<V> {
    fn eq(&self, other: &Self) -> bool {
        if self.width != other.width {
            return false;
        }

        return self.data == other.data;
    }
}

impl<V: Copy + fmt::Debug> fmt::Debug for Vec2D<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_list = f.debug_list();

        for y in 0..self.height() {
            debug_list.entry(&self.row(y));
        }

        debug_list.finish()
    }
}

struct Vec2DRow<'a, V: Copy + 'a> {
    /// The raw data we are iterating over.
    data: &'a Vec2D<V>,

    y: usize,
}

impl<'a, V: Copy> Vec2DRow<'a, V> {
    fn new(data: &'a Vec2D<V>, y: usize) -> Self {
        Self { data, y }
    }
}

impl<'a, V: Copy + fmt::Debug> fmt::Debug for Vec2DRow<'a, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for x in 0..self.data.width() {
            write!(f, "{:?}", self.data[Point(x, self.y)])?;

            if x < self.data.width() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")
    }
}

fn map_index(pos: Point<usize>, width: usize) -> usize {
    (pos.y() * width) + pos.x()
}

#[cfg(test)]
mod new {
    use super::*;

    #[test]
    fn it_should_create_vec2d_of_correct_size() {
        let pixels = Vec2D::new(Size(10, 20), 123);
        assert_eq!(pixels.size().area(), 200);
    }

    #[test]
    fn it_should_populate_vec2d_with_default_given() {
        let pixels = Vec2D::new(Size(2, 2), 123);

        assert_eq!(pixels.raw_data(), vec![123, 123, 123, 123,]);
    }
}

#[cfg(test)]
mod new_from_vecs {
    use super::*;

    #[test]
    fn it_should_create_an_empty_vec2d_from_empty_vecs() {
        #[rustfmt::skip]
        let vec2d : Vec2D<usize> = Vec2D::new_from_vecs(vec![]);
        let expected = Vec2D::new(Size(0, 0), 0);
        assert_eq!(vec2d, expected);
    }

    #[test]
    fn it_should_create_a_vec2d_from_elements_given() {
        #[rustfmt::skip]
        let vec2d : Vec2D<usize> = Vec2D::new_from_vecs(vec![
            vec![  0,  1,  2,  3,  4 ],
            vec![  5,  6,  7,  8,  9 ],
            vec![ 10, 11, 12, 13, 14 ],
            vec![ 15, 16, 17, 18, 19 ],
        ]);

        let mut expected = Vec2D::new(Size(5, 4), 0);
        let mut i = 0;
        for pos in expected.size() {
            expected[pos] = i;
            i += 1;
        }

        assert_eq!(vec2d, expected);
    }
}

#[cfg(test)]
mod indexing {
    use super::*;

    #[test]
    fn it_should_correctly_set_and_get_items_based_on_index() {
        let mut vec2d = Vec2D::new(Size(10, 10), 0);
        let index = Point(2, 3);

        assert_eq!(vec2d[index], 0);
        vec2d[index] = 1;
        assert_eq!(vec2d[index], 1);
    }

    #[test]
    fn it_should_correctly_index_items_across_whole_vec2d() {
        let vec2d_size = Size(10, 15);
        let mut vec2d = Vec2D::new(vec2d_size, Point(0, 0));

        for (val, _) in &vec2d {
            assert_eq!(val, Point(0, 0));
        }

        for x in 0..vec2d.width() {
            for y in 0..vec2d.height() {
                vec2d[Point(x, y)] = Point(x, y);
            }
        }

        for (val, pos) in &vec2d {
            assert_eq!(val, pos);
        }
    }
}

#[cfg(test)]
mod iterator {
    use super::*;

    #[test]
    fn it_should_iterate_over_all() {
        let vec2d_size = Size(10, 10);
        let vec2d = Vec2D::new(vec2d_size, 1);
        let mut count = 0;
        let mut pos_count_x = 0;

        for (n, pos) in &vec2d {
            count += n;
            pos_count_x += pos.x();
        }

        assert_eq!(count, vec2d_size.area());
        assert_eq!(pos_count_x, 10 * (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9));
    }

    #[test]
    fn it_should_iterate_over_larger_area() {
        let vec2d_size = Size(10, 10);
        let vec2d = Vec2D::new(vec2d_size, 1);
        let mut count = 0;
        let mut pos_count_x = 0;

        for (n, pos) in vec2d.iter_of(Rect(Point(0, 0), Size(20, 20))) {
            count += n;
            pos_count_x += pos.x();
        }

        assert_eq!(count, vec2d_size.area());
        assert_eq!(
            pos_count_x,
            vec2d_size.height() * (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9)
        );
    }

    #[test]
    fn it_should_iterate_over_partial_overlap() {
        let vec2d_size = Size(10, 10);
        let vec2d = Vec2D::new(vec2d_size, 1);
        let mut count = 0;
        let mut pos_count_x = 0;

        for (n, pos) in vec2d.iter_of(Rect(Point(5, 5), Size(20, 20))) {
            count += n;
            pos_count_x += pos.x();
        }

        assert_eq!(count, 5 * 5);
        assert_eq!(pos_count_x, 5 * (5 + 6 + 7 + 8 + 9));
    }

    #[test]
    fn it_should_set_and_then_iterate_over_all() {
        let vec2d_size = Size(10, 10);
        let mut vec2d = Vec2D::new(vec2d_size, 0);

        for x in 0..vec2d_size.width() {
            for y in 0..vec2d_size.height() {
                vec2d[Point(x, y)] = 1;
            }
        }

        let mut count = 0;
        let mut pos_count_x = 0;

        for (n, pos) in &vec2d {
            count += n;
            pos_count_x += pos.x();
        }

        assert_eq!(count, vec2d_size.area());
        assert_eq!(pos_count_x, 10 * (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9));
    }
}

#[cfg(test)]
mod debug {
    use super::*;

    #[test]
    fn it_should_print_debug_as_expected() {
        let mut vec2d = Vec2D::new(Size(5, 5), 0);

        for x in 0..vec2d.width() {
            for y in 0..vec2d.height() {
                vec2d[Point(x, y)] = x;
            }
        }

        let debug = format!("{:?}", vec2d);
        assert_eq!(
            debug,
            "[[0, 1, 2, 3, 4], [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], [0, 1, 2, 3, 4], [0, 1, 2, 3, 4]]"
        );
    }

    #[test]
    fn it_should_pretty_print_debug_as_expected() {
        let mut vec2d = Vec2D::new(Size(5, 5), 0);

        for x in 0..vec2d.width() {
            for y in 0..vec2d.height() {
                vec2d[Point(x, y)] = x;
            }
        }

        let debug = format!("{:#?}", vec2d);
        assert_eq!(
            debug,
            "[
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 4],
    [0, 1, 2, 3, 4],
]"
        );
    }
}
