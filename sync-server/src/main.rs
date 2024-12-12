// Importing necessary modules and types from standard libraries, tokio, and warp.
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, oneshot};
use warp::Filter;

// Defining a type alias for the shared state. The state holds a HashMap mapping
// unique identifiers (String) to oneshot senders that synchronize two parties.
type SyncState = Arc<RwLock<HashMap<String, oneshot::Sender<()>>>>;

// Asynchronous handler function that synchronizes two parties using a unique ID.
async fn wait_for_second_party_handler(
    id: String, // Unique identifier for the pair of parties.
    state: SyncState, // Shared state that holds the synchronization data.
) -> Result<impl warp::Reply, warp::Rejection> {
    // Create a one-shot channel for the synchronization process (this allows one party to signal the other).
    let (tx, rx) = oneshot::channel();

    {
        // Lock the state for writing to modify the synchronization data.
        let mut state_lock = state.write().await;

        // Check if there is already a sender waiting for a second party.
        if let Some(existing_tx) = state_lock.remove(&id) {
            // If a second party exists, signal the first party that the second party has arrived.
            let _ = existing_tx.send(()); 
            return Ok(warp::reply::with_status("Both parties synced.", warp::http::StatusCode::OK));
        }

        // If no second party exists, store the current party's sender in the state.
        state_lock.insert(id.clone(), tx);
    }

    // Wait for the second party to arrive or time out.
    match tokio::time::timeout(std::time::Duration::from_secs(10), rx).await {
        Ok(_) => Ok(warp::reply::with_status("Both parties synced.", warp::http::StatusCode::OK)),
        Err(_) => {
            // If the second party doesn't arrive in time, remove the timed-out entry from the state.
            let mut state_lock = state.write().await;
            state_lock.remove(&id);
            Err(warp::reject::custom(TimeoutError)) // Return a timeout error.
        }
    }
}

// Custom error type for handling timeouts.
#[derive(Debug)]
struct TimeoutError;

// Implementing the Reject trait for the custom TimeoutError to allow it to be returned as a warp rejection.
impl warp::reject::Reject for TimeoutError {}

// Main entry point for the application, defining the Warp server and routes.
#[tokio::main]
async fn main() {
    // Create a shared state that will be passed around (it holds the sync data).
    let state: SyncState = Arc::new(RwLock::new(HashMap::new()));

    // Clone the state for use in the route handler.
    let sync_state = warp::any().map(move || state.clone());

    // Define the route for waiting for a second party. It accepts a unique ID as a parameter.
    let wait_route = warp::path!("wait-for-second-party" / String)
        .and(sync_state) // Attach the state to the route.
        .and_then(wait_for_second_party_handler); // Link the handler to the route.

    // Start the server on localhost (127.0.0.1) on port 3030.
    warp::serve(wait_route).run(([127, 0, 0, 1], 3030)).await;
}

// Unit tests to verify different scenarios.
#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;
    use warp::Reply;

    // Test case where both parties sync successfully.
    #[tokio::test]
    async fn test_both_parties_sync_successfully() {
        let state: SyncState = Arc::new(RwLock::new(HashMap::new()));
        let id = Arc::new("test_id".to_string());

        // Simulate the first party calling the handler.
        let state_clone = state.clone();
        let id_clone = id.clone();
        let party_one = tokio::spawn(async move {
            wait_for_second_party_handler(id_clone.to_string(), state_clone).await
        });

        // Simulate the second party calling the handler.
        let state_clone = state.clone();
        let id_clone = id.clone();
        let party_two = tokio::spawn(async move {
            wait_for_second_party_handler(id_clone.to_string(), state_clone).await
        });

        // Await both parties and check that they both sync successfully.
        let res_one = party_one.await.unwrap().unwrap().into_response();
        let res_two = party_two.await.unwrap().unwrap().into_response();

        // Assert that both parties received a successful response.
        assert_eq!(res_one.status(), StatusCode::OK);
        assert_eq!(res_two.status(), StatusCode::OK);
    }

    // Test case where one party times out due to the absence of the second party.
    #[tokio::test]
    async fn test_timeout_for_single_party() {
        let state: SyncState = Arc::new(RwLock::new(HashMap::new()));
        let id = "test_timeout".to_string();

        // Call the handler for the first party, but there will be no second party.
        let result = wait_for_second_party_handler(id.clone(), state.clone()).await;

        // Check that the result is an error due to a timeout.
        assert!(result.is_err(), "Expected timeout error, but got success");
    }

    // Test case for edge cases where multiple requests with the same ID are made.
    #[tokio::test]
    async fn test_edge_case_multiple_requests_same_id() {
        let state: SyncState = Arc::new(RwLock::new(HashMap::new()));
        let id = Arc::new("duplicate_id".to_string());

        // Simulate three parties trying to sync with the same ID.
        let state_clone = state.clone();
        let id_clone = id.clone();
        let party_one = tokio::spawn(async move {
            wait_for_second_party_handler(id_clone.to_string(), state_clone).await
        });

        let state_clone = state.clone();
        let id_clone = id.clone();
        let party_two = tokio::spawn(async move {
            wait_for_second_party_handler(id_clone.to_string(), state_clone).await
        });

        let state_clone = state.clone();
        let id_clone = id.clone();
        let party_three = tokio::spawn(async move {
            wait_for_second_party_handler(id_clone.to_string(), state_clone).await
        });

        // Await the responses for all three parties.
        let res_one = party_one.await.unwrap().unwrap().into_response();
        let res_two = party_two.await.unwrap().unwrap().into_response();
        let res_three = party_three.await.unwrap();

        // Assert that the first two parties successfully sync, but the third party encounters an error.
        assert_eq!(res_one.status(), StatusCode::OK);
        assert_eq!(res_two.status(), StatusCode::OK);
        assert!(res_three.is_err());
    }
}
