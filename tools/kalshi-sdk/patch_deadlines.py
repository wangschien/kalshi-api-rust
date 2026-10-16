#!/usr/bin/env python3
"""Require caller deadlines and attach them to every generated HTTP request."""

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
    fields = fields.replace(client, "reqwest_middleware::ClientBuilder::new(reqwest::Client::builder().connect_timeout(connect_timeout).timeout(request_timeout).build()?).build()")
    prefix = text[:text.index("impl Configuration {")]
    prefix = prefix.replace("#[derive(Debug, Clone)]\npub struct Configuration {", """use std::time::Duration;

/// Every generated execution requires caller-supplied deadlines.
///
/// ```compile_fail
/// use kalshi_api_openapi::apis::configuration::Configuration;
/// let _ = Configuration::default();
/// ```
#[derive(Debug, Clone)]
pub struct Configuration {
    pub connect_timeout: Duration,
    pub request_timeout: Duration,""")
    path.write_text(prefix + f"""impl Configuration {{
    pub fn new(connect_timeout: Duration, request_timeout: Duration) -> Result<Self, reqwest::Error> {{
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
        path.write_text(text.replace(needle, "let req = req_builder.timeout(configuration.request_timeout).build()?;"))
    assert count > 0, "no generated execution sites found"
    print(f"caller deadlines applied to {count} generated operations")


if __name__ == "__main__":
    main()
