use crate::buffers::*;
use crate::float::*;

pub struct SampleMut {
    pub buffer: Buffer<Stereo<f32>>,
}

impl SampleMut {
    pub fn from(buffer: Buffer<Stereo<f32>>) -> Self {
        Self { buffer }
    }




}



