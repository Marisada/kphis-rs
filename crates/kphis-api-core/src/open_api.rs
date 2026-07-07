use kphis_util::error::AppError;

const BAD_REQUEST_EXAMPLE: &str = "{\
    \"action\": \"MakeTea\",\
    \"error_id\": \"01ARZ3NDEKTSV4RRFFQ69G5FAV\",\
    \"message\": \"Bad Request\",\
    \"source\": \"App\"\
}";

pub const UNAUTHORIZED_EXAMPLE: &str = "{\
    \"action\": \"MakeTea\",\
    \"error_id\": \"01ARZ3NDEKTSV4RRFFQ69G5FAV\",\
    \"message\": \"Unauthorized\",\
    \"source\": \"App\"\
}";

const FORBIDDEN_EXAMPLE: &str = "{\
    \"action\": \"MakeTea\",\
    \"error_id\": \"01ARZ3NDEKTSV4RRFFQ69G5FAV\",\
    \"message\": \"Forbidden\",\
    \"source\": \"App\"\
}";

const NOT_FOUND_EXAMPLE: &str = "{\
\"action\": \"MakeTea\",\
\"error_id\": \"01ARZ3NDEKTSV4RRFFQ69G5FAV\",\
\"message\": \"Not Found\",\
\"source\": \"App\"\
}";

#[allow(dead_code)]
#[derive(utoipa::ToSchema)]
#[schema(value_type = String, format = Binary)]
pub struct Binary(Vec<u8>);

/// 200, 400`<AppError>`, 401`<AppError>`, 500`<AppError>`
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
pub enum DocBytes {
    #[response(status = 200, description = "OK", content_type = "application/octet-stream")]
    Ok(#[to_schema] Binary),

    #[response(status = 400, description = "Bad Request", example = json!(BAD_REQUEST_EXAMPLE))]
    BadRequest(#[to_schema] AppError),

    #[response(status = 401, description = "Unauthorized", example = json!(UNAUTHORIZED_EXAMPLE))]
    Unauthorized(#[to_schema] AppError),

    #[response(status = 403, description = "Forbidden", example = json!(FORBIDDEN_EXAMPLE))]
    Forbidden(#[to_schema] AppError),

    #[response(status = 404, description = "Not Found", example = json!(NOT_FOUND_EXAMPLE))]
    NotFound(#[to_schema] AppError),

    #[response(status = 500, description = "Internal Server error")]
    ServerError(#[to_schema] AppError),
}

/// 200, 400`<AppError>`, 401`<AppError>`, 500`<AppError>`
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
pub enum DocImage {
    #[response(status = 200, description = "OK", content_type = "image/*")]
    Ok(#[to_schema] Binary),

    #[response(status = 400, description = "Bad Request", example = json!(BAD_REQUEST_EXAMPLE))]
    BadRequest(#[to_schema] AppError),

    #[response(status = 401, description = "Unauthorized", example = json!(UNAUTHORIZED_EXAMPLE))]
    Unauthorized(#[to_schema] AppError),

    #[response(status = 403, description = "Forbidden", example = json!(FORBIDDEN_EXAMPLE))]
    Forbidden(#[to_schema] AppError),

    #[response(status = 404, description = "Not Found", example = json!(NOT_FOUND_EXAMPLE))]
    NotFound(#[to_schema] AppError),

    #[response(status = 500, description = "Internal Server error")]
    ServerError(#[to_schema] AppError),
}

/// 200, 400`<AppError>`, 401`<AppError>`, 500`<AppError>`
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
pub enum DocPdf {
    #[response(status = 200, description = "OK", content_type = "application/pdf")]
    Ok(#[to_schema] Binary),

    #[response(status = 400, description = "Bad Request", example = json!(BAD_REQUEST_EXAMPLE))]
    BadRequest(#[to_schema] AppError),

    #[response(status = 401, description = "Unauthorized", example = json!(UNAUTHORIZED_EXAMPLE))]
    Unauthorized(#[to_schema] AppError),

    #[response(status = 403, description = "Forbidden", example = json!(FORBIDDEN_EXAMPLE))]
    Forbidden(#[to_schema] AppError),

    #[response(status = 404, description = "Not Found", example = json!(NOT_FOUND_EXAMPLE))]
    NotFound(#[to_schema] AppError),

    #[response(status = 500, description = "Internal Server error")]
    ServerError(#[to_schema] AppError),
}

/// 200`<T>`, 401`<AppError>`, 500`<AppError>`
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
pub enum DocOne<T>
where
    T: utoipa::ToSchema,
{
    #[response(status = 200, description = "OK")]
    Ok(T),

    #[response(status = 400, description = "Bad Request", example = json!(BAD_REQUEST_EXAMPLE))]
    BadRequest(#[to_schema] AppError),

    #[response(status = 401, description = "Unauthorized", example = json!(UNAUTHORIZED_EXAMPLE))]
    Unauthorized(#[to_schema] AppError),

    #[response(status = 403, description = "Forbidden", example = json!(FORBIDDEN_EXAMPLE))]
    Forbidden(#[to_schema] AppError),

    #[response(status = 404, description = "Not Found", example = json!(NOT_FOUND_EXAMPLE))]
    NotFound(#[to_schema] AppError),

    #[response(status = 500, description = "Internal Server error")]
    ServerError(#[to_schema] AppError),
}

/// 200`<Vec<T>>`, 401`<AppError>`, 500`<AppError>`
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
pub enum DocVec<T>
where
    T: utoipa::ToSchema,
{
    #[response(status = 200, description = "OK")]
    Ok([T; 1]),

    #[response(status = 400, description = "Bad Request", example = json!(BAD_REQUEST_EXAMPLE))]
    BadRequest(#[to_schema] AppError),

    #[response(status = 401, description = "Unauthorized", example = json!(UNAUTHORIZED_EXAMPLE))]
    Unauthorized(#[to_schema] AppError),

    #[response(status = 403, description = "Forbidden", example = json!(FORBIDDEN_EXAMPLE))]
    Forbidden(#[to_schema] AppError),

    #[response(status = 404, description = "Not Found", example = json!(NOT_FOUND_EXAMPLE))]
    NotFound(#[to_schema] AppError),

    #[response(status = 500, description = "Internal Server error")]
    ServerError(#[to_schema] AppError),
}

/// 200`<(u32, Vec<T>)>`, 401`<AppError>`, 500`<AppError>`
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
pub enum DocVecU32<T>
where
    T: utoipa::ToSchema,
{
    #[response(status = 200, description = "OK")]
    Ok((u32, [T; 1])),

    #[response(status = 400, description = "Bad Request", example = json!(BAD_REQUEST_EXAMPLE))]
    BadRequest(#[to_schema] AppError),

    #[response(status = 401, description = "Unauthorized", example = json!(UNAUTHORIZED_EXAMPLE))]
    Unauthorized(#[to_schema] AppError),

    #[response(status = 403, description = "Forbidden", example = json!(FORBIDDEN_EXAMPLE))]
    Forbidden(#[to_schema] AppError),

    #[response(status = 404, description = "Not Found", example = json!(NOT_FOUND_EXAMPLE))]
    NotFound(#[to_schema] AppError),

    #[response(status = 500, description = "Internal Server error")]
    ServerError(#[to_schema] AppError),
}

/// 200`<Option<T>>`, 401`<AppError>`, 500`<AppError>`
#[allow(dead_code)]
#[derive(utoipa::IntoResponses)]
pub enum DocOpt<T>
where
    T: utoipa::ToSchema,
{
    #[response(status = 200, description = "OK")]
    Ok(Option<T>),

    #[response(status = 400, description = "Bad Request", example = json!(BAD_REQUEST_EXAMPLE))]
    BadRequest(#[to_schema] AppError),

    #[response(status = 401, description = "Unauthorized", example = json!(UNAUTHORIZED_EXAMPLE))]
    Unauthorized(#[to_schema] AppError),

    #[response(status = 403, description = "Forbidden", example = json!(FORBIDDEN_EXAMPLE))]
    Forbidden(#[to_schema] AppError),

    #[response(status = 404, description = "Not Found", example = json!(NOT_FOUND_EXAMPLE))]
    NotFound(#[to_schema] AppError),

    #[response(status = 500, description = "Internal Server error")]
    ServerError(#[to_schema] AppError),
}
