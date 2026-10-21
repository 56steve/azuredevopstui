use notify_rust::Notification;

const OPEN_ACTION: &str = "open";

/// Shows a desktop notification, with an open action when there is a `url`, and waits for it to close.
/// Returns `url` when the open action was clicked.
pub async fn send(summary: String, body: String, url: Option<String>) -> Option<String> {
    let has_url = url.is_some();
    let result = tokio::task::spawn_blocking(move || {
        let mut notification = Notification::new();
        notification
            .appname("azuredevopstui")
            .summary(&summary)
            .body(&body);
        if has_url {
            notification.action(OPEN_ACTION, "Open");
        }
        let handle = notification.show()?;
        let mut open = false;
        handle.wait_for_action(|action| open = action == OPEN_ACTION);
        Ok::<_, notify_rust::error::Error>(open)
    })
    .await;
    match result {
        Ok(Ok(open)) => url.filter(|_| open),
        Ok(Err(err)) => {
            tracing::warn!("failed to show notification: {err}");
            None
        }
        Err(err) => {
            tracing::warn!("notification task failed: {err}");
            None
        }
    }
}
