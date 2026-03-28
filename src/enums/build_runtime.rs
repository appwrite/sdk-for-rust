use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum BuildRuntime {
    #[serde(rename = "node-14.5")]
    #[default]
    Node145,
    #[serde(rename = "node-16.0")]
    Node160,
    #[serde(rename = "node-18.0")]
    Node180,
    #[serde(rename = "node-19.0")]
    Node190,
    #[serde(rename = "node-20.0")]
    Node200,
    #[serde(rename = "node-21.0")]
    Node210,
    #[serde(rename = "node-22")]
    Node22,
    #[serde(rename = "node-23")]
    Node23,
    #[serde(rename = "node-24")]
    Node24,
    #[serde(rename = "node-25")]
    Node25,
    #[serde(rename = "php-8.0")]
    Php80,
    #[serde(rename = "php-8.1")]
    Php81,
    #[serde(rename = "php-8.2")]
    Php82,
    #[serde(rename = "php-8.3")]
    Php83,
    #[serde(rename = "php-8.4")]
    Php84,
    #[serde(rename = "ruby-3.0")]
    Ruby30,
    #[serde(rename = "ruby-3.1")]
    Ruby31,
    #[serde(rename = "ruby-3.2")]
    Ruby32,
    #[serde(rename = "ruby-3.3")]
    Ruby33,
    #[serde(rename = "ruby-3.4")]
    Ruby34,
    #[serde(rename = "ruby-4.0")]
    Ruby40,
    #[serde(rename = "python-3.8")]
    Python38,
    #[serde(rename = "python-3.9")]
    Python39,
    #[serde(rename = "python-3.10")]
    Python310,
    #[serde(rename = "python-3.11")]
    Python311,
    #[serde(rename = "python-3.12")]
    Python312,
    #[serde(rename = "python-3.13")]
    Python313,
    #[serde(rename = "python-3.14")]
    Python314,
    #[serde(rename = "python-ml-3.11")]
    PythonMl311,
    #[serde(rename = "python-ml-3.12")]
    PythonMl312,
    #[serde(rename = "python-ml-3.13")]
    PythonMl313,
    #[serde(rename = "deno-1.40")]
    Deno140,
    #[serde(rename = "deno-1.46")]
    Deno146,
    #[serde(rename = "deno-2.0")]
    Deno20,
    #[serde(rename = "deno-2.5")]
    Deno25,
    #[serde(rename = "deno-2.6")]
    Deno26,
    #[serde(rename = "dart-2.15")]
    Dart215,
    #[serde(rename = "dart-2.16")]
    Dart216,
    #[serde(rename = "dart-2.17")]
    Dart217,
    #[serde(rename = "dart-2.18")]
    Dart218,
    #[serde(rename = "dart-2.19")]
    Dart219,
    #[serde(rename = "dart-3.0")]
    Dart30,
    #[serde(rename = "dart-3.1")]
    Dart31,
    #[serde(rename = "dart-3.3")]
    Dart33,
    #[serde(rename = "dart-3.5")]
    Dart35,
    #[serde(rename = "dart-3.8")]
    Dart38,
    #[serde(rename = "dart-3.9")]
    Dart39,
    #[serde(rename = "dart-3.10")]
    Dart310,
    #[serde(rename = "dotnet-6.0")]
    Dotnet60,
    #[serde(rename = "dotnet-7.0")]
    Dotnet70,
    #[serde(rename = "dotnet-8.0")]
    Dotnet80,
    #[serde(rename = "dotnet-10")]
    Dotnet10,
    #[serde(rename = "java-8.0")]
    Java80,
    #[serde(rename = "java-11.0")]
    Java110,
    #[serde(rename = "java-17.0")]
    Java170,
    #[serde(rename = "java-18.0")]
    Java180,
    #[serde(rename = "java-21.0")]
    Java210,
    #[serde(rename = "java-22")]
    Java22,
    #[serde(rename = "java-25")]
    Java25,
    #[serde(rename = "swift-5.5")]
    Swift55,
    #[serde(rename = "swift-5.8")]
    Swift58,
    #[serde(rename = "swift-5.9")]
    Swift59,
    #[serde(rename = "swift-5.10")]
    Swift510,
    #[serde(rename = "swift-6.2")]
    Swift62,
    #[serde(rename = "kotlin-1.6")]
    Kotlin16,
    #[serde(rename = "kotlin-1.8")]
    Kotlin18,
    #[serde(rename = "kotlin-1.9")]
    Kotlin19,
    #[serde(rename = "kotlin-2.0")]
    Kotlin20,
    #[serde(rename = "kotlin-2.3")]
    Kotlin23,
    #[serde(rename = "cpp-17")]
    Cpp17,
    #[serde(rename = "cpp-20")]
    Cpp20,
    #[serde(rename = "bun-1.0")]
    Bun10,
    #[serde(rename = "bun-1.1")]
    Bun11,
    #[serde(rename = "bun-1.2")]
    Bun12,
    #[serde(rename = "bun-1.3")]
    Bun13,
    #[serde(rename = "go-1.23")]
    Go123,
    #[serde(rename = "go-1.24")]
    Go124,
    #[serde(rename = "go-1.25")]
    Go125,
    #[serde(rename = "go-1.26")]
    Go126,
    #[serde(rename = "static-1")]
    Static1,
    #[serde(rename = "flutter-3.24")]
    Flutter324,
    #[serde(rename = "flutter-3.27")]
    Flutter327,
    #[serde(rename = "flutter-3.29")]
    Flutter329,
    #[serde(rename = "flutter-3.32")]
    Flutter332,
    #[serde(rename = "flutter-3.35")]
    Flutter335,
    #[serde(rename = "flutter-3.38")]
    Flutter338,
}

impl BuildRuntime {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            BuildRuntime::Node145 => "node-14.5",
            BuildRuntime::Node160 => "node-16.0",
            BuildRuntime::Node180 => "node-18.0",
            BuildRuntime::Node190 => "node-19.0",
            BuildRuntime::Node200 => "node-20.0",
            BuildRuntime::Node210 => "node-21.0",
            BuildRuntime::Node22 => "node-22",
            BuildRuntime::Node23 => "node-23",
            BuildRuntime::Node24 => "node-24",
            BuildRuntime::Node25 => "node-25",
            BuildRuntime::Php80 => "php-8.0",
            BuildRuntime::Php81 => "php-8.1",
            BuildRuntime::Php82 => "php-8.2",
            BuildRuntime::Php83 => "php-8.3",
            BuildRuntime::Php84 => "php-8.4",
            BuildRuntime::Ruby30 => "ruby-3.0",
            BuildRuntime::Ruby31 => "ruby-3.1",
            BuildRuntime::Ruby32 => "ruby-3.2",
            BuildRuntime::Ruby33 => "ruby-3.3",
            BuildRuntime::Ruby34 => "ruby-3.4",
            BuildRuntime::Ruby40 => "ruby-4.0",
            BuildRuntime::Python38 => "python-3.8",
            BuildRuntime::Python39 => "python-3.9",
            BuildRuntime::Python310 => "python-3.10",
            BuildRuntime::Python311 => "python-3.11",
            BuildRuntime::Python312 => "python-3.12",
            BuildRuntime::Python313 => "python-3.13",
            BuildRuntime::Python314 => "python-3.14",
            BuildRuntime::PythonMl311 => "python-ml-3.11",
            BuildRuntime::PythonMl312 => "python-ml-3.12",
            BuildRuntime::PythonMl313 => "python-ml-3.13",
            BuildRuntime::Deno140 => "deno-1.40",
            BuildRuntime::Deno146 => "deno-1.46",
            BuildRuntime::Deno20 => "deno-2.0",
            BuildRuntime::Deno25 => "deno-2.5",
            BuildRuntime::Deno26 => "deno-2.6",
            BuildRuntime::Dart215 => "dart-2.15",
            BuildRuntime::Dart216 => "dart-2.16",
            BuildRuntime::Dart217 => "dart-2.17",
            BuildRuntime::Dart218 => "dart-2.18",
            BuildRuntime::Dart219 => "dart-2.19",
            BuildRuntime::Dart30 => "dart-3.0",
            BuildRuntime::Dart31 => "dart-3.1",
            BuildRuntime::Dart33 => "dart-3.3",
            BuildRuntime::Dart35 => "dart-3.5",
            BuildRuntime::Dart38 => "dart-3.8",
            BuildRuntime::Dart39 => "dart-3.9",
            BuildRuntime::Dart310 => "dart-3.10",
            BuildRuntime::Dotnet60 => "dotnet-6.0",
            BuildRuntime::Dotnet70 => "dotnet-7.0",
            BuildRuntime::Dotnet80 => "dotnet-8.0",
            BuildRuntime::Dotnet10 => "dotnet-10",
            BuildRuntime::Java80 => "java-8.0",
            BuildRuntime::Java110 => "java-11.0",
            BuildRuntime::Java170 => "java-17.0",
            BuildRuntime::Java180 => "java-18.0",
            BuildRuntime::Java210 => "java-21.0",
            BuildRuntime::Java22 => "java-22",
            BuildRuntime::Java25 => "java-25",
            BuildRuntime::Swift55 => "swift-5.5",
            BuildRuntime::Swift58 => "swift-5.8",
            BuildRuntime::Swift59 => "swift-5.9",
            BuildRuntime::Swift510 => "swift-5.10",
            BuildRuntime::Swift62 => "swift-6.2",
            BuildRuntime::Kotlin16 => "kotlin-1.6",
            BuildRuntime::Kotlin18 => "kotlin-1.8",
            BuildRuntime::Kotlin19 => "kotlin-1.9",
            BuildRuntime::Kotlin20 => "kotlin-2.0",
            BuildRuntime::Kotlin23 => "kotlin-2.3",
            BuildRuntime::Cpp17 => "cpp-17",
            BuildRuntime::Cpp20 => "cpp-20",
            BuildRuntime::Bun10 => "bun-1.0",
            BuildRuntime::Bun11 => "bun-1.1",
            BuildRuntime::Bun12 => "bun-1.2",
            BuildRuntime::Bun13 => "bun-1.3",
            BuildRuntime::Go123 => "go-1.23",
            BuildRuntime::Go124 => "go-1.24",
            BuildRuntime::Go125 => "go-1.25",
            BuildRuntime::Go126 => "go-1.26",
            BuildRuntime::Static1 => "static-1",
            BuildRuntime::Flutter324 => "flutter-3.24",
            BuildRuntime::Flutter327 => "flutter-3.27",
            BuildRuntime::Flutter329 => "flutter-3.29",
            BuildRuntime::Flutter332 => "flutter-3.32",
            BuildRuntime::Flutter335 => "flutter-3.35",
            BuildRuntime::Flutter338 => "flutter-3.38",
        }
    }
}

impl std::fmt::Display for BuildRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
