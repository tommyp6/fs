use actix_session::Session;
use actix_web::Result;
use serde::{Deserialize, Serialize};

const FLASH_KEY: &'static str = "_flash";

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum FlashKind {
    OK,
    INFO,
    ERROR,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FlashMessage {
    kind: FlashKind,
    msg: String,
    is_safe: bool,
}

#[allow(dead_code)]
impl FlashMessage {
    pub fn new<S: Into<String>>(kind: FlashKind, msg: S, is_safe: bool) -> Self {
        Self {
            kind,
            msg: msg.into(),
            is_safe: is_safe,
        }
    }

    pub fn ok<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: FlashKind::OK,
            msg: msg.into(),
            is_safe: false,
        }
    }

    pub fn ok_safe<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: FlashKind::OK,
            msg: msg.into(),
            is_safe: true,
        }
    }

    pub fn info<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: FlashKind::INFO,
            msg: msg.into(),
            is_safe: false,
        }
    }

    pub fn info_safe<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: FlashKind::INFO,
            msg: msg.into(),
            is_safe: true,
        }
    }

    pub fn error<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: FlashKind::ERROR,
            msg: msg.into(),
            is_safe: false,
        }
    }

    pub fn error_safe<S: Into<String>>(msg: S) -> Self {
        Self {
            kind: FlashKind::ERROR,
            msg: msg.into(),
            is_safe: true,
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
