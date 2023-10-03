mod classes;
use classes::coordinates::Coordinate;
use classes::world::World;

fn main() {

    // Create a 20x20 world
    let mut world = World::new(20,20);

    /* EXAMPLE 1
    // Add some voronoi cells to the world, one at each corner and one in the middle      
    world.add_cell(Coordinate::new(0,0));
    world.add_cell(Coordinate::new(0,19));
    world.add_cell(Coordinate::new(19, 0));
    world.add_cell(Coordinate::new(19, 19));
    world.add_cell(Coordinate::new(9, 9));
    */
    

    /* EXAMPLE 2
    // One voronoi cell at the top left corner and three places at "random" locations
    world.add_cell(Coordinate::new(0,0));
    world.add_cell(Coordinate::new(4, 11));
    world.add_cell(Coordinate::new(9, 9));
    world.add_cell(Coordinate::new(8, 0));
    */
    
    // Calculate voronoi values using Coordinate::{euclidean_pair, manhattan_pair} distance functions
    world.calculate_voronoi_values(&Coordinate::euclidean_pair);

    // Draw the voronoi world. The number represents which voronoi cell is closest. Voronoi cell centers are not drawn
    world.draw_voronoi_world();

}
