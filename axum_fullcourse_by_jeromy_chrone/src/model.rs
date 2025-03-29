// Simplistic Model layer

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct TicketForCreate {
    pub title: String,
}

#[derive(Clone)]
pub struct ModelController {
    ticket_stores: Arc<Mutex<Vec<Option<Ticket>>>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(ModelController {
            ticket_stores: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn create_ticket(&self, tc: TicketForCreate) -> Result<Ticket> {
        let mut store = self.ticket_stores.lock().unwrap();

        let id = store.len() as u64;

        let ticket = Ticket {
            id,
            title: tc.title,
        };

        store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_stores.lock().unwrap();

        let tickets = store.iter().filter_map(|t| t.clone()).collect();

        Ok(tickets)
    }

    pub async fn delete_ticket(&self) -> Result<()> {
        todo!();
        // TODO: stop following the tutorial here since it became a bit confusing, would continue later maybe

        Ok(())
    }
}
