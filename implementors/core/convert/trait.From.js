(function() {var implementors = {};
implementors["github_rest"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/reqwest/0.11.9/reqwest/error/struct.Error.html\" title=\"struct reqwest::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"github_rest/enum.GithubRestError.html\" title=\"enum github_rest::GithubRestError\">GithubRestError</a>","synthetic":false,"types":["github_rest::GithubRestError"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/serde_json/1.0.79/serde_json/error/struct.Error.html\" title=\"struct serde_json::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"github_rest/enum.GithubRestError.html\" title=\"enum github_rest::GithubRestError\">GithubRestError</a>","synthetic":false,"types":["github_rest::GithubRestError"]}];
implementors["octocat_rs"] = [{"text":"impl&lt;T, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;A&gt; for <a class=\"struct\" href=\"octocat_rs/github/command/struct.Command.html\" title=\"struct octocat_rs::github::command::Command\">Command</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = T&gt; + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + 'static,<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":false,"types":["octocat_rs::github::command::Command"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()