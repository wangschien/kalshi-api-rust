#!/usr/bin/env python3
"""Take optional caller deadlines and attach them to every generated HTTP request.

None sets no local deadline. The SDK has no default duration."""

import argparse
import re
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("crate", type=Path)
    args = parser.parse_args()
    path = args.crate / "src/apis/configuration.rs"
    text = path.read_text()
    defaults = re.search(r"impl Default for Configuration \{.*?Configuration \{(.*?)\n        \}", text, re.S)
    assert defaults, "expected fresh generator configuration"
    fields = defaults[1]
    client = "reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build()"
    assert fields.count(client) == 1
    fields = fields.replace(client, "reqwest_middleware::ClientBuilder::new(builder.build()?).build()")
    prefix = text[:text.index("impl Configuration {")]
    prefix = prefix.replace("#[derive(Debug, Clone)]\npub struct Configuration {", """use std::time::Duration;

/// The caller chooses both deadlines. `None` sets no local deadline; the
/// SDK has no default duration.
///
/// ```compile_fail
/// use kalshi_api_openapi::apis::configuration::Configuration;
/// let _ = Configuration::default();
/// ```
#[derive(Debug, Clone)]
pub struct Configuration {
    pub connect_timeout: Option<Duration>,
    pub request_timeout: Option<Duration>,""")
    path.write_text(prefix + f"""impl Configuration {{
    pub fn new(connect_timeout: Option<Duration>, request_timeout: Option<Duration>) -> Result<Self, reqwest::Error> {{
        let mut builder = reqwest::Client::builder();
        if let Some(timeout) = connect_timeout {{
            builder = builder.connect_timeout(timeout);
        }}
        if let Some(timeout) = request_timeout {{
            builder = builder.timeout(timeout);
        }}
        Ok(Configuration {{{fields}
            connect_timeout,
            request_timeout,
        }})
    }}
}}
""")
    count = 0
    for path in sorted((args.crate / "src/apis").glob("*_api.rs")):
        text = path.read_text()
        needle = "let req = req_builder.build()?;"
        count += text.count(needle)
        path.write_text(text.replace(needle, "if let Some(timeout) = configuration.request_timeout {\n        req_builder = req_builder.timeout(timeout);\n    }\n    let req = req_builder.build()?;"))
    assert count > 0, "no generated execution sites found"
    print(f"caller deadlines applied to {count} generated operations")


if __name__ == "__main__":
    main()
