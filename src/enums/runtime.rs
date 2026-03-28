use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Runtime {
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

impl Runtime {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Runtime::Node145 => "node-14.5",
            Runtime::Node160 => "node-16.0",
            Runtime::Node180 => "node-18.0",
            Runtime::Node190 => "node-19.0",
            Runtime::Node200 => "node-20.0",
            Runtime::Node210 => "node-21.0",
            Runtime::Node22 => "node-22",
            Runtime::Node23 => "node-23",
            Runtime::Node24 => "node-24",
            Runtime::Node25 => "node-25",
            Runtime::Php80 => "php-8.0",
            Runtime::Php81 => "php-8.1",
            Runtime::Php82 => "php-8.2",
            Runtime::Php83 => "php-8.3",
            Runtime::Php84 => "php-8.4",
            Runtime::Ruby30 => "ruby-3.0",
            Runtime::Ruby31 => "ruby-3.1",
            Runtime::Ruby32 => "ruby-3.2",
            Runtime::Ruby33 => "ruby-3.3",
            Runtime::Ruby34 => "ruby-3.4",
            Runtime::Ruby40 => "ruby-4.0",
            Runtime::Python38 => "python-3.8",
            Runtime::Python39 => "python-3.9",
            Runtime::Python310 => "python-3.10",
            Runtime::Python311 => "python-3.11",
            Runtime::Python312 => "python-3.12",
            Runtime::Python313 => "python-3.13",
            Runtime::Python314 => "python-3.14",
            Runtime::PythonMl311 => "python-ml-3.11",
            Runtime::PythonMl312 => "python-ml-3.12",
            Runtime::PythonMl313 => "python-ml-3.13",
            Runtime::Deno140 => "deno-1.40",
            Runtime::Deno146 => "deno-1.46",
            Runtime::Deno20 => "deno-2.0",
            Runtime::Deno25 => "deno-2.5",
            Runtime::Deno26 => "deno-2.6",
            Runtime::Dart215 => "dart-2.15",
            Runtime::Dart216 => "dart-2.16",
            Runtime::Dart217 => "dart-2.17",
            Runtime::Dart218 => "dart-2.18",
            Runtime::Dart219 => "dart-2.19",
            Runtime::Dart30 => "dart-3.0",
            Runtime::Dart31 => "dart-3.1",
            Runtime::Dart33 => "dart-3.3",
            Runtime::Dart35 => "dart-3.5",
            Runtime::Dart38 => "dart-3.8",
            Runtime::Dart39 => "dart-3.9",
            Runtime::Dart310 => "dart-3.10",
            Runtime::Dotnet60 => "dotnet-6.0",
            Runtime::Dotnet70 => "dotnet-7.0",
            Runtime::Dotnet80 => "dotnet-8.0",
            Runtime::Dotnet10 => "dotnet-10",
            Runtime::Java80 => "java-8.0",
            Runtime::Java110 => "java-11.0",
            Runtime::Java170 => "java-17.0",
            Runtime::Java180 => "java-18.0",
            Runtime::Java210 => "java-21.0",
            Runtime::Java22 => "java-22",
            Runtime::Java25 => "java-25",
            Runtime::Swift55 => "swift-5.5",
            Runtime::Swift58 => "swift-5.8",
            Runtime::Swift59 => "swift-5.9",
            Runtime::Swift510 => "swift-5.10",
            Runtime::Swift62 => "swift-6.2",
            Runtime::Kotlin16 => "kotlin-1.6",
            Runtime::Kotlin18 => "kotlin-1.8",
            Runtime::Kotlin19 => "kotlin-1.9",
            Runtime::Kotlin20 => "kotlin-2.0",
            Runtime::Kotlin23 => "kotlin-2.3",
            Runtime::Cpp17 => "cpp-17",
            Runtime::Cpp20 => "cpp-20",
            Runtime::Bun10 => "bun-1.0",
            Runtime::Bun11 => "bun-1.1",
            Runtime::Bun12 => "bun-1.2",
            Runtime::Bun13 => "bun-1.3",
            Runtime::Go123 => "go-1.23",
            Runtime::Go124 => "go-1.24",
            Runtime::Go125 => "go-1.25",
            Runtime::Go126 => "go-1.26",
            Runtime::Static1 => "static-1",
            Runtime::Flutter324 => "flutter-3.24",
            Runtime::Flutter327 => "flutter-3.27",
            Runtime::Flutter329 => "flutter-3.29",
            Runtime::Flutter332 => "flutter-3.32",
            Runtime::Flutter335 => "flutter-3.35",
            Runtime::Flutter338 => "flutter-3.38",
        }
    }
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
