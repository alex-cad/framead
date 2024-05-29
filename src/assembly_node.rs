use approx::relative_ne;
use nalgebra::Vector3;

// 螺纹代号
#[derive(PartialEq)]
pub enum ScrewType {
    M4,
    M5,
    M6,
    M8,
    M10,
    M12,
    M14,
}

// 内螺纹面
struct InnerScrewFace {
    screw_type: ScrewType,
    start: Vector3<f32>,
    end: Vector3<f32>,
}

impl InnerScrewFace {
    pub fn new(screw_type: ScrewType, start: Vector3<f32>, end: Vector3<f32>) -> Self {
        let is_close = relative_ne!(start, end);
        if !is_close {
            panic!("start and end should not be the same point");
        }
        InnerScrewFace {
            screw_type,
            start,
            end,
        }
    }

    pub fn is_match(&self, outer_screw_face: OuterScrewFace) -> bool {
        self.screw_type == outer_screw_face.screw_type
    }
}

// 外螺纹面
struct OuterScrewFace {
    screw_type: ScrewType,
    start: Vector3<f32>,
    end: Vector3<f32>,
}

impl OuterScrewFace {
    pub fn new(screw_type: ScrewType, start: Vector3<f32>, end: Vector3<f32>) -> Self {
        let is_close = relative_ne!(start, end);
        if !is_close {
            panic!("start and end should not be the same point");
        }
        OuterScrewFace {
            screw_type,
            start,
            end,
        }
    }

    pub fn is_match(&self, inner_screw_face: InnerScrewFace) -> bool {
        self.screw_type == inner_screw_face.screw_type
    }
}

// 弧形面
struct ArcFace {
    radius: u32, // 0.001mm
    start: Vector3<f32>,
    end: Vector3<f32>,
}

impl ArcFace {
    pub fn new(radius: u32, start: Vector3<f32>, end: Vector3<f32>) -> Self {
        let is_close = relative_ne!(start, end);
        if !is_close {
            panic!("start and end should not be the same point");
        }
        ArcFace { radius, start, end }
    }

    pub fn is_match(&self, pole_face: PoleFace) -> bool {
        self.radius == pole_face.radius
    }
}

// 杆面
struct PoleFace {
    radius: u32, // 0.001mm
    start: Vector3<f32>,
    end: Vector3<f32>,
}

impl PoleFace {
    pub fn new(radius: u32, start: Vector3<f32>, end: Vector3<f32>) -> Self {
        let is_close = relative_ne!(start, end);
        if !is_close {
            panic!("start and end should not be the same point");
        }
        PoleFace { radius, start, end }
    }

    pub fn is_match(&self, arc_face: ArcFace) -> bool {
        self.radius == arc_face.radius
    }
}

// 平面
struct PlaneFace {
    normal: Vector3<f32>,
    length: f32,
}

impl PlaneFace {
    pub fn new(normal: Vector3<f32>, length: f32) -> Self {
        PlaneFace { normal, length }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn aa_test() {
        let start = Vector3::new(0.0, 0.0, 0.0);
        let end = Vector3::new(1.0, 0.0, 0.0);
        let isf = InnerScrewFace::new(ScrewType::M8, start, end);
        let osf = OuterScrewFace::new(ScrewType::M8, start, end);
    }
}
