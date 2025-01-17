use async_trait::async_trait;
use tokio_stream::{StreamExt};
use mev_share_rs::{EventClient, sse::{Event}};
use crate::types::{Collector, CollectorStream};
use anyhow::Result;

/// A collector that streams from MEV-Share SSE endpoint
/// and generates [events](Event), which return tx hash, logs, and bundled txs.
pub struct MevShareCollector {
    mevshare_sse_url: String,
}

impl MevShareCollector {
    pub fn new(mevshare_sse_url: String) -> Self {
        Self { mevshare_sse_url }
    }
}

/// Implementation of the [Collector](Collector) trait for the
/// [MevShareCollector](MevShareCollector).
#[async_trait]
impl Collector<Event> for MevShareCollector
{
    async fn get_event_stream(&self) -> Result<CollectorStream<Event>> {
        let client = EventClient::default();
        let stream = client.subscribe(&self.mevshare_sse_url).await.unwrap();
        let stream = stream.filter_map(|event| match event {
            Ok(evt) => Some(evt),
            Err(_) => None
        });
        Ok(Box::pin(stream))
    }
}
