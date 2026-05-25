pub struct Transform {
    pub position: [f32; 2],
    pub absolute_dimensions: [f32; 2],
}

impl Transform {
    pub fn scale_to_fit(&mut self, target_w: f32, target_h: f32) {
        let [w, h] = self.absolute_dimensions;
        let [w_ratio, h_ratio] = [target_w / w, target_h / h];
        let scale = w_ratio.min(h_ratio);
        let new_absolute_dimensions = [w * scale, h * scale];
        self.absolute_dimensions = new_absolute_dimensions;
    }
}
