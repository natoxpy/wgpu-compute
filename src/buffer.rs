#[derive(Debug, Clone)]
pub struct ComputeBuffer {
    pub data: Vec<u8>,
}

impl ComputeBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn to_be_bytes(&self) -> Vec<u32> {
        self.data
            .chunks(4)
            .map(|a| u32::from_be_bytes(a.try_into().unwrap()))
            .collect::<Vec<u32>>()
    }

    pub fn to_le_bytes(&self) -> Vec<u32> {
        self.data
            .chunks(4)
            .map(|a| u32::from_le_bytes(a.try_into().unwrap()))
            .collect::<Vec<u32>>()
    }
}

impl From<ComputeBuffer> for Vec<u32> {
    fn from(value: ComputeBuffer) -> Vec<u32> {
        value
            .data
            .chunks(4)
            .map(|a| u32::from_le_bytes(a.try_into().unwrap()))
            .collect::<Vec<u32>>()
    }
}

impl From<ComputeBuffer> for Vec<f32> {
    fn from(value: ComputeBuffer) -> Vec<f32> {
        value
            .data
            .chunks(4)
            .map(|a| f32::from_le_bytes(a.try_into().unwrap()))
            .collect::<Vec<f32>>()
    }
}

impl From<Vec<f32>> for ComputeBuffer {
    fn from(value: Vec<f32>) -> ComputeBuffer {
        let data = value
            .iter()
            .map(|v| v.to_le_bytes())
            .collect::<Vec<[u8; 4]>>()
            .iter()
            .cloned()
            .flatten()
            .collect::<Vec<u8>>();

        ComputeBuffer { data }
    }
}

impl From<Vec<u32>> for ComputeBuffer {
    fn from(value: Vec<u32>) -> ComputeBuffer {
        let data = value
            .iter()
            .map(|v| v.to_le_bytes())
            .collect::<Vec<[u8; 4]>>()
            .iter()
            .cloned()
            .flatten()
            .collect::<Vec<u8>>();

        ComputeBuffer { data }
    }
}
