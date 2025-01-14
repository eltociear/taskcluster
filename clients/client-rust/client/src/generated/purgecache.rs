#![allow(unused_imports)]
#![cfg_attr(rustfmt, rustfmt_skip)]
/* THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT */
use crate::{Client, ClientBuilder, Credentials, Retry};
use anyhow::Error;
use serde_json::Value;
use std::time::Duration;
use crate::util::urlencode;

/// Purge Cache Service
///
/// The purge-cache service is responsible for tracking cache-purge requests.
///
/// User create purge requests for specific caches on specific workers, and
/// these requests are timestamped.  Workers consult the service before
/// starting a new task, and purge any caches older than the timestamp.
pub struct PurgeCache {
    /// The underlying client used to make API calls for this service.
    pub client: Client
}

#[allow(non_snake_case)]
impl PurgeCache {
    /// Create a new PurgeCache instance, based on the given client builder
    pub fn new<CB: Into<ClientBuilder>>(client_builder: CB) -> Result<Self, Error> {
        Ok(Self{
            client: client_builder
                .into()
                .path_prefix("api/purge-cache/v1/")
                .build()?,
        })
    }

    /// Ping Server
    ///
    /// Respond without doing anything.
    /// This endpoint is used to check that the service is up.
    pub async fn ping(&self) -> Result<(), Error> {
        let method = "GET";
        let (path, query) = Self::ping_details();
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Generate an unsigned URL for the ping endpoint
    pub fn ping_url(&self) -> Result<String, Error> {
        let (path, query) = Self::ping_details();
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the ping endpoint
    pub fn ping_signed_url(&self, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::ping_details();
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for ping
    fn ping_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "ping";
        let query = None;

        (path, query)
    }

    /// Load Balancer Heartbeat
    ///
    /// Respond without doing anything.
    /// This endpoint is used to check that the service is up.
    pub async fn lbheartbeat(&self) -> Result<(), Error> {
        let method = "GET";
        let (path, query) = Self::lbheartbeat_details();
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Generate an unsigned URL for the lbheartbeat endpoint
    pub fn lbheartbeat_url(&self) -> Result<String, Error> {
        let (path, query) = Self::lbheartbeat_details();
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the lbheartbeat endpoint
    pub fn lbheartbeat_signed_url(&self, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::lbheartbeat_details();
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for lbheartbeat
    fn lbheartbeat_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "__lbheartbeat__";
        let query = None;

        (path, query)
    }

    /// Taskcluster Version
    ///
    /// Respond with the JSON version object.
    /// https://github.com/mozilla-services/Dockerflow/blob/main/docs/version_object.md
    pub async fn version(&self) -> Result<(), Error> {
        let method = "GET";
        let (path, query) = Self::version_details();
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Generate an unsigned URL for the version endpoint
    pub fn version_url(&self) -> Result<String, Error> {
        let (path, query) = Self::version_details();
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the version endpoint
    pub fn version_signed_url(&self, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::version_details();
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for version
    fn version_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "__version__";
        let query = None;

        (path, query)
    }

    /// Purge Worker Cache
    ///
    /// Publish a request to purge caches named `cacheName` with
    /// on `workerPoolId` workers.
    ///
    /// If such a request already exists, its `before` timestamp is updated to
    /// the current time.
    pub async fn purgeCache(&self, workerPoolId: &str, payload: &Value) -> Result<(), Error> {
        let method = "POST";
        let (path, query) = Self::purgeCache_details(workerPoolId);
        let body = Some(payload);
        let resp = self.client.request(method, &path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Determine the HTTP request details for purgeCache
    fn purgeCache_details<'a>(workerPoolId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("purge-cache/{}", urlencode(workerPoolId));
        let query = None;

        (path, query)
    }

    /// All Open Purge Requests
    ///
    /// View all active purge requests.
    ///
    /// This is useful mostly for administors to view
    /// the set of open purge requests. It should not
    /// be used by workers. They should use the purgeRequests
    /// endpoint that is specific to their workerType and
    /// provisionerId.
    pub async fn allPurgeRequests(&self, continuationToken: Option<&str>, limit: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::allPurgeRequests_details(continuationToken, limit);
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the allPurgeRequests endpoint
    pub fn allPurgeRequests_url(&self, continuationToken: Option<&str>, limit: Option<&str>) -> Result<String, Error> {
        let (path, query) = Self::allPurgeRequests_details(continuationToken, limit);
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the allPurgeRequests endpoint
    pub fn allPurgeRequests_signed_url(&self, continuationToken: Option<&str>, limit: Option<&str>, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::allPurgeRequests_details(continuationToken, limit);
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for allPurgeRequests
    fn allPurgeRequests_details<'a>(continuationToken: Option<&'a str>, limit: Option<&'a str>) -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "purge-cache/list";
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }

        (path, query)
    }

    /// Open Purge Requests for a worker pool
    ///
    /// List the caches for this `workerPoolId` that should to be
    /// purged if they are from before the time given in the response.
    ///
    /// This is intended to be used by workers to determine which caches to purge.
    pub async fn purgeRequests(&self, workerPoolId: &str, since: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::purgeRequests_details(workerPoolId, since);
        let body = None;
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the purgeRequests endpoint
    pub fn purgeRequests_url(&self, workerPoolId: &str, since: Option<&str>) -> Result<String, Error> {
        let (path, query) = Self::purgeRequests_details(workerPoolId, since);
        self.client.make_url(&path, query)
    }

    /// Generate a signed URL for the purgeRequests endpoint
    pub fn purgeRequests_signed_url(&self, workerPoolId: &str, since: Option<&str>, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::purgeRequests_details(workerPoolId, since);
        self.client.make_signed_url(&path, query, ttl)
    }

    /// Determine the HTTP request details for purgeRequests
    fn purgeRequests_details<'a>(workerPoolId: &'a str, since: Option<&'a str>) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("purge-cache/{}", urlencode(workerPoolId));
        let mut query = None;
        if let Some(q) = since {
            query.get_or_insert_with(Vec::new).push(("since", q));
        }

        (path, query)
    }
}
