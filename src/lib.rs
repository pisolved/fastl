use std::{fmt, io::BufRead, ops::Add};

#[derive(Debug, PartialEq)]
struct StlVector<T: Add<Output = T> + Copy>(T, T, T);

impl<T: Add<Output = T> + Copy> StlVector<T> {
    fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }

    fn sum(&self) -> T {
        self.0 + self.1 + self.2
    }
}

#[derive(Debug, PartialEq)]
struct StlTriangle {
    x: StlVector<f32>,
    y: StlVector<f32>,
    z: StlVector<f32>,
}

impl StlTriangle {
    // Heron's formula (looked up)
    // There were other methods. Some claimed to be faster, but this was the
    // most approachable with my level of familiarity
    fn area(&self) -> f32 {
        let a = StlVector::new((self.x.0 - self.y.0).powf(2.0),
                               (self.x.1 - self.y.1).powf(2.0),
                               (self.x.2 - self.y.2).powf(2.0)).sum()
                                                               .sqrt();
        let b = StlVector::new((self.x.0 - self.z.0).powf(2.0),
                               (self.x.1 - self.z.1).powf(2.0),
                               (self.x.2 - self.z.2).powf(2.0)).sum()
                                                               .sqrt();
        let c = StlVector::new((self.y.0 - self.z.0).powf(2.0),
                               (self.y.1 - self.z.1).powf(2.0),
                               (self.y.2 - self.z.2).powf(2.0)).sum()
                                                               .sqrt();

        let perimeter = a + b + c;
        let s = perimeter * 0.5;

        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }
}

impl From<Vec<String>> for StlTriangle {
    fn from(vert_strings: Vec<String>) -> Self {
        let vertices = vec![vert_strings[0].trim()
                                           .split(' ')
                                           .skip(1)
                                           .map(|s| s.parse::<f32>().unwrap())
                                           .collect::<Vec<f32>>(),
                            vert_strings[1].trim()
                                           .split(' ')
                                           .skip(1)
                                           .map(|s| s.parse::<f32>().unwrap())
                                           .collect::<Vec<f32>>(),
                            vert_strings[2].trim()
                                           .split(' ')
                                           .skip(1)
                                           .map(|s| s.parse::<f32>().unwrap())
                                           .collect::<Vec<f32>>(),];

        Self { x: StlVector(vertices[0][0], vertices[0][1], vertices[0][2]),
               y: StlVector(vertices[1][0], vertices[1][1], vertices[1][2]),
               z: StlVector(vertices[2][0], vertices[2][1], vertices[2][2]) }
    }
}

#[derive(Debug)]
pub struct StlStats {
    area: f32,
    triangles: u32,
}

impl StlStats {
    fn new() -> Self {
        Self { area: 0.0,
               triangles: 0 }
    }
}

impl fmt::Display for StlStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "Number of Triangles: {}\nSurface Area: {:.4}",
               self.triangles, self.area)
    }
}

pub fn process_stl<R: BufRead>(inf: R) -> StlStats {
    let mut lines = inf.lines();
    // Per implementation notes, we'll assume the file is properly formed
    // and no triangles are overlapping.
    // We'll throw away the first line, and due to no requirements we'll
    // ignore the facet normal (should be perpindicular to the triangle face?)

    let mut stats = StlStats::new();

    // This seems ripe for optimization. Throwing the work of calculating area
    // into a thread pool and using channels might speed things up significantly
    // on larger files.

    while let Some(line) = lines.next() {
        if let Ok(line) = line {
            if line.trim().starts_with("facet") {
                let vertices = lines.by_ref()
                                    .skip(1)
                                    .take(3)
                                    .map(|r| r.ok().unwrap())
                                    .collect::<Vec<String>>();
                stats.area += StlTriangle::from(vertices).area();
                stats.triangles += 1;
            }
        }
    }
    stats
}

#[cfg(test)]
mod triangles {
    use crate::*;
    fn simple_input() -> &'static str {
        "solid simplePart
  facet normal 0 0 0
      outer loop
          vertex 0 0 0
          vertex 1 0 0
          vertex 1 1 1
      endloop
  endfacet
  facet normal 0 0 0
      outer loop
          vertex 0 0 0
          vertex 0 1 1
          vertex 1 1 1
      endloop
  endfacet
endsolid simplePart"
    }

    #[test]
    fn test_from() {
        let input = vec!["vertex 0 0 0".into(),
                         "vertex 1 0 0".into(),
                         " vertex 1 1 1".into()];
        let actual = StlTriangle::from(input);
        let expected = StlTriangle { x: StlVector(0., 0., 0.),
                                     y: StlVector(1., 0., 0.),
                                     z: StlVector(1., 1., 1.) };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_area() {
        let input = StlTriangle { x: StlVector(0., 0., 0.),
                                  y: StlVector(1., 0., 0.),
                                  z: StlVector(1., 1., 1.) };
        let expected = 0.7071;

        assert_eq!(f32::round(input.area() * 10000.0) * 0.0001, expected);
    }

    #[test]
    fn test_stats() {
        let input = simple_input();
        let actual = process_stl(input.as_bytes());
        let expected_area = 1.4142;
        let expected_triangles = 2;

        assert_eq!(f32::round(actual.area * 10000.0) * 0.0001, expected_area);
        assert_eq!(actual.triangles, expected_triangles)
    }
}
