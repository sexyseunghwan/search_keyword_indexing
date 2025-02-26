pub use std::{
    collections::{HashMap, VecDeque},
    env,
    fmt::Debug,
    fs::File,
    future::Future,
    io::{BufReader, Write},
    ops::Deref, //time::Duration,
    str::FromStr,
    sync::Arc,
};

pub use rand::{prelude::SliceRandom, rngs::StdRng, SeedableRng};

pub use tokio::{
    io::AsyncReadExt,
    net::TcpStream,
    signal,
    sync::{Mutex, MutexGuard, OnceCell},
    time::{Duration, Interval},
};

pub use futures_util::TryStreamExt;

pub use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub use log::{error, info, warn};

pub use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming, Record};

pub use chrono::{
    DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc,
};
pub use chrono_tz::Asia::Seoul;

pub use serde::{Deserialize, Serialize};

pub use serde::de::DeserializeOwned;

pub use serde_json::{json, Value};

pub use http::header::{HeaderMap, HeaderValue, CONTENT_TYPE};

pub use dotenv::dotenv;

pub use cron::Schedule;

pub use elasticsearch::{
    http::response::Response,
    http::transport::{ConnectionPool, Transport},
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    http::Url,
    indices::{
        IndicesCreateParts, IndicesDeleteParts, IndicesGetAliasParts, IndicesGetParts,
        IndicesRefreshParts,
    },
    BulkOperation, BulkParts, DeleteByQueryParts, DeleteParts, Elasticsearch, IndexParts,
    SearchParts,
};

pub use anyhow::{anyhow, Result};

pub use derive_new::new;
pub use getset::Getters;

pub use num_format::{Locale, ToFormattedString};

pub use tiberius::{
    time::DateTime as tiDatetime, AuthMethod, Client, Config, Query, QueryItem, Row, ToSql,
};

pub use async_trait::async_trait;

//use crate::repository::es_repository::*;
//use crate::repository::kafka_repository::*;
//pub static ELASTICSEARCH_CLIENT: OnceCell<Arc<EsRepositoryPub>> = OnceCell::new();
//pub static KAFKA_PRODUCER: OnceCell<Arc<KafkaRepositoryPub>> = OnceCell::const_new();

pub use regex::Regex;

pub use once_cell::sync::Lazy as once_lazy;
