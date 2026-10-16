#!/usr/bin/env python3
"""Expose typed, metadata-bearing, and unread responses for every REST operation."""

import argparse
import re
from pathlib import Path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("crate", type=Path)
    args = parser.parse_args()
    count = 0
    for path in sorted((args.crate / "src/apis").glob("*_api.rs")):
        parts = re.split(r"(?=pub async fn )", path.read_text())
        output = [parts[0]]
        for part in parts[1:]:
            signature = re.match(r"pub async fn (\w+)\((.*?)\)\s*->\s*Result<(.+?), Error<(\w+)>>\s*\{", part, re.S)
            assert signature, path
            name, parameters, value_type, error_type = signature.groups()
            assert not name.endswith(("_with_http_info", "_without_preload_content")), "response patch expects fresh generator output"
            last = part.rfind("\n}")
            assert last >= 0
            body = part[signature.end():last]
            marker = "    let resp = configuration.client.execute(req).await?;"
            assert body.count(marker) == 1, name
            building, handling = body.split(marker)
            handling = handling.replace("    let status = resp.status();", "")
            handling = re.sub(r"resp\s*\.headers\(\)", "headers", handling)
            handling = handling.replace("resp.text().await?", "content.clone()")
            handling = handling.replace("!status.is_client_error() && !status.is_server_error()", "status.is_success()")
            arguments = "configuration, params" if re.search(r"\bparams:", parameters) else "configuration"
            output.append(f"""pub async fn {name}({parameters}) -> Result<{value_type}, Error<{error_type}>> {{
    {name}_with_http_info({arguments}).await.map(|response| response.data)
}}

/// Returns the decoded response together with its status, headers, and original body.
pub async fn {name}_with_http_info({parameters}) -> Result<super::ApiResponse<{value_type}>, Error<{error_type}>> {{
    let resp = {name}_without_preload_content({arguments}).await?;
    let status = resp.status();
    let headers = resp.headers().clone();
    let raw_data = resp.bytes().await?.to_vec();
    let content = String::from_utf8_lossy(&raw_data).into_owned();
    let data: Result<{value_type}, Error<{error_type}>> = {{{handling}
    }};
    data.map(|data| super::ApiResponse {{ status, headers, data, raw_data }})
}}

/// Returns the HTTP response without reading its body or interpreting its status.
pub async fn {name}_without_preload_content({parameters}) -> Result<reqwest::Response, Error<{error_type}>> {{{building}
    configuration.client.execute(req).await.map_err(Error::from)
}}""" + part[last + 2:])
            count += 1
        path.write_text("".join(output))
    module = args.crate / "src/apis/mod.rs"
    text = module.read_text()
    assert "pub struct ApiResponse<" not in text
    text += """
/// A decoded API response with its HTTP metadata and original body bytes.
#[derive(Debug, Clone)]
pub struct ApiResponse<T> {
    pub status: reqwest::StatusCode,
    pub headers: reqwest::header::HeaderMap,
    pub data: T,
    pub raw_data: Vec<u8>,
}
"""
    module.write_text(text)
    print(f"response variants added for {count} operations")


if __name__ == "__main__":
    main()
