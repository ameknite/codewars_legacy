fn main() {}

struct Block {
    width: u32,
    length: u32,
    height: u32,
}

impl Block {
    fn new(arr: &[u32; 3]) -> Self {
        Self {
            width: arr[0],
            length: arr[1],
            height: arr[2],
        }
    }

    fn get_width(&self) -> u32 {
        self.width
    }
    fn get_length(&self) -> u32 {
        self.length
    }
    fn get_height(&self) -> u32 {
        self.height
    }
    fn get_volume(&self) -> u32 {
        self.width * self.length * self.height
    }
    fn get_surface_area(&self) -> u32 {
        (self.width * self.length * 2)
            + (self.width * self.height * 2)
            + (self.length * self.height * 2)
    }
}

#[cfg(test)]
mod tests {
    use super::Block;

    #[test]
    fn example_test() {
        let block = Block::new(&[2, 4, 6]);
        assert_eq!(
            block.get_width(),
            2,
            "Incorrect width\nExpected 2 but got {}",
            block.get_width()
        );
        assert_eq!(
            block.get_length(),
            4,
            "Incorrect length\nExpected 4 but got {}",
            block.get_length()
        );
        assert_eq!(
            block.get_height(),
            6,
            "Incorrect height\nExpected 6 but got {}",
            block.get_height()
        );
        assert_eq!(
            block.get_volume(),
            48,
            "Incorrect volume\nExpected 48 but got {}",
            block.get_volume()
        );
        assert_eq!(
            block.get_surface_area(),
            88,
            "Incorrect surface area\nExpected 88 but got {}",
            block.get_surface_area()
        );
    }
}
