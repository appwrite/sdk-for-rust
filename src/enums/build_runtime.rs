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
    #[serde(rename = "node-14.5-rc")]
    Node145Rc,
    #[serde(rename = "node-16.0-rc")]
    Node160Rc,
    #[serde(rename = "node-18.0-rc")]
    Node180Rc,
    #[serde(rename = "node-19.0-rc")]
    Node190Rc,
    #[serde(rename = "node-20.0-rc")]
    Node200Rc,
    #[serde(rename = "node-21.0-rc")]
    Node210Rc,
    #[serde(rename = "node-22-rc")]
    Node22Rc,
    #[serde(rename = "node-23-rc")]
    Node23Rc,
    #[serde(rename = "node-24-rc")]
    Node24Rc,
    #[serde(rename = "node-25-rc")]
    Node25Rc,
    #[serde(rename = "php-8.0-rc")]
    Php80Rc,
    #[serde(rename = "php-8.1-rc")]
    Php81Rc,
    #[serde(rename = "php-8.2-rc")]
    Php82Rc,
    #[serde(rename = "php-8.3-rc")]
    Php83Rc,
    #[serde(rename = "php-8.4-rc")]
    Php84Rc,
    #[serde(rename = "ruby-3.0-rc")]
    Ruby30Rc,
    #[serde(rename = "ruby-3.1-rc")]
    Ruby31Rc,
    #[serde(rename = "ruby-3.2-rc")]
    Ruby32Rc,
    #[serde(rename = "ruby-3.3-rc")]
    Ruby33Rc,
    #[serde(rename = "ruby-3.4-rc")]
    Ruby34Rc,
    #[serde(rename = "ruby-4.0-rc")]
    Ruby40Rc,
    #[serde(rename = "python-3.8-rc")]
    Python38Rc,
    #[serde(rename = "python-3.9-rc")]
    Python39Rc,
    #[serde(rename = "python-3.10-rc")]
    Python310Rc,
    #[serde(rename = "python-3.11-rc")]
    Python311Rc,
    #[serde(rename = "python-3.12-rc")]
    Python312Rc,
    #[serde(rename = "python-3.13-rc")]
    Python313Rc,
    #[serde(rename = "python-3.14-rc")]
    Python314Rc,
    #[serde(rename = "python-ml-3.11-rc")]
    PythonMl311Rc,
    #[serde(rename = "python-ml-3.12-rc")]
    PythonMl312Rc,
    #[serde(rename = "python-ml-3.13-rc")]
    PythonMl313Rc,
    #[serde(rename = "deno-1.40-rc")]
    Deno140Rc,
    #[serde(rename = "deno-1.46-rc")]
    Deno146Rc,
    #[serde(rename = "deno-2.0-rc")]
    Deno20Rc,
    #[serde(rename = "deno-2.5-rc")]
    Deno25Rc,
    #[serde(rename = "deno-2.6-rc")]
    Deno26Rc,
    #[serde(rename = "dart-2.15-rc")]
    Dart215Rc,
    #[serde(rename = "dart-2.16-rc")]
    Dart216Rc,
    #[serde(rename = "dart-2.17-rc")]
    Dart217Rc,
    #[serde(rename = "dart-2.18-rc")]
    Dart218Rc,
    #[serde(rename = "dart-2.19-rc")]
    Dart219Rc,
    #[serde(rename = "dart-3.0-rc")]
    Dart30Rc,
    #[serde(rename = "dart-3.1-rc")]
    Dart31Rc,
    #[serde(rename = "dart-3.3-rc")]
    Dart33Rc,
    #[serde(rename = "dart-3.5-rc")]
    Dart35Rc,
    #[serde(rename = "dart-3.8-rc")]
    Dart38Rc,
    #[serde(rename = "dart-3.9-rc")]
    Dart39Rc,
    #[serde(rename = "dart-3.10-rc")]
    Dart310Rc,
    #[serde(rename = "dotnet-6.0-rc")]
    Dotnet60Rc,
    #[serde(rename = "dotnet-7.0-rc")]
    Dotnet70Rc,
    #[serde(rename = "dotnet-8.0-rc")]
    Dotnet80Rc,
    #[serde(rename = "dotnet-10-rc")]
    Dotnet10Rc,
    #[serde(rename = "java-8.0-rc")]
    Java80Rc,
    #[serde(rename = "java-11.0-rc")]
    Java110Rc,
    #[serde(rename = "java-17.0-rc")]
    Java170Rc,
    #[serde(rename = "java-18.0-rc")]
    Java180Rc,
    #[serde(rename = "java-21.0-rc")]
    Java210Rc,
    #[serde(rename = "java-22-rc")]
    Java22Rc,
    #[serde(rename = "java-25-rc")]
    Java25Rc,
    #[serde(rename = "swift-5.5-rc")]
    Swift55Rc,
    #[serde(rename = "swift-5.8-rc")]
    Swift58Rc,
    #[serde(rename = "swift-5.9-rc")]
    Swift59Rc,
    #[serde(rename = "swift-5.10-rc")]
    Swift510Rc,
    #[serde(rename = "swift-6.2-rc")]
    Swift62Rc,
    #[serde(rename = "kotlin-1.6-rc")]
    Kotlin16Rc,
    #[serde(rename = "kotlin-1.8-rc")]
    Kotlin18Rc,
    #[serde(rename = "kotlin-1.9-rc")]
    Kotlin19Rc,
    #[serde(rename = "kotlin-2.0-rc")]
    Kotlin20Rc,
    #[serde(rename = "kotlin-2.3-rc")]
    Kotlin23Rc,
    #[serde(rename = "cpp-17-rc")]
    Cpp17Rc,
    #[serde(rename = "cpp-20-rc")]
    Cpp20Rc,
    #[serde(rename = "bun-1.0-rc")]
    Bun10Rc,
    #[serde(rename = "bun-1.1-rc")]
    Bun11Rc,
    #[serde(rename = "bun-1.2-rc")]
    Bun12Rc,
    #[serde(rename = "bun-1.3-rc")]
    Bun13Rc,
    #[serde(rename = "go-1.23-rc")]
    Go123Rc,
    #[serde(rename = "go-1.24-rc")]
    Go124Rc,
    #[serde(rename = "go-1.25-rc")]
    Go125Rc,
    #[serde(rename = "go-1.26-rc")]
    Go126Rc,
    #[serde(rename = "static-1-rc")]
    Static1Rc,
    #[serde(rename = "flutter-3.24-rc")]
    Flutter324Rc,
    #[serde(rename = "flutter-3.27-rc")]
    Flutter327Rc,
    #[serde(rename = "flutter-3.29-rc")]
    Flutter329Rc,
    #[serde(rename = "flutter-3.32-rc")]
    Flutter332Rc,
    #[serde(rename = "flutter-3.35-rc")]
    Flutter335Rc,
    #[serde(rename = "flutter-3.38-rc")]
    Flutter338Rc,
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
            BuildRuntime::Node145Rc => "node-14.5-rc",
            BuildRuntime::Node160Rc => "node-16.0-rc",
            BuildRuntime::Node180Rc => "node-18.0-rc",
            BuildRuntime::Node190Rc => "node-19.0-rc",
            BuildRuntime::Node200Rc => "node-20.0-rc",
            BuildRuntime::Node210Rc => "node-21.0-rc",
            BuildRuntime::Node22Rc => "node-22-rc",
            BuildRuntime::Node23Rc => "node-23-rc",
            BuildRuntime::Node24Rc => "node-24-rc",
            BuildRuntime::Node25Rc => "node-25-rc",
            BuildRuntime::Php80Rc => "php-8.0-rc",
            BuildRuntime::Php81Rc => "php-8.1-rc",
            BuildRuntime::Php82Rc => "php-8.2-rc",
            BuildRuntime::Php83Rc => "php-8.3-rc",
            BuildRuntime::Php84Rc => "php-8.4-rc",
            BuildRuntime::Ruby30Rc => "ruby-3.0-rc",
            BuildRuntime::Ruby31Rc => "ruby-3.1-rc",
            BuildRuntime::Ruby32Rc => "ruby-3.2-rc",
            BuildRuntime::Ruby33Rc => "ruby-3.3-rc",
            BuildRuntime::Ruby34Rc => "ruby-3.4-rc",
            BuildRuntime::Ruby40Rc => "ruby-4.0-rc",
            BuildRuntime::Python38Rc => "python-3.8-rc",
            BuildRuntime::Python39Rc => "python-3.9-rc",
            BuildRuntime::Python310Rc => "python-3.10-rc",
            BuildRuntime::Python311Rc => "python-3.11-rc",
            BuildRuntime::Python312Rc => "python-3.12-rc",
            BuildRuntime::Python313Rc => "python-3.13-rc",
            BuildRuntime::Python314Rc => "python-3.14-rc",
            BuildRuntime::PythonMl311Rc => "python-ml-3.11-rc",
            BuildRuntime::PythonMl312Rc => "python-ml-3.12-rc",
            BuildRuntime::PythonMl313Rc => "python-ml-3.13-rc",
            BuildRuntime::Deno140Rc => "deno-1.40-rc",
            BuildRuntime::Deno146Rc => "deno-1.46-rc",
            BuildRuntime::Deno20Rc => "deno-2.0-rc",
            BuildRuntime::Deno25Rc => "deno-2.5-rc",
            BuildRuntime::Deno26Rc => "deno-2.6-rc",
            BuildRuntime::Dart215Rc => "dart-2.15-rc",
            BuildRuntime::Dart216Rc => "dart-2.16-rc",
            BuildRuntime::Dart217Rc => "dart-2.17-rc",
            BuildRuntime::Dart218Rc => "dart-2.18-rc",
            BuildRuntime::Dart219Rc => "dart-2.19-rc",
            BuildRuntime::Dart30Rc => "dart-3.0-rc",
            BuildRuntime::Dart31Rc => "dart-3.1-rc",
            BuildRuntime::Dart33Rc => "dart-3.3-rc",
            BuildRuntime::Dart35Rc => "dart-3.5-rc",
            BuildRuntime::Dart38Rc => "dart-3.8-rc",
            BuildRuntime::Dart39Rc => "dart-3.9-rc",
            BuildRuntime::Dart310Rc => "dart-3.10-rc",
            BuildRuntime::Dotnet60Rc => "dotnet-6.0-rc",
            BuildRuntime::Dotnet70Rc => "dotnet-7.0-rc",
            BuildRuntime::Dotnet80Rc => "dotnet-8.0-rc",
            BuildRuntime::Dotnet10Rc => "dotnet-10-rc",
            BuildRuntime::Java80Rc => "java-8.0-rc",
            BuildRuntime::Java110Rc => "java-11.0-rc",
            BuildRuntime::Java170Rc => "java-17.0-rc",
            BuildRuntime::Java180Rc => "java-18.0-rc",
            BuildRuntime::Java210Rc => "java-21.0-rc",
            BuildRuntime::Java22Rc => "java-22-rc",
            BuildRuntime::Java25Rc => "java-25-rc",
            BuildRuntime::Swift55Rc => "swift-5.5-rc",
            BuildRuntime::Swift58Rc => "swift-5.8-rc",
            BuildRuntime::Swift59Rc => "swift-5.9-rc",
            BuildRuntime::Swift510Rc => "swift-5.10-rc",
            BuildRuntime::Swift62Rc => "swift-6.2-rc",
            BuildRuntime::Kotlin16Rc => "kotlin-1.6-rc",
            BuildRuntime::Kotlin18Rc => "kotlin-1.8-rc",
            BuildRuntime::Kotlin19Rc => "kotlin-1.9-rc",
            BuildRuntime::Kotlin20Rc => "kotlin-2.0-rc",
            BuildRuntime::Kotlin23Rc => "kotlin-2.3-rc",
            BuildRuntime::Cpp17Rc => "cpp-17-rc",
            BuildRuntime::Cpp20Rc => "cpp-20-rc",
            BuildRuntime::Bun10Rc => "bun-1.0-rc",
            BuildRuntime::Bun11Rc => "bun-1.1-rc",
            BuildRuntime::Bun12Rc => "bun-1.2-rc",
            BuildRuntime::Bun13Rc => "bun-1.3-rc",
            BuildRuntime::Go123Rc => "go-1.23-rc",
            BuildRuntime::Go124Rc => "go-1.24-rc",
            BuildRuntime::Go125Rc => "go-1.25-rc",
            BuildRuntime::Go126Rc => "go-1.26-rc",
            BuildRuntime::Static1Rc => "static-1-rc",
            BuildRuntime::Flutter324Rc => "flutter-3.24-rc",
            BuildRuntime::Flutter327Rc => "flutter-3.27-rc",
            BuildRuntime::Flutter329Rc => "flutter-3.29-rc",
            BuildRuntime::Flutter332Rc => "flutter-3.32-rc",
            BuildRuntime::Flutter335Rc => "flutter-3.35-rc",
            BuildRuntime::Flutter338Rc => "flutter-3.38-rc",
        }
    }
}

impl std::fmt::Display for BuildRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
