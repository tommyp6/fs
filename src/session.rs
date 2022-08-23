use actix_session::Session;
use actix_web::Result;
use serde::{Deserialize, Serialize};

const FLASH_KEY: &'static str = "_flash";

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub enum FlashKind {
    OK,
    INFO,
    ERROR,
}

#[derive(Serialize, Deserialize)]
pub struct FlashMessage {
    kind: FlashKind,
    msg: String,
}

impl FlashMessage {
    pub fn new<S: Into<String>>(kind: FlashKind, msg: S) -> Self {
        Self {
            kind,
            msg: msg.into(),
        }
    }
}

pub fn flash(session: &Session, msg: FlashMessage) -> Result<()> {
    if let Some(mut flash_messages) = session.get::<Vec<FlashMessage>>(FLASH_KEY)? {
        flash_messages.push(msg);
        session.insert(FLASH_KEY, flash_messages)?;
    } else {
        session.insert(FLASH_KEY, vec![msg])?;
    }

    Ok(())
}

pub fn get_flash_messages(session: &Session) -> Result<Vec<FlashMessage>> {
    if let Some(flash_messages) = session.get::<Vec<FlashMessage>>(FLASH_KEY)? {
        session.remove(FLASH_KEY);
        Ok(flash_messages)
    } else {
        Ok(vec![])
    }
}
