use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Flag {
    #[serde(rename = "af")]
    #[default]
    Afghanistan,
    #[serde(rename = "ao")]
    Angola,
    #[serde(rename = "al")]
    Albania,
    #[serde(rename = "ad")]
    Andorra,
    #[serde(rename = "ae")]
    UnitedArabEmirates,
    #[serde(rename = "ar")]
    Argentina,
    #[serde(rename = "am")]
    Armenia,
    #[serde(rename = "ag")]
    AntiguaAndBarbuda,
    #[serde(rename = "au")]
    Australia,
    #[serde(rename = "at")]
    Austria,
    #[serde(rename = "az")]
    Azerbaijan,
    #[serde(rename = "bi")]
    Burundi,
    #[serde(rename = "be")]
    Belgium,
    #[serde(rename = "bj")]
    Benin,
    #[serde(rename = "bf")]
    BurkinaFaso,
    #[serde(rename = "bd")]
    Bangladesh,
    #[serde(rename = "bg")]
    Bulgaria,
    #[serde(rename = "bh")]
    Bahrain,
    #[serde(rename = "bs")]
    Bahamas,
    #[serde(rename = "ba")]
    BosniaAndHerzegovina,
    #[serde(rename = "by")]
    Belarus,
    #[serde(rename = "bz")]
    Belize,
    #[serde(rename = "bo")]
    Bolivia,
    #[serde(rename = "br")]
    Brazil,
    #[serde(rename = "bb")]
    Barbados,
    #[serde(rename = "bn")]
    BruneiDarussalam,
    #[serde(rename = "bt")]
    Bhutan,
    #[serde(rename = "bw")]
    Botswana,
    #[serde(rename = "cf")]
    CentralAfricanRepublic,
    #[serde(rename = "ca")]
    Canada,
    #[serde(rename = "ch")]
    Switzerland,
    #[serde(rename = "cl")]
    Chile,
    #[serde(rename = "cn")]
    China,
    #[serde(rename = "ci")]
    CoteDIvoire,
    #[serde(rename = "cm")]
    Cameroon,
    #[serde(rename = "cd")]
    DemocraticRepublicOfTheCongo,
    #[serde(rename = "cg")]
    RepublicOfTheCongo,
    #[serde(rename = "co")]
    Colombia,
    #[serde(rename = "km")]
    Comoros,
    #[serde(rename = "cv")]
    CapeVerde,
    #[serde(rename = "cr")]
    CostaRica,
    #[serde(rename = "cu")]
    Cuba,
    #[serde(rename = "cy")]
    Cyprus,
    #[serde(rename = "cz")]
    CzechRepublic,
    #[serde(rename = "de")]
    Germany,
    #[serde(rename = "dj")]
    Djibouti,
    #[serde(rename = "dm")]
    Dominica,
    #[serde(rename = "dk")]
    Denmark,
    #[serde(rename = "do")]
    DominicanRepublic,
    #[serde(rename = "dz")]
    Algeria,
    #[serde(rename = "ec")]
    Ecuador,
    #[serde(rename = "eg")]
    Egypt,
    #[serde(rename = "er")]
    Eritrea,
    #[serde(rename = "es")]
    Spain,
    #[serde(rename = "ee")]
    Estonia,
    #[serde(rename = "et")]
    Ethiopia,
    #[serde(rename = "fi")]
    Finland,
    #[serde(rename = "fj")]
    Fiji,
    #[serde(rename = "fr")]
    France,
    #[serde(rename = "fm")]
    MicronesiaFederatedStatesOf,
    #[serde(rename = "ga")]
    Gabon,
    #[serde(rename = "gb")]
    UnitedKingdom,
    #[serde(rename = "ge")]
    Georgia,
    #[serde(rename = "gh")]
    Ghana,
    #[serde(rename = "gn")]
    Guinea,
    #[serde(rename = "gm")]
    Gambia,
    #[serde(rename = "gw")]
    GuineaBissau,
    #[serde(rename = "gq")]
    EquatorialGuinea,
    #[serde(rename = "gr")]
    Greece,
    #[serde(rename = "gd")]
    Grenada,
    #[serde(rename = "gt")]
    Guatemala,
    #[serde(rename = "gy")]
    Guyana,
    #[serde(rename = "hn")]
    Honduras,
    #[serde(rename = "hr")]
    Croatia,
    #[serde(rename = "ht")]
    Haiti,
    #[serde(rename = "hu")]
    Hungary,
    #[serde(rename = "id")]
    Indonesia,
    #[serde(rename = "in")]
    India,
    #[serde(rename = "ie")]
    Ireland,
    #[serde(rename = "ir")]
    IranIslamicRepublicOf,
    #[serde(rename = "iq")]
    Iraq,
    #[serde(rename = "is")]
    Iceland,
    #[serde(rename = "il")]
    Israel,
    #[serde(rename = "it")]
    Italy,
    #[serde(rename = "jm")]
    Jamaica,
    #[serde(rename = "jo")]
    Jordan,
    #[serde(rename = "jp")]
    Japan,
    #[serde(rename = "kz")]
    Kazakhstan,
    #[serde(rename = "ke")]
    Kenya,
    #[serde(rename = "kg")]
    Kyrgyzstan,
    #[serde(rename = "kh")]
    Cambodia,
    #[serde(rename = "ki")]
    Kiribati,
    #[serde(rename = "kn")]
    SaintKittsAndNevis,
    #[serde(rename = "kr")]
    SouthKorea,
    #[serde(rename = "kw")]
    Kuwait,
    #[serde(rename = "la")]
    LaoPeopleSDemocraticRepublic,
    #[serde(rename = "lb")]
    Lebanon,
    #[serde(rename = "lr")]
    Liberia,
    #[serde(rename = "ly")]
    Libya,
    #[serde(rename = "lc")]
    SaintLucia,
    #[serde(rename = "li")]
    Liechtenstein,
    #[serde(rename = "lk")]
    SriLanka,
    #[serde(rename = "ls")]
    Lesotho,
    #[serde(rename = "lt")]
    Lithuania,
    #[serde(rename = "lu")]
    Luxembourg,
    #[serde(rename = "lv")]
    Latvia,
    #[serde(rename = "ma")]
    Morocco,
    #[serde(rename = "mc")]
    Monaco,
    #[serde(rename = "md")]
    Moldova,
    #[serde(rename = "mg")]
    Madagascar,
    #[serde(rename = "mv")]
    Maldives,
    #[serde(rename = "mx")]
    Mexico,
    #[serde(rename = "mh")]
    MarshallIslands,
    #[serde(rename = "mk")]
    NorthMacedonia,
    #[serde(rename = "ml")]
    Mali,
    #[serde(rename = "mt")]
    Malta,
    #[serde(rename = "mm")]
    Myanmar,
    #[serde(rename = "me")]
    Montenegro,
    #[serde(rename = "mn")]
    Mongolia,
    #[serde(rename = "mz")]
    Mozambique,
    #[serde(rename = "mr")]
    Mauritania,
    #[serde(rename = "mu")]
    Mauritius,
    #[serde(rename = "mw")]
    Malawi,
    #[serde(rename = "my")]
    Malaysia,
    #[serde(rename = "na")]
    Namibia,
    #[serde(rename = "ne")]
    Niger,
    #[serde(rename = "ng")]
    Nigeria,
    #[serde(rename = "ni")]
    Nicaragua,
    #[serde(rename = "nl")]
    Netherlands,
    #[serde(rename = "no")]
    Norway,
    #[serde(rename = "np")]
    Nepal,
    #[serde(rename = "nr")]
    Nauru,
    #[serde(rename = "nz")]
    NewZealand,
    #[serde(rename = "om")]
    Oman,
    #[serde(rename = "pk")]
    Pakistan,
    #[serde(rename = "pa")]
    Panama,
    #[serde(rename = "pe")]
    Peru,
    #[serde(rename = "ph")]
    Philippines,
    #[serde(rename = "pw")]
    Palau,
    #[serde(rename = "pg")]
    PapuaNewGuinea,
    #[serde(rename = "pl")]
    Poland,
    #[serde(rename = "pf")]
    FrenchPolynesia,
    #[serde(rename = "kp")]
    NorthKorea,
    #[serde(rename = "pt")]
    Portugal,
    #[serde(rename = "py")]
    Paraguay,
    #[serde(rename = "qa")]
    Qatar,
    #[serde(rename = "ro")]
    Romania,
    #[serde(rename = "ru")]
    Russia,
    #[serde(rename = "rw")]
    Rwanda,
    #[serde(rename = "sa")]
    SaudiArabia,
    #[serde(rename = "sd")]
    Sudan,
    #[serde(rename = "sn")]
    Senegal,
    #[serde(rename = "sg")]
    Singapore,
    #[serde(rename = "sb")]
    SolomonIslands,
    #[serde(rename = "sl")]
    SierraLeone,
    #[serde(rename = "sv")]
    ElSalvador,
    #[serde(rename = "sm")]
    SanMarino,
    #[serde(rename = "so")]
    Somalia,
    #[serde(rename = "rs")]
    Serbia,
    #[serde(rename = "ss")]
    SouthSudan,
    #[serde(rename = "st")]
    SaoTomeAndPrincipe,
    #[serde(rename = "sr")]
    Suriname,
    #[serde(rename = "sk")]
    Slovakia,
    #[serde(rename = "si")]
    Slovenia,
    #[serde(rename = "se")]
    Sweden,
    #[serde(rename = "sz")]
    Eswatini,
    #[serde(rename = "sc")]
    Seychelles,
    #[serde(rename = "sy")]
    Syria,
    #[serde(rename = "td")]
    Chad,
    #[serde(rename = "tg")]
    Togo,
    #[serde(rename = "th")]
    Thailand,
    #[serde(rename = "tj")]
    Tajikistan,
    #[serde(rename = "tm")]
    Turkmenistan,
    #[serde(rename = "tl")]
    TimorLeste,
    #[serde(rename = "to")]
    Tonga,
    #[serde(rename = "tt")]
    TrinidadAndTobago,
    #[serde(rename = "tn")]
    Tunisia,
    #[serde(rename = "tr")]
    Turkey,
    #[serde(rename = "tv")]
    Tuvalu,
    #[serde(rename = "tz")]
    Tanzania,
    #[serde(rename = "ug")]
    Uganda,
    #[serde(rename = "ua")]
    Ukraine,
    #[serde(rename = "uy")]
    Uruguay,
    #[serde(rename = "us")]
    UnitedStates,
    #[serde(rename = "uz")]
    Uzbekistan,
    #[serde(rename = "va")]
    VaticanCity,
    #[serde(rename = "vc")]
    SaintVincentAndTheGrenadines,
    #[serde(rename = "ve")]
    Venezuela,
    #[serde(rename = "vn")]
    Vietnam,
    #[serde(rename = "vu")]
    Vanuatu,
    #[serde(rename = "ws")]
    Samoa,
    #[serde(rename = "ye")]
    Yemen,
    #[serde(rename = "za")]
    SouthAfrica,
    #[serde(rename = "zm")]
    Zambia,
    #[serde(rename = "zw")]
    Zimbabwe,
}

impl Flag {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Flag::Afghanistan => "af",
            Flag::Angola => "ao",
            Flag::Albania => "al",
            Flag::Andorra => "ad",
            Flag::UnitedArabEmirates => "ae",
            Flag::Argentina => "ar",
            Flag::Armenia => "am",
            Flag::AntiguaAndBarbuda => "ag",
            Flag::Australia => "au",
            Flag::Austria => "at",
            Flag::Azerbaijan => "az",
            Flag::Burundi => "bi",
            Flag::Belgium => "be",
            Flag::Benin => "bj",
            Flag::BurkinaFaso => "bf",
            Flag::Bangladesh => "bd",
            Flag::Bulgaria => "bg",
            Flag::Bahrain => "bh",
            Flag::Bahamas => "bs",
            Flag::BosniaAndHerzegovina => "ba",
            Flag::Belarus => "by",
            Flag::Belize => "bz",
            Flag::Bolivia => "bo",
            Flag::Brazil => "br",
            Flag::Barbados => "bb",
            Flag::BruneiDarussalam => "bn",
            Flag::Bhutan => "bt",
            Flag::Botswana => "bw",
            Flag::CentralAfricanRepublic => "cf",
            Flag::Canada => "ca",
            Flag::Switzerland => "ch",
            Flag::Chile => "cl",
            Flag::China => "cn",
            Flag::CoteDIvoire => "ci",
            Flag::Cameroon => "cm",
            Flag::DemocraticRepublicOfTheCongo => "cd",
            Flag::RepublicOfTheCongo => "cg",
            Flag::Colombia => "co",
            Flag::Comoros => "km",
            Flag::CapeVerde => "cv",
            Flag::CostaRica => "cr",
            Flag::Cuba => "cu",
            Flag::Cyprus => "cy",
            Flag::CzechRepublic => "cz",
            Flag::Germany => "de",
            Flag::Djibouti => "dj",
            Flag::Dominica => "dm",
            Flag::Denmark => "dk",
            Flag::DominicanRepublic => "do",
            Flag::Algeria => "dz",
            Flag::Ecuador => "ec",
            Flag::Egypt => "eg",
            Flag::Eritrea => "er",
            Flag::Spain => "es",
            Flag::Estonia => "ee",
            Flag::Ethiopia => "et",
            Flag::Finland => "fi",
            Flag::Fiji => "fj",
            Flag::France => "fr",
            Flag::MicronesiaFederatedStatesOf => "fm",
            Flag::Gabon => "ga",
            Flag::UnitedKingdom => "gb",
            Flag::Georgia => "ge",
            Flag::Ghana => "gh",
            Flag::Guinea => "gn",
            Flag::Gambia => "gm",
            Flag::GuineaBissau => "gw",
            Flag::EquatorialGuinea => "gq",
            Flag::Greece => "gr",
            Flag::Grenada => "gd",
            Flag::Guatemala => "gt",
            Flag::Guyana => "gy",
            Flag::Honduras => "hn",
            Flag::Croatia => "hr",
            Flag::Haiti => "ht",
            Flag::Hungary => "hu",
            Flag::Indonesia => "id",
            Flag::India => "in",
            Flag::Ireland => "ie",
            Flag::IranIslamicRepublicOf => "ir",
            Flag::Iraq => "iq",
            Flag::Iceland => "is",
            Flag::Israel => "il",
            Flag::Italy => "it",
            Flag::Jamaica => "jm",
            Flag::Jordan => "jo",
            Flag::Japan => "jp",
            Flag::Kazakhstan => "kz",
            Flag::Kenya => "ke",
            Flag::Kyrgyzstan => "kg",
            Flag::Cambodia => "kh",
            Flag::Kiribati => "ki",
            Flag::SaintKittsAndNevis => "kn",
            Flag::SouthKorea => "kr",
            Flag::Kuwait => "kw",
            Flag::LaoPeopleSDemocraticRepublic => "la",
            Flag::Lebanon => "lb",
            Flag::Liberia => "lr",
            Flag::Libya => "ly",
            Flag::SaintLucia => "lc",
            Flag::Liechtenstein => "li",
            Flag::SriLanka => "lk",
            Flag::Lesotho => "ls",
            Flag::Lithuania => "lt",
            Flag::Luxembourg => "lu",
            Flag::Latvia => "lv",
            Flag::Morocco => "ma",
            Flag::Monaco => "mc",
            Flag::Moldova => "md",
            Flag::Madagascar => "mg",
            Flag::Maldives => "mv",
            Flag::Mexico => "mx",
            Flag::MarshallIslands => "mh",
            Flag::NorthMacedonia => "mk",
            Flag::Mali => "ml",
            Flag::Malta => "mt",
            Flag::Myanmar => "mm",
            Flag::Montenegro => "me",
            Flag::Mongolia => "mn",
            Flag::Mozambique => "mz",
            Flag::Mauritania => "mr",
            Flag::Mauritius => "mu",
            Flag::Malawi => "mw",
            Flag::Malaysia => "my",
            Flag::Namibia => "na",
            Flag::Niger => "ne",
            Flag::Nigeria => "ng",
            Flag::Nicaragua => "ni",
            Flag::Netherlands => "nl",
            Flag::Norway => "no",
            Flag::Nepal => "np",
            Flag::Nauru => "nr",
            Flag::NewZealand => "nz",
            Flag::Oman => "om",
            Flag::Pakistan => "pk",
            Flag::Panama => "pa",
            Flag::Peru => "pe",
            Flag::Philippines => "ph",
            Flag::Palau => "pw",
            Flag::PapuaNewGuinea => "pg",
            Flag::Poland => "pl",
            Flag::FrenchPolynesia => "pf",
            Flag::NorthKorea => "kp",
            Flag::Portugal => "pt",
            Flag::Paraguay => "py",
            Flag::Qatar => "qa",
            Flag::Romania => "ro",
            Flag::Russia => "ru",
            Flag::Rwanda => "rw",
            Flag::SaudiArabia => "sa",
            Flag::Sudan => "sd",
            Flag::Senegal => "sn",
            Flag::Singapore => "sg",
            Flag::SolomonIslands => "sb",
            Flag::SierraLeone => "sl",
            Flag::ElSalvador => "sv",
            Flag::SanMarino => "sm",
            Flag::Somalia => "so",
            Flag::Serbia => "rs",
            Flag::SouthSudan => "ss",
            Flag::SaoTomeAndPrincipe => "st",
            Flag::Suriname => "sr",
            Flag::Slovakia => "sk",
            Flag::Slovenia => "si",
            Flag::Sweden => "se",
            Flag::Eswatini => "sz",
            Flag::Seychelles => "sc",
            Flag::Syria => "sy",
            Flag::Chad => "td",
            Flag::Togo => "tg",
            Flag::Thailand => "th",
            Flag::Tajikistan => "tj",
            Flag::Turkmenistan => "tm",
            Flag::TimorLeste => "tl",
            Flag::Tonga => "to",
            Flag::TrinidadAndTobago => "tt",
            Flag::Tunisia => "tn",
            Flag::Turkey => "tr",
            Flag::Tuvalu => "tv",
            Flag::Tanzania => "tz",
            Flag::Uganda => "ug",
            Flag::Ukraine => "ua",
            Flag::Uruguay => "uy",
            Flag::UnitedStates => "us",
            Flag::Uzbekistan => "uz",
            Flag::VaticanCity => "va",
            Flag::SaintVincentAndTheGrenadines => "vc",
            Flag::Venezuela => "ve",
            Flag::Vietnam => "vn",
            Flag::Vanuatu => "vu",
            Flag::Samoa => "ws",
            Flag::Yemen => "ye",
            Flag::SouthAfrica => "za",
            Flag::Zambia => "zm",
            Flag::Zimbabwe => "zw",
        }
    }
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
