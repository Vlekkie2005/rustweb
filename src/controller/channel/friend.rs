use rocket::http::Status;
use rocket::serde::json::{json, Json};
use rocket::serde::json::Value::Null;
use rocket::State;
use crate::repositories::Repositories;
use crate::security::user_auth_guard::AuthenticatedUser;
use crate::services::response_service::{ApiResponse, ResponseService};
use crate::security::user_id_validator_quard::UserId;

#[rocket::post("/friends/<target_id>")]
pub async fn friend_request(
    user: AuthenticatedUser,
    target_id: UserId,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    if *user == *target_id {
        return ResponseService::error("You cannot send a friend request to yourself", None, 400);
    }

    if repos.user.find_by_id(*target_id).await.ok().flatten().is_none() {
        return ResponseService::error("User not found", None, 404);
    }

    let friend = match repos.friend.create(*user, *target_id).await {
        Ok(friend_req) => friend_req,
        Err(e) => {
            if let Some(db_error) = e.as_database_error() {
                if db_error.is_unique_violation() {
                    return ResponseService::error("Friend request already exists", None, 409)
                }
            }
            return ResponseService::error("Database error", None, 500)
        }
    };

    ResponseService::success(json!(friend), 201)
}

#[rocket::put("/friends/requests/<target_id>/accept")]
pub async fn friend_accept(
    user: AuthenticatedUser,
    target_id: UserId,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    if repos.user.find_by_id(*target_id).await.ok().flatten().is_none() {
        return ResponseService::error("User not found", None, 404);
    }

    let existing = match repos.friend.find(*user, *target_id).await {
        Ok(Some(f)) => f,
        Ok(None) => return ResponseService::error("Friend request not found", None, 404),
        Err(_) => return ResponseService::error("Database error", None, 500),
    };

    if existing.requested_by == *user {
        return ResponseService::error("Cannot accept your own friend request", None, 400);
    }

    match repos.friend.accept(*user, *target_id).await {
        Ok(Some(f)) => f,
        Ok(None) => return ResponseService::error("Friend request not found or already accepted", None, 404),
        Err(_) => return ResponseService::error("Database error", None, 500),
    };

    ResponseService::success(Null, 200)
}

#[rocket::post("/friends/<target_id>/block")]
pub async fn friend_block(
    user: AuthenticatedUser,
    target_id: UserId,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    if repos.user.find_by_id(*target_id).await.ok().flatten().is_none() {
        return ResponseService::error("User not found", None, 404);
    }

    if let Err(e) = repos.friend.block(*user, *target_id).await {
        if let Some(db_error) = e.as_database_error() {
            if db_error.is_unique_violation() {
                return ResponseService::error("Already blocked", None, 409);
            }
        }
        return ResponseService::error("Database error", None, 500);
    }

    ResponseService::success(Null, 200)
}

#[rocket::delete("/friends/<target_id>/unblock", rank = 2)]
pub async fn friend_unblock(
    user: AuthenticatedUser,
    target_id: UserId,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    if repos.user.find_by_id(*target_id).await.ok().flatten().is_none() {
        return ResponseService::error("User not found", None, 404);
    }

    if let Err(e) = repos.friend.unblock(*user, *target_id).await {
        if let Some(db_error) = e.as_database_error() {
            if db_error.is_unique_violation() {
                return ResponseService::error("Not blocked", None, 404);
            }
        }
        return ResponseService::error("Database error", None, 500);
    }

    ResponseService::success(Null, 200)
}

#[rocket::get("/friends")]
pub async fn friends(
    user: AuthenticatedUser,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    let friends = match repos.friend.find_friends(*user).await {
        Ok(f) => f,
        Err(_) => return ResponseService::error("Database error", None, 500),
    };

    ResponseService::success(json!(friends), 200)
}

#[rocket::get("/friends/requests")]
pub async fn get_friend_requests(
    user: AuthenticatedUser,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    let friends = match repos.friend.find_incoming(*user).await {
        Ok(f) => f,
        Err(_) => return ResponseService::error("Database error", None, 500),
    };

    ResponseService::success(json!(friends), 200)
}

#[rocket::get("/friends/requests/sent")]
pub async fn get_friend_pending_requests(
    user: AuthenticatedUser,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    let friends = match repos.friend.find_outgoing(*user).await {
        Ok(f) => f,
        Err(_) => return ResponseService::error("Database error", None, 500),
    };

    ResponseService::success(json!(friends), 200)
}

#[rocket::delete("/friends/requests/<target_id>", rank = 1)]
pub async fn delete_friend_pending_request (
    user: AuthenticatedUser,
    target_id: UserId,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    if repos.user.find_by_id(*target_id).await.ok().flatten().is_none() {
        return ResponseService::error("User not found", None, 404);
    }

    match repos.friend.find(*user, *target_id).await {
        Ok(Some(_)) => {},
        Ok(None) => return ResponseService::error("Friend not found", None, 404),
        Err(_) => return ResponseService::error("Database error", None, 500),
    }

    if let Err(_) = repos.friend.remove(*user, *target_id).await {
        return ResponseService::error("Database error", None, 500);
    }

    ResponseService::success(Null, 200)
}

#[rocket::delete("/friends/<target_id>")]
pub async fn friend_remove(
    user: AuthenticatedUser,
    target_id: UserId,
    repos: &State<Repositories>,
) -> (Status, Json<ApiResponse>) {
    if repos.user.find_by_id(*target_id).await.ok().flatten().is_none() {
        return ResponseService::error("User not found", None, 404);
    }

    match repos.friend.find(*user, *target_id).await {
        Ok(Some(_)) => {},
        Ok(None) => return ResponseService::error("Friend not found", None, 404),
        Err(_) => return ResponseService::error("Database error", None, 500),
    }

    if let Err(_) = repos.friend.remove(*user, *target_id).await {
        return ResponseService::error("Database error", None, 500);
    }

    ResponseService::success(Null, 200)
}