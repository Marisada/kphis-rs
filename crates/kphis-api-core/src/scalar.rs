use utoipa::openapi::OpenApi;

pub fn html(openapi: OpenApi) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
  <head>
    <title>KPHIS API Document</title>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
  </head>
  <body>
    <script id="api-reference" type="application/json">
      {spec}
    </script>
    <script>
      var configuration = {{
        customCss: `* {{ font-family: "Comic Sans MS", cursive, sans-serif; }}`,
        favicon: 'statics/icons/favicon-16x16.png',
        theme: 'bluePlanet',
      }}
      document.getElementById('api-reference').dataset.configuration = JSON.stringify(configuration)
    </script>
    <script src="https://cdn.jsdelivr.net/npm/@scalar/api-reference@1.57.5"></script>
  </body>
</html>"#,
        spec = serde_json::to_string(&openapi).expect("Invalid OpenAPI spec, expected OpenApi, String, &str or serde_json::Value",)
    )
}
