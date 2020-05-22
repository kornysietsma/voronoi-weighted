#[macro_use]
extern crate approx;

mod conflict_list;
mod convex_hull;
mod face;
mod horizon_edge;
mod vector;
mod vertex;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
