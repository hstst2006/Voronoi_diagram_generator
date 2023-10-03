#[derive(Debug, Copy, Clone)]
pub struct Coordinate
{
    pub x: i64,
    pub y: i64
}

impl Coordinate
{
    // Create a new coordinate pair from any two numbers implementing Into<i32>
    pub fn new(x: i64, y: i64) -> Coordinate
    {
        Coordinate 
        {
            x, 
            y
        }
    }

    // Calculates the manhattan distance between a pair of coordinates
    // Distance returned as f32 to be used with euclidean pair as delegate function in world.calculate_voronoi_values
    pub fn manhattan_pair(first: &Coordinate, second: &Coordinate) -> f32
    {
        let distance = (first.x - second.x).abs() + (first.y - second.y).abs();
        
        distance as f32
    }
    
    // Calculates the manhattan distance between this coordinate and another
    pub fn manhattan_to(&self, other: &Coordinate) -> f32
    {
        Coordinate::manhattan_pair(&self, &other)
    }

    // Calculates the euclidean distance between a pair of coordinates
    pub fn euclidean_pair(first: &Coordinate, second: &Coordinate) -> f32
    {
        let xs_squared = (second.x - first.x).pow(2);
        let ys_squared = (second.y - first.y).pow(2);

        let mut distance = (xs_squared + ys_squared) as f32;
        distance = distance.sqrt();

        distance
    }
    // Calculates the euclidean distance between this coordinate and another
    pub fn euclidean_to(&self, other: &Coordinate) ->  f32
    {
        Coordinate::euclidean_pair(&self, &other)
    }

}
