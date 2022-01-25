use serenity::{
    async_trait, http::Http,
    model::interactions::application_command::ApplicationCommandInteraction,
};
use songbird::{Event, EventContext, EventHandler, Songbird};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use crate::strings::IDLE_ALERT;

pub struct IdleHandler {
    pub http: Arc<Http>,
    pub manager: Arc<Songbird>,
    pub interaction: ApplicationCommandInteraction,
    pub limit: usize,
    pub count: Arc<AtomicUsize>,
}

#[async_trait]
impl EventHandler for IdleHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            if !track_list.is_empty() {
                self.count.store(0, Ordering::Relaxed);
                return None;
            }

            if self.count.fetch_add(1, Ordering::Relaxed) >= self.limit {
                let guild_id = self.interaction.guild_id?;

                if let Some(call) = self.manager.get(guild_id) {
                    let mut handler = call.lock().await;

                    if handler.leave().await.is_ok() {
                        self.interaction
                            .channel_id
                            .say(&self.http, IDLE_ALERT)
                            .await
                            .unwrap();
                    }
                }
            }
        }
        None
    }
}