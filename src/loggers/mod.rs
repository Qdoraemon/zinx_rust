use log::{debug, error, info, log_enabled, warn, Level};
use std::{sync::OnceLock};
// use lazy_static::lazy_static;
// lazy_static! {
/* 
pub static mut  LOGGER :Option< Loggers> = None;
// }

pub fn logger() -> &'static mut Loggers{
    match unsafe {LOGGER} {
        Some(ref logger) => logger,
        None => {
            panic!("logger is not initialized");
    }
        
    }
}
*/

pub static LOGGER: OnceLock<Loggers> = OnceLock::new();

pub struct Loggers {
    
}

impl Loggers {
    pub fn init() {
        env_logger::init();
    }
    pub fn new()-> &'static Loggers {

        // env_logger::init();

        // unsafe {
            // LOGGER = Some(Loggers {});
        // }
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