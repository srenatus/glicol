use crate::{Buffer, Input, Node, BoxedNodeSend, NodeData, Message, impl_to_boxed_nodedata};

#[derive(Debug, Copy, Clone)]
pub struct SquOsc {
    pub freq: f32,
    pub phase: f32,
    pub sr: usize,
    inc: f32,
}

impl std::default::Default for SquOsc {
    fn default() -> Self {
        Self {
            freq: 1.0,
            phase: 0.0,
            sr: 44100,
            inc: 0.,
        }
    }
}

impl SquOsc {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn freq(self, freq: f32) -> Self {
        Self {
            freq, ..self
        }
    }
    pub fn sr(self, sr: usize) -> Self {
        Self {
            sr, ..self
        }
    }
    pub fn phase(self, phase: f32) -> Self {
        Self {
            phase, ..self
        }
    }
    impl_to_boxed_nodedata!();
}

impl<const N: usize> Node<N> for SquOsc {
    fn process(&mut self, inputs: &[Input<N>], output: &mut [Buffer<N>]) {
        match inputs.len() {
            0 => {
                for i in 0..N {
                    if self.phase <= 0.5 {
                        output[0][i] = 1.0;
                    } else {
                        output[0][i] = -1.0;
                    }

                    self.phase += self.freq / self.sr as f32;
                    if self.phase > 1. {
                        self.phase -= 1.
                    }
                }
            },
            1 => {
                    let mod_buf = &mut inputs[0].buffers();
                    for i in 0..N {
                        if mod_buf[0][i] != 0. {
                            self.inc = mod_buf[0][i]
                        };
                        if self.phase <= 0.5 {
                            output[0][i] = 1.0;
                        } else {
                            output[0][i] = -1.0;
                        }

                        self.phase += self.inc / self.sr as f32;
                        if self.phase > 1. {
                            self.phase -= 1.
                        }
                    }
            }
            _ => return ()
        }
    }
    fn send_msg(&mut self, info: Message) {

        match info {
            Message::SetToNumber(v) => {
                match v.0 {
                    0 => {self.freq = v.1},
                    _ => {}
                }
            }
            _ => {}
        }
    }
}