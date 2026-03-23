use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum Timezone {
    #[serde(rename = "africa/abidjan")]
    #[default]
    AfricaAbidjan,
    #[serde(rename = "africa/accra")]
    AfricaAccra,
    #[serde(rename = "africa/addis_ababa")]
    AfricaAddisAbaba,
    #[serde(rename = "africa/algiers")]
    AfricaAlgiers,
    #[serde(rename = "africa/asmara")]
    AfricaAsmara,
    #[serde(rename = "africa/bamako")]
    AfricaBamako,
    #[serde(rename = "africa/bangui")]
    AfricaBangui,
    #[serde(rename = "africa/banjul")]
    AfricaBanjul,
    #[serde(rename = "africa/bissau")]
    AfricaBissau,
    #[serde(rename = "africa/blantyre")]
    AfricaBlantyre,
    #[serde(rename = "africa/brazzaville")]
    AfricaBrazzaville,
    #[serde(rename = "africa/bujumbura")]
    AfricaBujumbura,
    #[serde(rename = "africa/cairo")]
    AfricaCairo,
    #[serde(rename = "africa/casablanca")]
    AfricaCasablanca,
    #[serde(rename = "africa/ceuta")]
    AfricaCeuta,
    #[serde(rename = "africa/conakry")]
    AfricaConakry,
    #[serde(rename = "africa/dakar")]
    AfricaDakar,
    #[serde(rename = "africa/dar_es_salaam")]
    AfricaDarEsSalaam,
    #[serde(rename = "africa/djibouti")]
    AfricaDjibouti,
    #[serde(rename = "africa/douala")]
    AfricaDouala,
    #[serde(rename = "africa/el_aaiun")]
    AfricaElAaiun,
    #[serde(rename = "africa/freetown")]
    AfricaFreetown,
    #[serde(rename = "africa/gaborone")]
    AfricaGaborone,
    #[serde(rename = "africa/harare")]
    AfricaHarare,
    #[serde(rename = "africa/johannesburg")]
    AfricaJohannesburg,
    #[serde(rename = "africa/juba")]
    AfricaJuba,
    #[serde(rename = "africa/kampala")]
    AfricaKampala,
    #[serde(rename = "africa/khartoum")]
    AfricaKhartoum,
    #[serde(rename = "africa/kigali")]
    AfricaKigali,
    #[serde(rename = "africa/kinshasa")]
    AfricaKinshasa,
    #[serde(rename = "africa/lagos")]
    AfricaLagos,
    #[serde(rename = "africa/libreville")]
    AfricaLibreville,
    #[serde(rename = "africa/lome")]
    AfricaLome,
    #[serde(rename = "africa/luanda")]
    AfricaLuanda,
    #[serde(rename = "africa/lubumbashi")]
    AfricaLubumbashi,
    #[serde(rename = "africa/lusaka")]
    AfricaLusaka,
    #[serde(rename = "africa/malabo")]
    AfricaMalabo,
    #[serde(rename = "africa/maputo")]
    AfricaMaputo,
    #[serde(rename = "africa/maseru")]
    AfricaMaseru,
    #[serde(rename = "africa/mbabane")]
    AfricaMbabane,
    #[serde(rename = "africa/mogadishu")]
    AfricaMogadishu,
    #[serde(rename = "africa/monrovia")]
    AfricaMonrovia,
    #[serde(rename = "africa/nairobi")]
    AfricaNairobi,
    #[serde(rename = "africa/ndjamena")]
    AfricaNdjamena,
    #[serde(rename = "africa/niamey")]
    AfricaNiamey,
    #[serde(rename = "africa/nouakchott")]
    AfricaNouakchott,
    #[serde(rename = "africa/ouagadougou")]
    AfricaOuagadougou,
    #[serde(rename = "africa/porto-novo")]
    AfricaPortoNovo,
    #[serde(rename = "africa/sao_tome")]
    AfricaSaoTome,
    #[serde(rename = "africa/tripoli")]
    AfricaTripoli,
    #[serde(rename = "africa/tunis")]
    AfricaTunis,
    #[serde(rename = "africa/windhoek")]
    AfricaWindhoek,
    #[serde(rename = "america/adak")]
    AmericaAdak,
    #[serde(rename = "america/anchorage")]
    AmericaAnchorage,
    #[serde(rename = "america/anguilla")]
    AmericaAnguilla,
    #[serde(rename = "america/antigua")]
    AmericaAntigua,
    #[serde(rename = "america/araguaina")]
    AmericaAraguaina,
    #[serde(rename = "america/argentina/buenos_aires")]
    AmericaArgentinaBuenosAires,
    #[serde(rename = "america/argentina/catamarca")]
    AmericaArgentinaCatamarca,
    #[serde(rename = "america/argentina/cordoba")]
    AmericaArgentinaCordoba,
    #[serde(rename = "america/argentina/jujuy")]
    AmericaArgentinaJujuy,
    #[serde(rename = "america/argentina/la_rioja")]
    AmericaArgentinaLaRioja,
    #[serde(rename = "america/argentina/mendoza")]
    AmericaArgentinaMendoza,
    #[serde(rename = "america/argentina/rio_gallegos")]
    AmericaArgentinaRioGallegos,
    #[serde(rename = "america/argentina/salta")]
    AmericaArgentinaSalta,
    #[serde(rename = "america/argentina/san_juan")]
    AmericaArgentinaSanJuan,
    #[serde(rename = "america/argentina/san_luis")]
    AmericaArgentinaSanLuis,
    #[serde(rename = "america/argentina/tucuman")]
    AmericaArgentinaTucuman,
    #[serde(rename = "america/argentina/ushuaia")]
    AmericaArgentinaUshuaia,
    #[serde(rename = "america/aruba")]
    AmericaAruba,
    #[serde(rename = "america/asuncion")]
    AmericaAsuncion,
    #[serde(rename = "america/atikokan")]
    AmericaAtikokan,
    #[serde(rename = "america/bahia")]
    AmericaBahia,
    #[serde(rename = "america/bahia_banderas")]
    AmericaBahiaBanderas,
    #[serde(rename = "america/barbados")]
    AmericaBarbados,
    #[serde(rename = "america/belem")]
    AmericaBelem,
    #[serde(rename = "america/belize")]
    AmericaBelize,
    #[serde(rename = "america/blanc-sablon")]
    AmericaBlancSablon,
    #[serde(rename = "america/boa_vista")]
    AmericaBoaVista,
    #[serde(rename = "america/bogota")]
    AmericaBogota,
    #[serde(rename = "america/boise")]
    AmericaBoise,
    #[serde(rename = "america/cambridge_bay")]
    AmericaCambridgeBay,
    #[serde(rename = "america/campo_grande")]
    AmericaCampoGrande,
    #[serde(rename = "america/cancun")]
    AmericaCancun,
    #[serde(rename = "america/caracas")]
    AmericaCaracas,
    #[serde(rename = "america/cayenne")]
    AmericaCayenne,
    #[serde(rename = "america/cayman")]
    AmericaCayman,
    #[serde(rename = "america/chicago")]
    AmericaChicago,
    #[serde(rename = "america/chihuahua")]
    AmericaChihuahua,
    #[serde(rename = "america/ciudad_juarez")]
    AmericaCiudadJuarez,
    #[serde(rename = "america/costa_rica")]
    AmericaCostaRica,
    #[serde(rename = "america/coyhaique")]
    AmericaCoyhaique,
    #[serde(rename = "america/creston")]
    AmericaCreston,
    #[serde(rename = "america/cuiaba")]
    AmericaCuiaba,
    #[serde(rename = "america/curacao")]
    AmericaCuracao,
    #[serde(rename = "america/danmarkshavn")]
    AmericaDanmarkshavn,
    #[serde(rename = "america/dawson")]
    AmericaDawson,
    #[serde(rename = "america/dawson_creek")]
    AmericaDawsonCreek,
    #[serde(rename = "america/denver")]
    AmericaDenver,
    #[serde(rename = "america/detroit")]
    AmericaDetroit,
    #[serde(rename = "america/dominica")]
    AmericaDominica,
    #[serde(rename = "america/edmonton")]
    AmericaEdmonton,
    #[serde(rename = "america/eirunepe")]
    AmericaEirunepe,
    #[serde(rename = "america/el_salvador")]
    AmericaElSalvador,
    #[serde(rename = "america/fort_nelson")]
    AmericaFortNelson,
    #[serde(rename = "america/fortaleza")]
    AmericaFortaleza,
    #[serde(rename = "america/glace_bay")]
    AmericaGlaceBay,
    #[serde(rename = "america/goose_bay")]
    AmericaGooseBay,
    #[serde(rename = "america/grand_turk")]
    AmericaGrandTurk,
    #[serde(rename = "america/grenada")]
    AmericaGrenada,
    #[serde(rename = "america/guadeloupe")]
    AmericaGuadeloupe,
    #[serde(rename = "america/guatemala")]
    AmericaGuatemala,
    #[serde(rename = "america/guayaquil")]
    AmericaGuayaquil,
    #[serde(rename = "america/guyana")]
    AmericaGuyana,
    #[serde(rename = "america/halifax")]
    AmericaHalifax,
    #[serde(rename = "america/havana")]
    AmericaHavana,
    #[serde(rename = "america/hermosillo")]
    AmericaHermosillo,
    #[serde(rename = "america/indiana/indianapolis")]
    AmericaIndianaIndianapolis,
    #[serde(rename = "america/indiana/knox")]
    AmericaIndianaKnox,
    #[serde(rename = "america/indiana/marengo")]
    AmericaIndianaMarengo,
    #[serde(rename = "america/indiana/petersburg")]
    AmericaIndianaPetersburg,
    #[serde(rename = "america/indiana/tell_city")]
    AmericaIndianaTellCity,
    #[serde(rename = "america/indiana/vevay")]
    AmericaIndianaVevay,
    #[serde(rename = "america/indiana/vincennes")]
    AmericaIndianaVincennes,
    #[serde(rename = "america/indiana/winamac")]
    AmericaIndianaWinamac,
    #[serde(rename = "america/inuvik")]
    AmericaInuvik,
    #[serde(rename = "america/iqaluit")]
    AmericaIqaluit,
    #[serde(rename = "america/jamaica")]
    AmericaJamaica,
    #[serde(rename = "america/juneau")]
    AmericaJuneau,
    #[serde(rename = "america/kentucky/louisville")]
    AmericaKentuckyLouisville,
    #[serde(rename = "america/kentucky/monticello")]
    AmericaKentuckyMonticello,
    #[serde(rename = "america/kralendijk")]
    AmericaKralendijk,
    #[serde(rename = "america/la_paz")]
    AmericaLaPaz,
    #[serde(rename = "america/lima")]
    AmericaLima,
    #[serde(rename = "america/los_angeles")]
    AmericaLosAngeles,
    #[serde(rename = "america/lower_princes")]
    AmericaLowerPrinces,
    #[serde(rename = "america/maceio")]
    AmericaMaceio,
    #[serde(rename = "america/managua")]
    AmericaManagua,
    #[serde(rename = "america/manaus")]
    AmericaManaus,
    #[serde(rename = "america/marigot")]
    AmericaMarigot,
    #[serde(rename = "america/martinique")]
    AmericaMartinique,
    #[serde(rename = "america/matamoros")]
    AmericaMatamoros,
    #[serde(rename = "america/mazatlan")]
    AmericaMazatlan,
    #[serde(rename = "america/menominee")]
    AmericaMenominee,
    #[serde(rename = "america/merida")]
    AmericaMerida,
    #[serde(rename = "america/metlakatla")]
    AmericaMetlakatla,
    #[serde(rename = "america/mexico_city")]
    AmericaMexicoCity,
    #[serde(rename = "america/miquelon")]
    AmericaMiquelon,
    #[serde(rename = "america/moncton")]
    AmericaMoncton,
    #[serde(rename = "america/monterrey")]
    AmericaMonterrey,
    #[serde(rename = "america/montevideo")]
    AmericaMontevideo,
    #[serde(rename = "america/montserrat")]
    AmericaMontserrat,
    #[serde(rename = "america/nassau")]
    AmericaNassau,
    #[serde(rename = "america/new_york")]
    AmericaNewYork,
    #[serde(rename = "america/nome")]
    AmericaNome,
    #[serde(rename = "america/noronha")]
    AmericaNoronha,
    #[serde(rename = "america/north_dakota/beulah")]
    AmericaNorthDakotaBeulah,
    #[serde(rename = "america/north_dakota/center")]
    AmericaNorthDakotaCenter,
    #[serde(rename = "america/north_dakota/new_salem")]
    AmericaNorthDakotaNewSalem,
    #[serde(rename = "america/nuuk")]
    AmericaNuuk,
    #[serde(rename = "america/ojinaga")]
    AmericaOjinaga,
    #[serde(rename = "america/panama")]
    AmericaPanama,
    #[serde(rename = "america/paramaribo")]
    AmericaParamaribo,
    #[serde(rename = "america/phoenix")]
    AmericaPhoenix,
    #[serde(rename = "america/port-au-prince")]
    AmericaPortAuPrince,
    #[serde(rename = "america/port_of_spain")]
    AmericaPortOfSpain,
    #[serde(rename = "america/porto_velho")]
    AmericaPortoVelho,
    #[serde(rename = "america/puerto_rico")]
    AmericaPuertoRico,
    #[serde(rename = "america/punta_arenas")]
    AmericaPuntaArenas,
    #[serde(rename = "america/rankin_inlet")]
    AmericaRankinInlet,
    #[serde(rename = "america/recife")]
    AmericaRecife,
    #[serde(rename = "america/regina")]
    AmericaRegina,
    #[serde(rename = "america/resolute")]
    AmericaResolute,
    #[serde(rename = "america/rio_branco")]
    AmericaRioBranco,
    #[serde(rename = "america/santarem")]
    AmericaSantarem,
    #[serde(rename = "america/santiago")]
    AmericaSantiago,
    #[serde(rename = "america/santo_domingo")]
    AmericaSantoDomingo,
    #[serde(rename = "america/sao_paulo")]
    AmericaSaoPaulo,
    #[serde(rename = "america/scoresbysund")]
    AmericaScoresbysund,
    #[serde(rename = "america/sitka")]
    AmericaSitka,
    #[serde(rename = "america/st_barthelemy")]
    AmericaStBarthelemy,
    #[serde(rename = "america/st_johns")]
    AmericaStJohns,
    #[serde(rename = "america/st_kitts")]
    AmericaStKitts,
    #[serde(rename = "america/st_lucia")]
    AmericaStLucia,
    #[serde(rename = "america/st_thomas")]
    AmericaStThomas,
    #[serde(rename = "america/st_vincent")]
    AmericaStVincent,
    #[serde(rename = "america/swift_current")]
    AmericaSwiftCurrent,
    #[serde(rename = "america/tegucigalpa")]
    AmericaTegucigalpa,
    #[serde(rename = "america/thule")]
    AmericaThule,
    #[serde(rename = "america/tijuana")]
    AmericaTijuana,
    #[serde(rename = "america/toronto")]
    AmericaToronto,
    #[serde(rename = "america/tortola")]
    AmericaTortola,
    #[serde(rename = "america/vancouver")]
    AmericaVancouver,
    #[serde(rename = "america/whitehorse")]
    AmericaWhitehorse,
    #[serde(rename = "america/winnipeg")]
    AmericaWinnipeg,
    #[serde(rename = "america/yakutat")]
    AmericaYakutat,
    #[serde(rename = "antarctica/casey")]
    AntarcticaCasey,
    #[serde(rename = "antarctica/davis")]
    AntarcticaDavis,
    #[serde(rename = "antarctica/dumontdurville")]
    AntarcticaDumontdurville,
    #[serde(rename = "antarctica/macquarie")]
    AntarcticaMacquarie,
    #[serde(rename = "antarctica/mawson")]
    AntarcticaMawson,
    #[serde(rename = "antarctica/mcmurdo")]
    AntarcticaMcmurdo,
    #[serde(rename = "antarctica/palmer")]
    AntarcticaPalmer,
    #[serde(rename = "antarctica/rothera")]
    AntarcticaRothera,
    #[serde(rename = "antarctica/syowa")]
    AntarcticaSyowa,
    #[serde(rename = "antarctica/troll")]
    AntarcticaTroll,
    #[serde(rename = "antarctica/vostok")]
    AntarcticaVostok,
    #[serde(rename = "arctic/longyearbyen")]
    ArcticLongyearbyen,
    #[serde(rename = "asia/aden")]
    AsiaAden,
    #[serde(rename = "asia/almaty")]
    AsiaAlmaty,
    #[serde(rename = "asia/amman")]
    AsiaAmman,
    #[serde(rename = "asia/anadyr")]
    AsiaAnadyr,
    #[serde(rename = "asia/aqtau")]
    AsiaAqtau,
    #[serde(rename = "asia/aqtobe")]
    AsiaAqtobe,
    #[serde(rename = "asia/ashgabat")]
    AsiaAshgabat,
    #[serde(rename = "asia/atyrau")]
    AsiaAtyrau,
    #[serde(rename = "asia/baghdad")]
    AsiaBaghdad,
    #[serde(rename = "asia/bahrain")]
    AsiaBahrain,
    #[serde(rename = "asia/baku")]
    AsiaBaku,
    #[serde(rename = "asia/bangkok")]
    AsiaBangkok,
    #[serde(rename = "asia/barnaul")]
    AsiaBarnaul,
    #[serde(rename = "asia/beirut")]
    AsiaBeirut,
    #[serde(rename = "asia/bishkek")]
    AsiaBishkek,
    #[serde(rename = "asia/brunei")]
    AsiaBrunei,
    #[serde(rename = "asia/chita")]
    AsiaChita,
    #[serde(rename = "asia/colombo")]
    AsiaColombo,
    #[serde(rename = "asia/damascus")]
    AsiaDamascus,
    #[serde(rename = "asia/dhaka")]
    AsiaDhaka,
    #[serde(rename = "asia/dili")]
    AsiaDili,
    #[serde(rename = "asia/dubai")]
    AsiaDubai,
    #[serde(rename = "asia/dushanbe")]
    AsiaDushanbe,
    #[serde(rename = "asia/famagusta")]
    AsiaFamagusta,
    #[serde(rename = "asia/gaza")]
    AsiaGaza,
    #[serde(rename = "asia/hebron")]
    AsiaHebron,
    #[serde(rename = "asia/ho_chi_minh")]
    AsiaHoChiMinh,
    #[serde(rename = "asia/hong_kong")]
    AsiaHongKong,
    #[serde(rename = "asia/hovd")]
    AsiaHovd,
    #[serde(rename = "asia/irkutsk")]
    AsiaIrkutsk,
    #[serde(rename = "asia/jakarta")]
    AsiaJakarta,
    #[serde(rename = "asia/jayapura")]
    AsiaJayapura,
    #[serde(rename = "asia/jerusalem")]
    AsiaJerusalem,
    #[serde(rename = "asia/kabul")]
    AsiaKabul,
    #[serde(rename = "asia/kamchatka")]
    AsiaKamchatka,
    #[serde(rename = "asia/karachi")]
    AsiaKarachi,
    #[serde(rename = "asia/kathmandu")]
    AsiaKathmandu,
    #[serde(rename = "asia/khandyga")]
    AsiaKhandyga,
    #[serde(rename = "asia/kolkata")]
    AsiaKolkata,
    #[serde(rename = "asia/krasnoyarsk")]
    AsiaKrasnoyarsk,
    #[serde(rename = "asia/kuala_lumpur")]
    AsiaKualaLumpur,
    #[serde(rename = "asia/kuching")]
    AsiaKuching,
    #[serde(rename = "asia/kuwait")]
    AsiaKuwait,
    #[serde(rename = "asia/macau")]
    AsiaMacau,
    #[serde(rename = "asia/magadan")]
    AsiaMagadan,
    #[serde(rename = "asia/makassar")]
    AsiaMakassar,
    #[serde(rename = "asia/manila")]
    AsiaManila,
    #[serde(rename = "asia/muscat")]
    AsiaMuscat,
    #[serde(rename = "asia/nicosia")]
    AsiaNicosia,
    #[serde(rename = "asia/novokuznetsk")]
    AsiaNovokuznetsk,
    #[serde(rename = "asia/novosibirsk")]
    AsiaNovosibirsk,
    #[serde(rename = "asia/omsk")]
    AsiaOmsk,
    #[serde(rename = "asia/oral")]
    AsiaOral,
    #[serde(rename = "asia/phnom_penh")]
    AsiaPhnomPenh,
    #[serde(rename = "asia/pontianak")]
    AsiaPontianak,
    #[serde(rename = "asia/pyongyang")]
    AsiaPyongyang,
    #[serde(rename = "asia/qatar")]
    AsiaQatar,
    #[serde(rename = "asia/qostanay")]
    AsiaQostanay,
    #[serde(rename = "asia/qyzylorda")]
    AsiaQyzylorda,
    #[serde(rename = "asia/riyadh")]
    AsiaRiyadh,
    #[serde(rename = "asia/sakhalin")]
    AsiaSakhalin,
    #[serde(rename = "asia/samarkand")]
    AsiaSamarkand,
    #[serde(rename = "asia/seoul")]
    AsiaSeoul,
    #[serde(rename = "asia/shanghai")]
    AsiaShanghai,
    #[serde(rename = "asia/singapore")]
    AsiaSingapore,
    #[serde(rename = "asia/srednekolymsk")]
    AsiaSrednekolymsk,
    #[serde(rename = "asia/taipei")]
    AsiaTaipei,
    #[serde(rename = "asia/tashkent")]
    AsiaTashkent,
    #[serde(rename = "asia/tbilisi")]
    AsiaTbilisi,
    #[serde(rename = "asia/tehran")]
    AsiaTehran,
    #[serde(rename = "asia/thimphu")]
    AsiaThimphu,
    #[serde(rename = "asia/tokyo")]
    AsiaTokyo,
    #[serde(rename = "asia/tomsk")]
    AsiaTomsk,
    #[serde(rename = "asia/ulaanbaatar")]
    AsiaUlaanbaatar,
    #[serde(rename = "asia/urumqi")]
    AsiaUrumqi,
    #[serde(rename = "asia/ust-nera")]
    AsiaUstNera,
    #[serde(rename = "asia/vientiane")]
    AsiaVientiane,
    #[serde(rename = "asia/vladivostok")]
    AsiaVladivostok,
    #[serde(rename = "asia/yakutsk")]
    AsiaYakutsk,
    #[serde(rename = "asia/yangon")]
    AsiaYangon,
    #[serde(rename = "asia/yekaterinburg")]
    AsiaYekaterinburg,
    #[serde(rename = "asia/yerevan")]
    AsiaYerevan,
    #[serde(rename = "atlantic/azores")]
    AtlanticAzores,
    #[serde(rename = "atlantic/bermuda")]
    AtlanticBermuda,
    #[serde(rename = "atlantic/canary")]
    AtlanticCanary,
    #[serde(rename = "atlantic/cape_verde")]
    AtlanticCapeVerde,
    #[serde(rename = "atlantic/faroe")]
    AtlanticFaroe,
    #[serde(rename = "atlantic/madeira")]
    AtlanticMadeira,
    #[serde(rename = "atlantic/reykjavik")]
    AtlanticReykjavik,
    #[serde(rename = "atlantic/south_georgia")]
    AtlanticSouthGeorgia,
    #[serde(rename = "atlantic/st_helena")]
    AtlanticStHelena,
    #[serde(rename = "atlantic/stanley")]
    AtlanticStanley,
    #[serde(rename = "australia/adelaide")]
    AustraliaAdelaide,
    #[serde(rename = "australia/brisbane")]
    AustraliaBrisbane,
    #[serde(rename = "australia/broken_hill")]
    AustraliaBrokenHill,
    #[serde(rename = "australia/darwin")]
    AustraliaDarwin,
    #[serde(rename = "australia/eucla")]
    AustraliaEucla,
    #[serde(rename = "australia/hobart")]
    AustraliaHobart,
    #[serde(rename = "australia/lindeman")]
    AustraliaLindeman,
    #[serde(rename = "australia/lord_howe")]
    AustraliaLordHowe,
    #[serde(rename = "australia/melbourne")]
    AustraliaMelbourne,
    #[serde(rename = "australia/perth")]
    AustraliaPerth,
    #[serde(rename = "australia/sydney")]
    AustraliaSydney,
    #[serde(rename = "europe/amsterdam")]
    EuropeAmsterdam,
    #[serde(rename = "europe/andorra")]
    EuropeAndorra,
    #[serde(rename = "europe/astrakhan")]
    EuropeAstrakhan,
    #[serde(rename = "europe/athens")]
    EuropeAthens,
    #[serde(rename = "europe/belgrade")]
    EuropeBelgrade,
    #[serde(rename = "europe/berlin")]
    EuropeBerlin,
    #[serde(rename = "europe/bratislava")]
    EuropeBratislava,
    #[serde(rename = "europe/brussels")]
    EuropeBrussels,
    #[serde(rename = "europe/bucharest")]
    EuropeBucharest,
    #[serde(rename = "europe/budapest")]
    EuropeBudapest,
    #[serde(rename = "europe/busingen")]
    EuropeBusingen,
    #[serde(rename = "europe/chisinau")]
    EuropeChisinau,
    #[serde(rename = "europe/copenhagen")]
    EuropeCopenhagen,
    #[serde(rename = "europe/dublin")]
    EuropeDublin,
    #[serde(rename = "europe/gibraltar")]
    EuropeGibraltar,
    #[serde(rename = "europe/guernsey")]
    EuropeGuernsey,
    #[serde(rename = "europe/helsinki")]
    EuropeHelsinki,
    #[serde(rename = "europe/isle_of_man")]
    EuropeIsleOfMan,
    #[serde(rename = "europe/istanbul")]
    EuropeIstanbul,
    #[serde(rename = "europe/jersey")]
    EuropeJersey,
    #[serde(rename = "europe/kaliningrad")]
    EuropeKaliningrad,
    #[serde(rename = "europe/kirov")]
    EuropeKirov,
    #[serde(rename = "europe/kyiv")]
    EuropeKyiv,
    #[serde(rename = "europe/lisbon")]
    EuropeLisbon,
    #[serde(rename = "europe/ljubljana")]
    EuropeLjubljana,
    #[serde(rename = "europe/london")]
    EuropeLondon,
    #[serde(rename = "europe/luxembourg")]
    EuropeLuxembourg,
    #[serde(rename = "europe/madrid")]
    EuropeMadrid,
    #[serde(rename = "europe/malta")]
    EuropeMalta,
    #[serde(rename = "europe/mariehamn")]
    EuropeMariehamn,
    #[serde(rename = "europe/minsk")]
    EuropeMinsk,
    #[serde(rename = "europe/monaco")]
    EuropeMonaco,
    #[serde(rename = "europe/moscow")]
    EuropeMoscow,
    #[serde(rename = "europe/oslo")]
    EuropeOslo,
    #[serde(rename = "europe/paris")]
    EuropeParis,
    #[serde(rename = "europe/podgorica")]
    EuropePodgorica,
    #[serde(rename = "europe/prague")]
    EuropePrague,
    #[serde(rename = "europe/riga")]
    EuropeRiga,
    #[serde(rename = "europe/rome")]
    EuropeRome,
    #[serde(rename = "europe/samara")]
    EuropeSamara,
    #[serde(rename = "europe/san_marino")]
    EuropeSanMarino,
    #[serde(rename = "europe/sarajevo")]
    EuropeSarajevo,
    #[serde(rename = "europe/saratov")]
    EuropeSaratov,
    #[serde(rename = "europe/simferopol")]
    EuropeSimferopol,
    #[serde(rename = "europe/skopje")]
    EuropeSkopje,
    #[serde(rename = "europe/sofia")]
    EuropeSofia,
    #[serde(rename = "europe/stockholm")]
    EuropeStockholm,
    #[serde(rename = "europe/tallinn")]
    EuropeTallinn,
    #[serde(rename = "europe/tirane")]
    EuropeTirane,
    #[serde(rename = "europe/ulyanovsk")]
    EuropeUlyanovsk,
    #[serde(rename = "europe/vaduz")]
    EuropeVaduz,
    #[serde(rename = "europe/vatican")]
    EuropeVatican,
    #[serde(rename = "europe/vienna")]
    EuropeVienna,
    #[serde(rename = "europe/vilnius")]
    EuropeVilnius,
    #[serde(rename = "europe/volgograd")]
    EuropeVolgograd,
    #[serde(rename = "europe/warsaw")]
    EuropeWarsaw,
    #[serde(rename = "europe/zagreb")]
    EuropeZagreb,
    #[serde(rename = "europe/zurich")]
    EuropeZurich,
    #[serde(rename = "indian/antananarivo")]
    IndianAntananarivo,
    #[serde(rename = "indian/chagos")]
    IndianChagos,
    #[serde(rename = "indian/christmas")]
    IndianChristmas,
    #[serde(rename = "indian/cocos")]
    IndianCocos,
    #[serde(rename = "indian/comoro")]
    IndianComoro,
    #[serde(rename = "indian/kerguelen")]
    IndianKerguelen,
    #[serde(rename = "indian/mahe")]
    IndianMahe,
    #[serde(rename = "indian/maldives")]
    IndianMaldives,
    #[serde(rename = "indian/mauritius")]
    IndianMauritius,
    #[serde(rename = "indian/mayotte")]
    IndianMayotte,
    #[serde(rename = "indian/reunion")]
    IndianReunion,
    #[serde(rename = "pacific/apia")]
    PacificApia,
    #[serde(rename = "pacific/auckland")]
    PacificAuckland,
    #[serde(rename = "pacific/bougainville")]
    PacificBougainville,
    #[serde(rename = "pacific/chatham")]
    PacificChatham,
    #[serde(rename = "pacific/chuuk")]
    PacificChuuk,
    #[serde(rename = "pacific/easter")]
    PacificEaster,
    #[serde(rename = "pacific/efate")]
    PacificEfate,
    #[serde(rename = "pacific/fakaofo")]
    PacificFakaofo,
    #[serde(rename = "pacific/fiji")]
    PacificFiji,
    #[serde(rename = "pacific/funafuti")]
    PacificFunafuti,
    #[serde(rename = "pacific/galapagos")]
    PacificGalapagos,
    #[serde(rename = "pacific/gambier")]
    PacificGambier,
    #[serde(rename = "pacific/guadalcanal")]
    PacificGuadalcanal,
    #[serde(rename = "pacific/guam")]
    PacificGuam,
    #[serde(rename = "pacific/honolulu")]
    PacificHonolulu,
    #[serde(rename = "pacific/kanton")]
    PacificKanton,
    #[serde(rename = "pacific/kiritimati")]
    PacificKiritimati,
    #[serde(rename = "pacific/kosrae")]
    PacificKosrae,
    #[serde(rename = "pacific/kwajalein")]
    PacificKwajalein,
    #[serde(rename = "pacific/majuro")]
    PacificMajuro,
    #[serde(rename = "pacific/marquesas")]
    PacificMarquesas,
    #[serde(rename = "pacific/midway")]
    PacificMidway,
    #[serde(rename = "pacific/nauru")]
    PacificNauru,
    #[serde(rename = "pacific/niue")]
    PacificNiue,
    #[serde(rename = "pacific/norfolk")]
    PacificNorfolk,
    #[serde(rename = "pacific/noumea")]
    PacificNoumea,
    #[serde(rename = "pacific/pago_pago")]
    PacificPagoPago,
    #[serde(rename = "pacific/palau")]
    PacificPalau,
    #[serde(rename = "pacific/pitcairn")]
    PacificPitcairn,
    #[serde(rename = "pacific/pohnpei")]
    PacificPohnpei,
    #[serde(rename = "pacific/port_moresby")]
    PacificPortMoresby,
    #[serde(rename = "pacific/rarotonga")]
    PacificRarotonga,
    #[serde(rename = "pacific/saipan")]
    PacificSaipan,
    #[serde(rename = "pacific/tahiti")]
    PacificTahiti,
    #[serde(rename = "pacific/tarawa")]
    PacificTarawa,
    #[serde(rename = "pacific/tongatapu")]
    PacificTongatapu,
    #[serde(rename = "pacific/wake")]
    PacificWake,
    #[serde(rename = "pacific/wallis")]
    PacificWallis,
    #[serde(rename = "utc")]
    Utc,
}

impl Timezone {
    /// Get the string value of the enum
    pub fn as_str(&self) -> &str {
        match self {
            Timezone::AfricaAbidjan => "africa/abidjan",
            Timezone::AfricaAccra => "africa/accra",
            Timezone::AfricaAddisAbaba => "africa/addis_ababa",
            Timezone::AfricaAlgiers => "africa/algiers",
            Timezone::AfricaAsmara => "africa/asmara",
            Timezone::AfricaBamako => "africa/bamako",
            Timezone::AfricaBangui => "africa/bangui",
            Timezone::AfricaBanjul => "africa/banjul",
            Timezone::AfricaBissau => "africa/bissau",
            Timezone::AfricaBlantyre => "africa/blantyre",
            Timezone::AfricaBrazzaville => "africa/brazzaville",
            Timezone::AfricaBujumbura => "africa/bujumbura",
            Timezone::AfricaCairo => "africa/cairo",
            Timezone::AfricaCasablanca => "africa/casablanca",
            Timezone::AfricaCeuta => "africa/ceuta",
            Timezone::AfricaConakry => "africa/conakry",
            Timezone::AfricaDakar => "africa/dakar",
            Timezone::AfricaDarEsSalaam => "africa/dar_es_salaam",
            Timezone::AfricaDjibouti => "africa/djibouti",
            Timezone::AfricaDouala => "africa/douala",
            Timezone::AfricaElAaiun => "africa/el_aaiun",
            Timezone::AfricaFreetown => "africa/freetown",
            Timezone::AfricaGaborone => "africa/gaborone",
            Timezone::AfricaHarare => "africa/harare",
            Timezone::AfricaJohannesburg => "africa/johannesburg",
            Timezone::AfricaJuba => "africa/juba",
            Timezone::AfricaKampala => "africa/kampala",
            Timezone::AfricaKhartoum => "africa/khartoum",
            Timezone::AfricaKigali => "africa/kigali",
            Timezone::AfricaKinshasa => "africa/kinshasa",
            Timezone::AfricaLagos => "africa/lagos",
            Timezone::AfricaLibreville => "africa/libreville",
            Timezone::AfricaLome => "africa/lome",
            Timezone::AfricaLuanda => "africa/luanda",
            Timezone::AfricaLubumbashi => "africa/lubumbashi",
            Timezone::AfricaLusaka => "africa/lusaka",
            Timezone::AfricaMalabo => "africa/malabo",
            Timezone::AfricaMaputo => "africa/maputo",
            Timezone::AfricaMaseru => "africa/maseru",
            Timezone::AfricaMbabane => "africa/mbabane",
            Timezone::AfricaMogadishu => "africa/mogadishu",
            Timezone::AfricaMonrovia => "africa/monrovia",
            Timezone::AfricaNairobi => "africa/nairobi",
            Timezone::AfricaNdjamena => "africa/ndjamena",
            Timezone::AfricaNiamey => "africa/niamey",
            Timezone::AfricaNouakchott => "africa/nouakchott",
            Timezone::AfricaOuagadougou => "africa/ouagadougou",
            Timezone::AfricaPortoNovo => "africa/porto-novo",
            Timezone::AfricaSaoTome => "africa/sao_tome",
            Timezone::AfricaTripoli => "africa/tripoli",
            Timezone::AfricaTunis => "africa/tunis",
            Timezone::AfricaWindhoek => "africa/windhoek",
            Timezone::AmericaAdak => "america/adak",
            Timezone::AmericaAnchorage => "america/anchorage",
            Timezone::AmericaAnguilla => "america/anguilla",
            Timezone::AmericaAntigua => "america/antigua",
            Timezone::AmericaAraguaina => "america/araguaina",
            Timezone::AmericaArgentinaBuenosAires => "america/argentina/buenos_aires",
            Timezone::AmericaArgentinaCatamarca => "america/argentina/catamarca",
            Timezone::AmericaArgentinaCordoba => "america/argentina/cordoba",
            Timezone::AmericaArgentinaJujuy => "america/argentina/jujuy",
            Timezone::AmericaArgentinaLaRioja => "america/argentina/la_rioja",
            Timezone::AmericaArgentinaMendoza => "america/argentina/mendoza",
            Timezone::AmericaArgentinaRioGallegos => "america/argentina/rio_gallegos",
            Timezone::AmericaArgentinaSalta => "america/argentina/salta",
            Timezone::AmericaArgentinaSanJuan => "america/argentina/san_juan",
            Timezone::AmericaArgentinaSanLuis => "america/argentina/san_luis",
            Timezone::AmericaArgentinaTucuman => "america/argentina/tucuman",
            Timezone::AmericaArgentinaUshuaia => "america/argentina/ushuaia",
            Timezone::AmericaAruba => "america/aruba",
            Timezone::AmericaAsuncion => "america/asuncion",
            Timezone::AmericaAtikokan => "america/atikokan",
            Timezone::AmericaBahia => "america/bahia",
            Timezone::AmericaBahiaBanderas => "america/bahia_banderas",
            Timezone::AmericaBarbados => "america/barbados",
            Timezone::AmericaBelem => "america/belem",
            Timezone::AmericaBelize => "america/belize",
            Timezone::AmericaBlancSablon => "america/blanc-sablon",
            Timezone::AmericaBoaVista => "america/boa_vista",
            Timezone::AmericaBogota => "america/bogota",
            Timezone::AmericaBoise => "america/boise",
            Timezone::AmericaCambridgeBay => "america/cambridge_bay",
            Timezone::AmericaCampoGrande => "america/campo_grande",
            Timezone::AmericaCancun => "america/cancun",
            Timezone::AmericaCaracas => "america/caracas",
            Timezone::AmericaCayenne => "america/cayenne",
            Timezone::AmericaCayman => "america/cayman",
            Timezone::AmericaChicago => "america/chicago",
            Timezone::AmericaChihuahua => "america/chihuahua",
            Timezone::AmericaCiudadJuarez => "america/ciudad_juarez",
            Timezone::AmericaCostaRica => "america/costa_rica",
            Timezone::AmericaCoyhaique => "america/coyhaique",
            Timezone::AmericaCreston => "america/creston",
            Timezone::AmericaCuiaba => "america/cuiaba",
            Timezone::AmericaCuracao => "america/curacao",
            Timezone::AmericaDanmarkshavn => "america/danmarkshavn",
            Timezone::AmericaDawson => "america/dawson",
            Timezone::AmericaDawsonCreek => "america/dawson_creek",
            Timezone::AmericaDenver => "america/denver",
            Timezone::AmericaDetroit => "america/detroit",
            Timezone::AmericaDominica => "america/dominica",
            Timezone::AmericaEdmonton => "america/edmonton",
            Timezone::AmericaEirunepe => "america/eirunepe",
            Timezone::AmericaElSalvador => "america/el_salvador",
            Timezone::AmericaFortNelson => "america/fort_nelson",
            Timezone::AmericaFortaleza => "america/fortaleza",
            Timezone::AmericaGlaceBay => "america/glace_bay",
            Timezone::AmericaGooseBay => "america/goose_bay",
            Timezone::AmericaGrandTurk => "america/grand_turk",
            Timezone::AmericaGrenada => "america/grenada",
            Timezone::AmericaGuadeloupe => "america/guadeloupe",
            Timezone::AmericaGuatemala => "america/guatemala",
            Timezone::AmericaGuayaquil => "america/guayaquil",
            Timezone::AmericaGuyana => "america/guyana",
            Timezone::AmericaHalifax => "america/halifax",
            Timezone::AmericaHavana => "america/havana",
            Timezone::AmericaHermosillo => "america/hermosillo",
            Timezone::AmericaIndianaIndianapolis => "america/indiana/indianapolis",
            Timezone::AmericaIndianaKnox => "america/indiana/knox",
            Timezone::AmericaIndianaMarengo => "america/indiana/marengo",
            Timezone::AmericaIndianaPetersburg => "america/indiana/petersburg",
            Timezone::AmericaIndianaTellCity => "america/indiana/tell_city",
            Timezone::AmericaIndianaVevay => "america/indiana/vevay",
            Timezone::AmericaIndianaVincennes => "america/indiana/vincennes",
            Timezone::AmericaIndianaWinamac => "america/indiana/winamac",
            Timezone::AmericaInuvik => "america/inuvik",
            Timezone::AmericaIqaluit => "america/iqaluit",
            Timezone::AmericaJamaica => "america/jamaica",
            Timezone::AmericaJuneau => "america/juneau",
            Timezone::AmericaKentuckyLouisville => "america/kentucky/louisville",
            Timezone::AmericaKentuckyMonticello => "america/kentucky/monticello",
            Timezone::AmericaKralendijk => "america/kralendijk",
            Timezone::AmericaLaPaz => "america/la_paz",
            Timezone::AmericaLima => "america/lima",
            Timezone::AmericaLosAngeles => "america/los_angeles",
            Timezone::AmericaLowerPrinces => "america/lower_princes",
            Timezone::AmericaMaceio => "america/maceio",
            Timezone::AmericaManagua => "america/managua",
            Timezone::AmericaManaus => "america/manaus",
            Timezone::AmericaMarigot => "america/marigot",
            Timezone::AmericaMartinique => "america/martinique",
            Timezone::AmericaMatamoros => "america/matamoros",
            Timezone::AmericaMazatlan => "america/mazatlan",
            Timezone::AmericaMenominee => "america/menominee",
            Timezone::AmericaMerida => "america/merida",
            Timezone::AmericaMetlakatla => "america/metlakatla",
            Timezone::AmericaMexicoCity => "america/mexico_city",
            Timezone::AmericaMiquelon => "america/miquelon",
            Timezone::AmericaMoncton => "america/moncton",
            Timezone::AmericaMonterrey => "america/monterrey",
            Timezone::AmericaMontevideo => "america/montevideo",
            Timezone::AmericaMontserrat => "america/montserrat",
            Timezone::AmericaNassau => "america/nassau",
            Timezone::AmericaNewYork => "america/new_york",
            Timezone::AmericaNome => "america/nome",
            Timezone::AmericaNoronha => "america/noronha",
            Timezone::AmericaNorthDakotaBeulah => "america/north_dakota/beulah",
            Timezone::AmericaNorthDakotaCenter => "america/north_dakota/center",
            Timezone::AmericaNorthDakotaNewSalem => "america/north_dakota/new_salem",
            Timezone::AmericaNuuk => "america/nuuk",
            Timezone::AmericaOjinaga => "america/ojinaga",
            Timezone::AmericaPanama => "america/panama",
            Timezone::AmericaParamaribo => "america/paramaribo",
            Timezone::AmericaPhoenix => "america/phoenix",
            Timezone::AmericaPortAuPrince => "america/port-au-prince",
            Timezone::AmericaPortOfSpain => "america/port_of_spain",
            Timezone::AmericaPortoVelho => "america/porto_velho",
            Timezone::AmericaPuertoRico => "america/puerto_rico",
            Timezone::AmericaPuntaArenas => "america/punta_arenas",
            Timezone::AmericaRankinInlet => "america/rankin_inlet",
            Timezone::AmericaRecife => "america/recife",
            Timezone::AmericaRegina => "america/regina",
            Timezone::AmericaResolute => "america/resolute",
            Timezone::AmericaRioBranco => "america/rio_branco",
            Timezone::AmericaSantarem => "america/santarem",
            Timezone::AmericaSantiago => "america/santiago",
            Timezone::AmericaSantoDomingo => "america/santo_domingo",
            Timezone::AmericaSaoPaulo => "america/sao_paulo",
            Timezone::AmericaScoresbysund => "america/scoresbysund",
            Timezone::AmericaSitka => "america/sitka",
            Timezone::AmericaStBarthelemy => "america/st_barthelemy",
            Timezone::AmericaStJohns => "america/st_johns",
            Timezone::AmericaStKitts => "america/st_kitts",
            Timezone::AmericaStLucia => "america/st_lucia",
            Timezone::AmericaStThomas => "america/st_thomas",
            Timezone::AmericaStVincent => "america/st_vincent",
            Timezone::AmericaSwiftCurrent => "america/swift_current",
            Timezone::AmericaTegucigalpa => "america/tegucigalpa",
            Timezone::AmericaThule => "america/thule",
            Timezone::AmericaTijuana => "america/tijuana",
            Timezone::AmericaToronto => "america/toronto",
            Timezone::AmericaTortola => "america/tortola",
            Timezone::AmericaVancouver => "america/vancouver",
            Timezone::AmericaWhitehorse => "america/whitehorse",
            Timezone::AmericaWinnipeg => "america/winnipeg",
            Timezone::AmericaYakutat => "america/yakutat",
            Timezone::AntarcticaCasey => "antarctica/casey",
            Timezone::AntarcticaDavis => "antarctica/davis",
            Timezone::AntarcticaDumontdurville => "antarctica/dumontdurville",
            Timezone::AntarcticaMacquarie => "antarctica/macquarie",
            Timezone::AntarcticaMawson => "antarctica/mawson",
            Timezone::AntarcticaMcmurdo => "antarctica/mcmurdo",
            Timezone::AntarcticaPalmer => "antarctica/palmer",
            Timezone::AntarcticaRothera => "antarctica/rothera",
            Timezone::AntarcticaSyowa => "antarctica/syowa",
            Timezone::AntarcticaTroll => "antarctica/troll",
            Timezone::AntarcticaVostok => "antarctica/vostok",
            Timezone::ArcticLongyearbyen => "arctic/longyearbyen",
            Timezone::AsiaAden => "asia/aden",
            Timezone::AsiaAlmaty => "asia/almaty",
            Timezone::AsiaAmman => "asia/amman",
            Timezone::AsiaAnadyr => "asia/anadyr",
            Timezone::AsiaAqtau => "asia/aqtau",
            Timezone::AsiaAqtobe => "asia/aqtobe",
            Timezone::AsiaAshgabat => "asia/ashgabat",
            Timezone::AsiaAtyrau => "asia/atyrau",
            Timezone::AsiaBaghdad => "asia/baghdad",
            Timezone::AsiaBahrain => "asia/bahrain",
            Timezone::AsiaBaku => "asia/baku",
            Timezone::AsiaBangkok => "asia/bangkok",
            Timezone::AsiaBarnaul => "asia/barnaul",
            Timezone::AsiaBeirut => "asia/beirut",
            Timezone::AsiaBishkek => "asia/bishkek",
            Timezone::AsiaBrunei => "asia/brunei",
            Timezone::AsiaChita => "asia/chita",
            Timezone::AsiaColombo => "asia/colombo",
            Timezone::AsiaDamascus => "asia/damascus",
            Timezone::AsiaDhaka => "asia/dhaka",
            Timezone::AsiaDili => "asia/dili",
            Timezone::AsiaDubai => "asia/dubai",
            Timezone::AsiaDushanbe => "asia/dushanbe",
            Timezone::AsiaFamagusta => "asia/famagusta",
            Timezone::AsiaGaza => "asia/gaza",
            Timezone::AsiaHebron => "asia/hebron",
            Timezone::AsiaHoChiMinh => "asia/ho_chi_minh",
            Timezone::AsiaHongKong => "asia/hong_kong",
            Timezone::AsiaHovd => "asia/hovd",
            Timezone::AsiaIrkutsk => "asia/irkutsk",
            Timezone::AsiaJakarta => "asia/jakarta",
            Timezone::AsiaJayapura => "asia/jayapura",
            Timezone::AsiaJerusalem => "asia/jerusalem",
            Timezone::AsiaKabul => "asia/kabul",
            Timezone::AsiaKamchatka => "asia/kamchatka",
            Timezone::AsiaKarachi => "asia/karachi",
            Timezone::AsiaKathmandu => "asia/kathmandu",
            Timezone::AsiaKhandyga => "asia/khandyga",
            Timezone::AsiaKolkata => "asia/kolkata",
            Timezone::AsiaKrasnoyarsk => "asia/krasnoyarsk",
            Timezone::AsiaKualaLumpur => "asia/kuala_lumpur",
            Timezone::AsiaKuching => "asia/kuching",
            Timezone::AsiaKuwait => "asia/kuwait",
            Timezone::AsiaMacau => "asia/macau",
            Timezone::AsiaMagadan => "asia/magadan",
            Timezone::AsiaMakassar => "asia/makassar",
            Timezone::AsiaManila => "asia/manila",
            Timezone::AsiaMuscat => "asia/muscat",
            Timezone::AsiaNicosia => "asia/nicosia",
            Timezone::AsiaNovokuznetsk => "asia/novokuznetsk",
            Timezone::AsiaNovosibirsk => "asia/novosibirsk",
            Timezone::AsiaOmsk => "asia/omsk",
            Timezone::AsiaOral => "asia/oral",
            Timezone::AsiaPhnomPenh => "asia/phnom_penh",
            Timezone::AsiaPontianak => "asia/pontianak",
            Timezone::AsiaPyongyang => "asia/pyongyang",
            Timezone::AsiaQatar => "asia/qatar",
            Timezone::AsiaQostanay => "asia/qostanay",
            Timezone::AsiaQyzylorda => "asia/qyzylorda",
            Timezone::AsiaRiyadh => "asia/riyadh",
            Timezone::AsiaSakhalin => "asia/sakhalin",
            Timezone::AsiaSamarkand => "asia/samarkand",
            Timezone::AsiaSeoul => "asia/seoul",
            Timezone::AsiaShanghai => "asia/shanghai",
            Timezone::AsiaSingapore => "asia/singapore",
            Timezone::AsiaSrednekolymsk => "asia/srednekolymsk",
            Timezone::AsiaTaipei => "asia/taipei",
            Timezone::AsiaTashkent => "asia/tashkent",
            Timezone::AsiaTbilisi => "asia/tbilisi",
            Timezone::AsiaTehran => "asia/tehran",
            Timezone::AsiaThimphu => "asia/thimphu",
            Timezone::AsiaTokyo => "asia/tokyo",
            Timezone::AsiaTomsk => "asia/tomsk",
            Timezone::AsiaUlaanbaatar => "asia/ulaanbaatar",
            Timezone::AsiaUrumqi => "asia/urumqi",
            Timezone::AsiaUstNera => "asia/ust-nera",
            Timezone::AsiaVientiane => "asia/vientiane",
            Timezone::AsiaVladivostok => "asia/vladivostok",
            Timezone::AsiaYakutsk => "asia/yakutsk",
            Timezone::AsiaYangon => "asia/yangon",
            Timezone::AsiaYekaterinburg => "asia/yekaterinburg",
            Timezone::AsiaYerevan => "asia/yerevan",
            Timezone::AtlanticAzores => "atlantic/azores",
            Timezone::AtlanticBermuda => "atlantic/bermuda",
            Timezone::AtlanticCanary => "atlantic/canary",
            Timezone::AtlanticCapeVerde => "atlantic/cape_verde",
            Timezone::AtlanticFaroe => "atlantic/faroe",
            Timezone::AtlanticMadeira => "atlantic/madeira",
            Timezone::AtlanticReykjavik => "atlantic/reykjavik",
            Timezone::AtlanticSouthGeorgia => "atlantic/south_georgia",
            Timezone::AtlanticStHelena => "atlantic/st_helena",
            Timezone::AtlanticStanley => "atlantic/stanley",
            Timezone::AustraliaAdelaide => "australia/adelaide",
            Timezone::AustraliaBrisbane => "australia/brisbane",
            Timezone::AustraliaBrokenHill => "australia/broken_hill",
            Timezone::AustraliaDarwin => "australia/darwin",
            Timezone::AustraliaEucla => "australia/eucla",
            Timezone::AustraliaHobart => "australia/hobart",
            Timezone::AustraliaLindeman => "australia/lindeman",
            Timezone::AustraliaLordHowe => "australia/lord_howe",
            Timezone::AustraliaMelbourne => "australia/melbourne",
            Timezone::AustraliaPerth => "australia/perth",
            Timezone::AustraliaSydney => "australia/sydney",
            Timezone::EuropeAmsterdam => "europe/amsterdam",
            Timezone::EuropeAndorra => "europe/andorra",
            Timezone::EuropeAstrakhan => "europe/astrakhan",
            Timezone::EuropeAthens => "europe/athens",
            Timezone::EuropeBelgrade => "europe/belgrade",
            Timezone::EuropeBerlin => "europe/berlin",
            Timezone::EuropeBratislava => "europe/bratislava",
            Timezone::EuropeBrussels => "europe/brussels",
            Timezone::EuropeBucharest => "europe/bucharest",
            Timezone::EuropeBudapest => "europe/budapest",
            Timezone::EuropeBusingen => "europe/busingen",
            Timezone::EuropeChisinau => "europe/chisinau",
            Timezone::EuropeCopenhagen => "europe/copenhagen",
            Timezone::EuropeDublin => "europe/dublin",
            Timezone::EuropeGibraltar => "europe/gibraltar",
            Timezone::EuropeGuernsey => "europe/guernsey",
            Timezone::EuropeHelsinki => "europe/helsinki",
            Timezone::EuropeIsleOfMan => "europe/isle_of_man",
            Timezone::EuropeIstanbul => "europe/istanbul",
            Timezone::EuropeJersey => "europe/jersey",
            Timezone::EuropeKaliningrad => "europe/kaliningrad",
            Timezone::EuropeKirov => "europe/kirov",
            Timezone::EuropeKyiv => "europe/kyiv",
            Timezone::EuropeLisbon => "europe/lisbon",
            Timezone::EuropeLjubljana => "europe/ljubljana",
            Timezone::EuropeLondon => "europe/london",
            Timezone::EuropeLuxembourg => "europe/luxembourg",
            Timezone::EuropeMadrid => "europe/madrid",
            Timezone::EuropeMalta => "europe/malta",
            Timezone::EuropeMariehamn => "europe/mariehamn",
            Timezone::EuropeMinsk => "europe/minsk",
            Timezone::EuropeMonaco => "europe/monaco",
            Timezone::EuropeMoscow => "europe/moscow",
            Timezone::EuropeOslo => "europe/oslo",
            Timezone::EuropeParis => "europe/paris",
            Timezone::EuropePodgorica => "europe/podgorica",
            Timezone::EuropePrague => "europe/prague",
            Timezone::EuropeRiga => "europe/riga",
            Timezone::EuropeRome => "europe/rome",
            Timezone::EuropeSamara => "europe/samara",
            Timezone::EuropeSanMarino => "europe/san_marino",
            Timezone::EuropeSarajevo => "europe/sarajevo",
            Timezone::EuropeSaratov => "europe/saratov",
            Timezone::EuropeSimferopol => "europe/simferopol",
            Timezone::EuropeSkopje => "europe/skopje",
            Timezone::EuropeSofia => "europe/sofia",
            Timezone::EuropeStockholm => "europe/stockholm",
            Timezone::EuropeTallinn => "europe/tallinn",
            Timezone::EuropeTirane => "europe/tirane",
            Timezone::EuropeUlyanovsk => "europe/ulyanovsk",
            Timezone::EuropeVaduz => "europe/vaduz",
            Timezone::EuropeVatican => "europe/vatican",
            Timezone::EuropeVienna => "europe/vienna",
            Timezone::EuropeVilnius => "europe/vilnius",
            Timezone::EuropeVolgograd => "europe/volgograd",
            Timezone::EuropeWarsaw => "europe/warsaw",
            Timezone::EuropeZagreb => "europe/zagreb",
            Timezone::EuropeZurich => "europe/zurich",
            Timezone::IndianAntananarivo => "indian/antananarivo",
            Timezone::IndianChagos => "indian/chagos",
            Timezone::IndianChristmas => "indian/christmas",
            Timezone::IndianCocos => "indian/cocos",
            Timezone::IndianComoro => "indian/comoro",
            Timezone::IndianKerguelen => "indian/kerguelen",
            Timezone::IndianMahe => "indian/mahe",
            Timezone::IndianMaldives => "indian/maldives",
            Timezone::IndianMauritius => "indian/mauritius",
            Timezone::IndianMayotte => "indian/mayotte",
            Timezone::IndianReunion => "indian/reunion",
            Timezone::PacificApia => "pacific/apia",
            Timezone::PacificAuckland => "pacific/auckland",
            Timezone::PacificBougainville => "pacific/bougainville",
            Timezone::PacificChatham => "pacific/chatham",
            Timezone::PacificChuuk => "pacific/chuuk",
            Timezone::PacificEaster => "pacific/easter",
            Timezone::PacificEfate => "pacific/efate",
            Timezone::PacificFakaofo => "pacific/fakaofo",
            Timezone::PacificFiji => "pacific/fiji",
            Timezone::PacificFunafuti => "pacific/funafuti",
            Timezone::PacificGalapagos => "pacific/galapagos",
            Timezone::PacificGambier => "pacific/gambier",
            Timezone::PacificGuadalcanal => "pacific/guadalcanal",
            Timezone::PacificGuam => "pacific/guam",
            Timezone::PacificHonolulu => "pacific/honolulu",
            Timezone::PacificKanton => "pacific/kanton",
            Timezone::PacificKiritimati => "pacific/kiritimati",
            Timezone::PacificKosrae => "pacific/kosrae",
            Timezone::PacificKwajalein => "pacific/kwajalein",
            Timezone::PacificMajuro => "pacific/majuro",
            Timezone::PacificMarquesas => "pacific/marquesas",
            Timezone::PacificMidway => "pacific/midway",
            Timezone::PacificNauru => "pacific/nauru",
            Timezone::PacificNiue => "pacific/niue",
            Timezone::PacificNorfolk => "pacific/norfolk",
            Timezone::PacificNoumea => "pacific/noumea",
            Timezone::PacificPagoPago => "pacific/pago_pago",
            Timezone::PacificPalau => "pacific/palau",
            Timezone::PacificPitcairn => "pacific/pitcairn",
            Timezone::PacificPohnpei => "pacific/pohnpei",
            Timezone::PacificPortMoresby => "pacific/port_moresby",
            Timezone::PacificRarotonga => "pacific/rarotonga",
            Timezone::PacificSaipan => "pacific/saipan",
            Timezone::PacificTahiti => "pacific/tahiti",
            Timezone::PacificTarawa => "pacific/tarawa",
            Timezone::PacificTongatapu => "pacific/tongatapu",
            Timezone::PacificWake => "pacific/wake",
            Timezone::PacificWallis => "pacific/wallis",
            Timezone::Utc => "utc",
        }
    }
}

impl std::fmt::Display for Timezone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
