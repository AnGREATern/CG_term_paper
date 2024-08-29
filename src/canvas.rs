pub struct Canvas {
    // frame: Vec<Vec<u8>>,
    frame: Vec<u8>,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32, color: u8) -> Self {
        // let frame = vec![vec![0; height as usize]; width as usize];
        let frame = vec![color; (4 * height * width) as usize];
        Self {
            frame,
            width,
            height,
        }
    }

    pub fn frame(&self) -> &[u8] {
        &self.frame.as_slice()
    }
}
