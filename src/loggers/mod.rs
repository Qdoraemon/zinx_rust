use log::{debug, info, warn};
use std::sync::OnceLock;


pub static LOGGER: OnceLock<Loggers> = OnceLock::new();

pub struct Loggers {
    
}

impl Loggers {
    pub fn init() {
        env_logger::init();
    }
    pub fn new()-> &'static Loggers {

        LOGGER.get_or_init(|| 
            Loggers {}
        )
    }
    pub fn info(&self,message: &str){
        #[cfg(any ( feature ="release", feature = "test"))]
        {
            info!("{}",message);
        }
    }
    pub fn debug(&self,message: &str){
        #[cfg(feature = "test")]
        {
            debug!("{:?}",message);
        }
    }
    pub fn warn(&self,message: &str) {
        #[cfg(any ( feature ="release", feature = "test"))]
        {
            warn!("{}",message);
        }
    }

    
}