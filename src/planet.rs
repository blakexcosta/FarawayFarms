#[derive(Debug)]
pub enum Biome{
    Desert,
    Tropical,
    Water,
    Mountain,
    Temperate,
}

#[derive(Debug)]
pub struct Planet {
    pub name: String,
    pub biome: Biome,
    pub size_x: u32,
    pub size_y: u32,
}
impl Planet {
    pub fn new(_name: String, _biome: Biome, _size_x: u32, _size_y: u32) -> Planet{
        return Planet{ 
            name: _name,
            biome: _biome,
            size_x: _size_x,
            size_y: _size_y
        };
    }
}