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
            Runtime::Node145Rc => "node-14.5-rc",
            Runtime::Node160Rc => "node-16.0-rc",
            Runtime::Node180Rc => "node-18.0-rc",
            Runtime::Node190Rc => "node-19.0-rc",
            Runtime::Node200Rc => "node-20.0-rc",
            Runtime::Node210Rc => "node-21.0-rc",
            Runtime::Node22Rc => "node-22-rc",
            Runtime::Node23Rc => "node-23-rc",
            Runtime::Node24Rc => "node-24-rc",
            Runtime::Node25Rc => "node-25-rc",
            Runtime::Php80Rc => "php-8.0-rc",
            Runtime::Php81Rc => "php-8.1-rc",
            Runtime::Php82Rc => "php-8.2-rc",
            Runtime::Php83Rc => "php-8.3-rc",
            Runtime::Php84Rc => "php-8.4-rc",
            Runtime::Ruby30Rc => "ruby-3.0-rc",
            Runtime::Ruby31Rc => "ruby-3.1-rc",
            Runtime::Ruby32Rc => "ruby-3.2-rc",
            Runtime::Ruby33Rc => "ruby-3.3-rc",
            Runtime::Ruby34Rc => "ruby-3.4-rc",
            Runtime::Ruby40Rc => "ruby-4.0-rc",
            Runtime::Python38Rc => "python-3.8-rc",
            Runtime::Python39Rc => "python-3.9-rc",
            Runtime::Python310Rc => "python-3.10-rc",
            Runtime::Python311Rc => "python-3.11-rc",
            Runtime::Python312Rc => "python-3.12-rc",
            Runtime::Python313Rc => "python-3.13-rc",
            Runtime::Python314Rc => "python-3.14-rc",
            Runtime::PythonMl311Rc => "python-ml-3.11-rc",
            Runtime::PythonMl312Rc => "python-ml-3.12-rc",
            Runtime::PythonMl313Rc => "python-ml-3.13-rc",
            Runtime::Deno140Rc => "deno-1.40-rc",
            Runtime::Deno146Rc => "deno-1.46-rc",
            Runtime::Deno20Rc => "deno-2.0-rc",
            Runtime::Deno25Rc => "deno-2.5-rc",
            Runtime::Deno26Rc => "deno-2.6-rc",
            Runtime::Dart215Rc => "dart-2.15-rc",
            Runtime::Dart216Rc => "dart-2.16-rc",
            Runtime::Dart217Rc => "dart-2.17-rc",
            Runtime::Dart218Rc => "dart-2.18-rc",
            Runtime::Dart219Rc => "dart-2.19-rc",
            Runtime::Dart30Rc => "dart-3.0-rc",
            Runtime::Dart31Rc => "dart-3.1-rc",
            Runtime::Dart33Rc => "dart-3.3-rc",
            Runtime::Dart35Rc => "dart-3.5-rc",
            Runtime::Dart38Rc => "dart-3.8-rc",
            Runtime::Dart39Rc => "dart-3.9-rc",
            Runtime::Dart310Rc => "dart-3.10-rc",
            Runtime::Dotnet60Rc => "dotnet-6.0-rc",
            Runtime::Dotnet70Rc => "dotnet-7.0-rc",
            Runtime::Dotnet80Rc => "dotnet-8.0-rc",
            Runtime::Dotnet10Rc => "dotnet-10-rc",
            Runtime::Java80Rc => "java-8.0-rc",
            Runtime::Java110Rc => "java-11.0-rc",
            Runtime::Java170Rc => "java-17.0-rc",
            Runtime::Java180Rc => "java-18.0-rc",
            Runtime::Java210Rc => "java-21.0-rc",
            Runtime::Java22Rc => "java-22-rc",
            Runtime::Java25Rc => "java-25-rc",
            Runtime::Swift55Rc => "swift-5.5-rc",
            Runtime::Swift58Rc => "swift-5.8-rc",
            Runtime::Swift59Rc => "swift-5.9-rc",
            Runtime::Swift510Rc => "swift-5.10-rc",
            Runtime::Swift62Rc => "swift-6.2-rc",
            Runtime::Kotlin16Rc => "kotlin-1.6-rc",
            Runtime::Kotlin18Rc => "kotlin-1.8-rc",
            Runtime::Kotlin19Rc => "kotlin-1.9-rc",
            Runtime::Kotlin20Rc => "kotlin-2.0-rc",
            Runtime::Kotlin23Rc => "kotlin-2.3-rc",
            Runtime::Cpp17Rc => "cpp-17-rc",
            Runtime::Cpp20Rc => "cpp-20-rc",
            Runtime::Bun10Rc => "bun-1.0-rc",
            Runtime::Bun11Rc => "bun-1.1-rc",
            Runtime::Bun12Rc => "bun-1.2-rc",
            Runtime::Bun13Rc => "bun-1.3-rc",
            Runtime::Go123Rc => "go-1.23-rc",
            Runtime::Go124Rc => "go-1.24-rc",
            Runtime::Go125Rc => "go-1.25-rc",
            Runtime::Go126Rc => "go-1.26-rc",
            Runtime::Static1Rc => "static-1-rc",
            Runtime::Flutter324Rc => "flutter-3.24-rc",
            Runtime::Flutter327Rc => "flutter-3.27-rc",
            Runtime::Flutter329Rc => "flutter-3.29-rc",
            Runtime::Flutter332Rc => "flutter-3.32-rc",
            Runtime::Flutter335Rc => "flutter-3.35-rc",
            Runtime::Flutter338Rc => "flutter-3.38-rc",
        }
    }
}

impl std::fmt::Display for Runtime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
