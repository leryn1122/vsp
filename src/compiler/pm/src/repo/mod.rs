/// The client send two requests to the repository.
///   - GET /config.json
///   - GET /index.yaml
///
/// The `config.json` file contains the configuration for the repository.
///
/// ```restful
/// GET /config.json
/// ```
///
/// ```json
/// {
///   "site": "https://domain.com",
///   "dl": "/dl/",
///   "api": "/api/v1/artifacts"
/// }
/// ```
///
/// ```plaintext
/// GET /index.yaml
/// ```
///
/// ```yaml
/// apiVersion: "0.1.0"
/// entries:
///   - name: ""
/// ```
pub mod config;
pub mod index;
