use crate::Coordinate;

#[derive(Debug)]
pub struct World
{
    coordinates: Vec<Coordinate>,
    cells: Vec<Coordinate>,
    pub voronoi_values: Vec<i64>,
    width: i64,
    //height: i64
}

impl World
{
    pub fn new(_width: i64, _height: i64) -> World
    {
        let width: i64 = _width.into();
        let height: i64 = _height.into();

        let mut coordinates = Vec::new();
        let cells = Vec::new();
        let voronoi_values = Vec::new();

        for y in 0..height
        {
            for x in 0..width
            {
                let coordinate = Coordinate::new(x, y);
                coordinates.push(coordinate);
            }
        }
        World
        {
            coordinates,
            cells,
            voronoi_values,
            width,
            //height
        }
    }

    // Adds a voronoi cell to the world
    // Voronoi cells cannot overlap, so we look for duplicate cells before pushing it to the vector
    // TODO: Implement Ord for coordinates, so we can sort the vector and call dedup
    pub fn add_cell(&mut self, coordinate: Coordinate) 
    {
        self.cells.push(coordinate);
    }

    // Use the index of the cell to signify ownership of cell
    pub fn calculate_voronoi_values(&mut self, distance_function: &dyn Fn(&Coordinate, &Coordinate) -> f32)
    {

        if self.cells.len() < 1 { return ;}

        for coordinate in &self.coordinates
        {
            let mut closest_cell = 0;
            let mut shortest_distance = f32::MAX;

            for i in 0..self.cells.len()
            {
                let distance = distance_function(coordinate, &self.cells[i]);
                if distance <= shortest_distance
                {
                    closest_cell = i;
                    shortest_distance = distance;
                }
            }

            self.voronoi_values.push(closest_cell as i64);

        }
    }

    pub fn draw_voronoi_world(&self)
    {
        for i in 0..self.voronoi_values.len()
        {
            if i as i64 % self.width == 0
            {
                println!("");
            }
            let voronoi_value = self.voronoi_values[i];
            print!("{:^3}", voronoi_value);
        }
    }

}