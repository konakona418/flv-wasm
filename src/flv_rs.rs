use std::collections::VecDeque;
use std::error::Error;
use flv_rs_single::exchange::RemuxedData;
use flv_rs_single::flv::decoder::Decoder;

pub struct FlvRs {
    pub decoder: Decoder,
}

impl FlvRs {
    pub fn new() -> Self {
        FlvRs {
            decoder: Decoder::new(VecDeque::new()),
        }
    }

    pub fn push_data(&mut self, data: &mut VecDeque<u8>) -> Result<(), Box<dyn Error>> {
        self.decoder.push_data(data);
        Ok(())
    }

    pub fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.decoder.start()?;
        self.decoder.run()
    }

    pub fn continue_decoding(&mut self) -> Result<(), Box<dyn Error>> {
        self.decoder.continue_decoding()
    }

    pub fn get_codec_conf(&mut self) -> Result<(String, String), Box<dyn Error>> {
        self.decoder.get_codec_conf()
    }

    pub fn consume(&mut self) -> Result<RemuxedData, Box<dyn Error>> {
        self.decoder.consume()
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn Error>> {
        self.decoder.stop()
    }
}