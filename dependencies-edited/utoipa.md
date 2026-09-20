[utiopa](https://github.com/juhaku/utoipa)
- Added Time
- Check scalar at `https://www.jsdelivr.com/package/npm/@scalar/api-reference` = v1.69.2
> edit version at /crates/kphis-api-core/src/scalar.rs:24
> last check date = 2026-09-19

## remove rust-toolchain.toml

## utoipa-gen/src/schema_type.rs
:116 edit is_primitive()
```diff
            #[cfg(feature = "time")]
            if !primitive {
                primitive = matches!(
                    name,
-                   "Date" | "PrimitiveDateTime" | "OffsetDateTime" | "Duration"
+                   "Date" | "Time" | "PrimitiveDateTime" | "OffsetDateTime" | "Duration"
                );
            }
```

:289 edit to_token()
```diff
            #[cfg(feature = "time")]
-           "PrimitiveDateTime" | "OffsetDateTime" => {
+           "Time" | "PrimitiveDateTime" | "OffsetDateTime" => {
                schema_type_tokens(tokens, SchemaTypeInner::String, self.nullable)
            }
```

:325 add in pub enum KnownFormat
```diff
    Binary,
    Date,
+   Time,
    DateTime,
    Password,
```

:397 add in fn from_path()
```diff
            #[cfg(feature = "chrono")]
            "NaiveDate" => Self::Date,

+           #[cfg(feature = "chrono")]
+           "NaiveTime" => Self::Time,
```

:421 add in fn from_path()
```diff
+           #[cfg(feature = "time")]
+           "Time" => Self::Time,

            #[cfg(feature = "time")]
            "PrimitiveDateTime" | "OffsetDateTime" => Self::DateTime,
```

:448 add in fn get_allowed_formats()
```diff
            "Binary",
            "Date",
+           "Time",
            "DateTime",
```

:528 add in fn parse() of impl Parse for KnownFormat
```diff
                "Binary" => Ok(Self::Binary),
                "Date" => Ok(Self::Date),
+               "Time" => Ok(Self::Time),
                "DateTime" => Ok(Self::DateTime),
```

:608 add to to_tokens()
```diff
            Self::Date => tokens.extend(quote!(utoipa::openapi::schema::SchemaFormat::KnownFormat(
                utoipa::openapi::schema::KnownFormat::Date
            ))),
+           Self::Time => tokens.extend(quote!(utoipa::openapi::schema::SchemaFormat::KnownFormat(
+               utoipa::openapi::schema::KnownFormat::Time
+           ))),
            Self::DateTime => tokens.extend(quote!(utoipa::openapi::schema::SchemaFormat::KnownFormat(
                utoipa::openapi::schema::KnownFormat::DateTime
            ))),
```

:763 add in fn new() of impl PrimitiveType
```diff
            #[cfg(feature = "url")]
            "Url" => {
                syn::parse_quote!(String)
            }

            #[cfg(feature = "time")]
-            "PrimitiveDateTime" | "OffsetDateTime" => {
+            "Time" | "PrimitiveDateTime" | "OffsetDateTime" => {
                syn::parse_quote!(String)
            }
```
