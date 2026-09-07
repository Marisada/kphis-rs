[tokio-cron-scheduler](https://github.com/mvniekerk/tokio-cron-scheduler)

> last check date = 2026-09-07

## tokio-cron-scheduler/src/job_scheduler.rs
change info! to debug!
: 11
```diff
    use crate::store::{MetaDataStorage, NotificationStore};
-   use chrono::{DateTime, NaiveDateTime, Utc};
+   use chrono::{DateTime, Utc};
    use std::future::Future;
    use std::pin::Pin;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    #[cfg(all(unix, feature = "signal"))]
    use tokio::signal::unix::SignalKind;
    use tokio::sync::RwLock;
-   use tracing::{error, info};
+   use tracing::{debug, error};
    use uuid::Uuid;
```

:270
```diff
    pub async fn add(&self, job: JobLocked) -> Result<Uuid, JobSchedulerError> {
        let guid = job.guid();
        if !self.inited().await {
-           info!("Uninited");
+           debug!("Job scheduler uninited");
            let mut s = self.clone();
            s.init().await?;
        }

        let context = self.context.clone();
        JobCreator::add(&context, job).await?;
-       info!("Job creator created");
+       debug!("Job creator created");

        Ok(guid)
    }
```

remove depreciated `NaiveDateTime::from_timestamp_opt`
: 37
```diff
            if vv.next_tick == 0 {
                    return None;
                }
-               match NaiveDateTime::from_timestamp_opt(vv.next_tick as i64, 0) {
-                   None => None,
-                   Some(ts) => Some(DateTime::from_naive_utc_and_offset(ts, Utc)),
-               }
+               DateTime::from_timestamp(vv.next_tick as i64, 0)
            } else {
                None
            }
```

## tokio-cron-scheduler/src/store/metadata_store.rs
remove deadcode

:2
```diff
-   use crate::JobSchedulerError;
    use crate::job::JobToRunAsync;
    #[cfg(not(feature = "has_bytes"))]
    use crate::job::job_data::{JobAndNextTick, JobStoredData};
    #[cfg(feature = "has_bytes")]
    use crate::job::job_data_prost::{JobAndNextTick, JobStoredData};
-   use crate::store::{CodeGet, DataStore, InitStore};
+   use crate::store::{DataStore, InitStore};
    use chrono::{DateTime, Utc};
    use std::future::Future;
    use std::pin::Pin;
```

:27
```diff

-   pub trait JobCodeGet: CodeGet<Box<JobToRunAsync>> {}
```

## tokio-cron-scheduler/src/store/notification_store.rs
remove deadcode

:6
```diff
    use crate::job::job_data_prost::{JobState, NotificationData};
    use crate::job::{JobId, NotificationId};
-   use crate::store::{CodeGet, DataStore, InitStore};
-   use crate::{JobSchedulerError, OnJobNotification};
+   use crate::store::{DataStore, InitStore};
+   use crate::JobSchedulerError;
    use std::future::Future;
    use std::pin::Pin;
    use uuid::Uuid;
```

:35
```diff

-   pub trait NotificationRunnableCodeGet: CodeGet<Box<OnJobNotification>> {}
```
