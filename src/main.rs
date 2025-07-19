use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use anyhow::{Context, Result, anyhow};
use dialoguer::{Input, Select, Confirm};
use colored::Colorize;
use futures::stream::{FuturesUnordered, StreamExt};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Copy, Clone)]
struct Region {
    code: &'static str,
    name: &'static str,
}

const REGIONS: &[Region] = &[
    // Africa, Middle East, and India
    Region { code: "DZ", name: "Algeria" },
    Region { code: "AO", name: "Angola" },
    Region { code: "BJ", name: "Benin" },
    Region { code: "BW", name: "Botswana" },
    Region { code: "BF", name: "Burkina Faso" },
    Region { code: "CM", name: "Cameroon" },
    Region { code: "CI", name: "Côte d’Ivoire" },
    Region { code: "CD", name: "Democratic Republic of the Congo" },
    Region { code: "EG", name: "Egypt" },
    Region { code: "GH", name: "Ghana" },
    Region { code: "GW", name: "Guinea-Bissau" },
    Region { code: "IN", name: "India" },
    Region { code: "IL", name: "Israel" },
    Region { code: "JO", name: "Jordan" },
    Region { code: "KE", name: "Kenya" },
    Region { code: "KW", name: "Kuwait" },
    Region { code: "LR", name: "Liberia" },
    Region { code: "LY", name: "Libya" },
    Region { code: "MG", name: "Madagascar" },
    Region { code: "MW", name: "Malawi" },
    Region { code: "ML", name: "Mali" },
    Region { code: "MR", name: "Mauritania" },
    Region { code: "MU", name: "Mauritius" },
    Region { code: "MA", name: "Morocco" },
    Region { code: "MZ", name: "Mozambique" },
    Region { code: "NA", name: "Namibia" },
    Region { code: "NE", name: "Niger" },
    Region { code: "NG", name: "Nigeria" },
    Region { code: "OM", name: "Oman" },
    Region { code: "PK", name: "Pakistan" },
    Region { code: "QA", name: "Qatar" },
    Region { code: "RW", name: "Rwanda" },
    Region { code: "SA", name: "Saudi Arabia" },
    Region { code: "SN", name: "Senegal" },
    Region { code: "SC", name: "Seychelles" },
    Region { code: "SL", name: "Sierra Leone" },
    Region { code: "ZA", name: "South Africa" },
    Region { code: "TZ", name: "Tanzania" },
    Region { code: "TN", name: "Tunisia" },
    Region { code: "UG", name: "Uganda" },
    Region { code: "AE", name: "United Arab Emirates" },
    Region { code: "ZM", name: "Zambia" },
    Region { code: "ZW", name: "Zimbabwe" },

    // Asia Pacific
    Region { code: "AU", name: "Australia" },
    Region { code: "BD", name: "Bangladesh" },
    Region { code: "BT", name: "Bhutan" },
    Region { code: "BN", name: "Brunei Darussalam" },
    Region { code: "KH", name: "Cambodia" },
    Region { code: "CN", name: "China" },
    Region { code: "FJ", name: "Fiji" },
    Region { code: "HK", name: "Hong Kong" },
    Region { code: "ID", name: "Indonesia" },
    Region { code: "JP", name: "Japan" },
    Region { code: "KZ", name: "Kazakhstan" },
    Region { code: "KG", name: "Kyrgyzstan" },
    Region { code: "MO", name: "Macau" },
    Region { code: "MY", name: "Malaysia" },
    Region { code: "MV", name: "Maldives" },
    Region { code: "MN", name: "Mongolia" },
    Region { code: "MM", name: "Myanmar" },
    Region { code: "NP", name: "Nepal" },
    Region { code: "NZ", name: "New Zealand" },
    Region { code: "PH", name: "Philippines" },
    Region { code: "SG", name: "Singapore" },
    Region { code: "KR", name: "South Korea" },
    Region { code: "LK", name: "Sri Lanka" },
    Region { code: "TW", name: "Taiwan" },
    Region { code: "TJ", name: "Tajikistan" },
    Region { code: "TH", name: "Thailand" },
    Region { code: "TM", name: "Turkmenistan" },
    Region { code: "UZ", name: "Uzbekistan" },
    Region { code: "VN", name: "Vietnam" },

    // Europe
    Region { code: "AL", name: "Albania" },
    Region { code: "AM", name: "Armenia" },
    Region { code: "AT", name: "Austria" },
    Region { code: "AZ", name: "Azerbaijan" },
    Region { code: "BY", name: "Belarus" },
    Region { code: "BE", name: "Belgium" },
    Region { code: "BA", name: "Bosnia and Herzegovina" },
    Region { code: "BG", name: "Bulgaria" },
    Region { code: "HR", name: "Croatia" },
    Region { code: "CY", name: "Cyprus" },
    Region { code: "CZ", name: "Czech Republic" },
    Region { code: "DK", name: "Denmark" },
    Region { code: "EE", name: "Estonia" },
    Region { code: "FI", name: "Finland" },
    Region { code: "FR", name: "France" },
    Region { code: "GE", name: "Georgia" },
    Region { code: "DE", name: "Germany" },
    Region { code: "GR", name: "Greece" },
    Region { code: "HU", name: "Hungary" },
    Region { code: "IS", name: "Iceland" },
    Region { code: "IE", name: "Ireland" },
    Region { code: "IT", name: "Italy" },
    Region { code: "XK", name: "Kosovo" },
    Region { code: "LV", name: "Latvia" },
    Region { code: "LI", name: "Liechtenstein" },
    Region { code: "LT", name: "Lithuania" },
    Region { code: "LU", name: "Luxembourg" },
    Region { code: "MT", name: "Malta" },
    Region { code: "MD", name: "Moldova" },
    Region { code: "ME", name: "Montenegro" },
    Region { code: "NL", name: "Netherlands" },
    Region { code: "MK", name: "North Macedonia" },
    Region { code: "NO", name: "Norway" },
    Region { code: "PL", name: "Poland" },
    Region { code: "PT", name: "Portugal" },
    Region { code: "RO", name: "Romania" },
    Region { code: "RU", name: "Russia" },
    Region { code: "SK", name: "Slovakia" },
    Region { code: "SI", name: "Slovenia" },
    Region { code: "ES", name: "Spain" },
    Region { code: "SE", name: "Sweden" },
    Region { code: "CH", name: "Switzerland" },
    Region { code: "TR", name: "Turkey" },
    Region { code: "UA", name: "Ukraine" },
    Region { code: "GB", name: "United Kingdom" },

    // Latin America and the Caribbean
    Region { code: "AI", name: "Anguilla" },
    Region { code: "AG", name: "Antigua and Barbuda" },
    Region { code: "AR", name: "Argentina" },
    Region { code: "BS", name: "Bahamas" },
    Region { code: "BB", name: "Barbados" },
    Region { code: "BZ", name: "Belize" },
    Region { code: "BM", name: "Bermuda" },
    Region { code: "BO", name: "Bolivia" },
    Region { code: "BR", name: "Brazil" },
    Region { code: "VG", name: "British Virgin Islands" },
    Region { code: "KY", name: "Cayman Islands" },
    Region { code: "CL", name: "Chile" },
    Region { code: "CO", name: "Colombia" },
    Region { code: "CR", name: "Costa Rica" },
    Region { code: "DM", name: "Dominica" },
    Region { code: "DO", name: "Dominican Republic" },
    Region { code: "EC", name: "Ecuador" },
    Region { code: "SV", name: "El Salvador" },
    Region { code: "GD", name: "Grenada" },
    Region { code: "GT", name: "Guatemala" },
    Region { code: "GY", name: "Guyana" },
    Region { code: "HN", name: "Honduras" },
    Region { code: "JM", name: "Jamaica" },
    Region { code: "MX", name: "Mexico" },
    Region { code: "MS", name: "Montserrat" },
    Region { code: "NI", name: "Nicaragua" },
    Region { code: "PA", name: "Panama" },
    Region { code: "PY", name: "Paraguay" },
    Region { code: "PE", name: "Peru" },
    Region { code: "KN", name: "St. Kitts & Nevis" },
    Region { code: "LC", name: "St. Lucia" },
    Region { code: "VC", name: "St. Vincent & The Grenadines" },
    Region { code: "SR", name: "Suriname" },
    Region { code: "TT", name: "Trinidad & Tobago" },
    Region { code: "TC", name: "Turks & Caicos" },
    Region { code: "UY", name: "Uruguay" },
    Region { code: "VE", name: "Venezuela" },

    // North America
    Region { code: "CA", name: "Canada" },
    Region { code: "US", name: "United States" },
    Region { code: "PR", name: "Puerto Rico" },

    // Oceania
    Region { code: "FJ", name: "Fiji" },
    Region { code: "FM", name: "Micronesia" },
    Region { code: "NR", name: "Nauru" },
    Region { code: "NZ", name: "New Zealand" },
    Region { code: "PG", name: "Papua New Guinea" },
    Region { code: "SB", name: "Solomon Islands" },
    Region { code: "TO", name: "Tonga" },
    Region { code: "VU", name: "Vanuatu" },
];

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Pricing {
    region: String,
    amount: f64,
    currency: String,
    converted_amount: Option<f64>,
}

/// Map ISO currency code to symbol
fn currency_symbol(code: &str) -> &str {
    match code {
        // Africa, Middle East, India
        "DZD" => "دج",   // Algerian Dinar
        "AOA" => "Kz",   // Angolan Kwanza
        "XOF" => "Fr",   // CFA Franc BCEAO
        "BWP" => "P",    // Botswana Pula
        "XAF" => "Fr",   // CFA Franc BEAC
        "CDF" => "FC",   // Congolese Franc
        "EGP" => "£",    // Egyptian Pound
        "GHS" => "₵",    // Ghanaian Cedi
        "INR" => "₹",    // Indian Rupee
        "ILS" => "₪",    // Israeli New Shekel
        "JOD" => "JD",   // Jordanian Dinar
        "KES" => "KSh",  // Kenyan Shilling
        "KWD" => "KD",   // Kuwaiti Dinar
        "LRD" => "$",    // Liberian Dollar
        "LYD" => "LD",   // Libyan Dinar
        "MGA" => "Ar",   // Malagasy Ariary
        "MWK" => "MK",   // Malawian Kwacha
        "MRU" => "UM",   // Mauritanian Ouguiya
        "MUR" => "₨",    // Mauritian Rupee
        "MAD" => "د.م.", // Moroccan Dirham
        "MZN" => "MTn",  // Mozambican Metical
        "NAD" => "$",    // Namibian Dollar
        "NGN" => "₦",    // Nigerian Naira
        "OMR" => "ر.ع.", // Omani Rial
        "PKR" => "₨",    // Pakistani Rupee
        "QAR" => "ر.ق",  // Qatari Riyal
        "RWF" => "FRw",  // Rwandan Franc
        "SAR" => "ر.س",  // Saudi Riyal
        "SCR" => "SR",   // Seychellois Rupee
        "SLL" => "Le",   // Sierra Leonean Leone
        "ZAR" => "R",    // South African Rand
        "TZS" => "TZS",  // Tanzanian Shilling
        "TND" => "د.ت",  // Tunisian Dinar
        "UGX" => "USh",  // Ugandan Shilling
        "AED" => "د.إ",  // UAE Dirham
        "ZMW" => "ZK",   // Zambian Kwacha
        "ZWL" => "Z$",   // Zimbabwean Dollar
        // Asia Pacific
        "AUD" => "$",    // Australian Dollar
        "BDT" => "৳",    // Bangladeshi Taka
        "BTN" => "Nu.",  // Bhutanese Ngultrum
        "BND" => "B$",   // Brunei Dollar
        "KHR" => "៛",    // Cambodian Riel (suffix)
        "CNY" => "¥",    // Chinese Yuan
        "FJD" => "FJ$",  // Fiji Dollar
        "HKD" => "HK$",  // Hong Kong Dollar
        "IDR" => "Rp",   // Indonesian Rupiah
        "JPY" => "¥",    // Japanese Yen
        "KZT" => "₸",    // Kazakhstani Tenge
        "KGS" => "лв",   // Kyrgyzstani Som
        "MOP" => "P",    // Macanese Pataca
        "MYR" => "RM",   // Malaysian Ringgit
        "MVR" => "Rf.",  // Maldivian Rufiyaa
        "MNT" => "₮",    // Mongolian Tögrög
        "MMK" => "K",    // Myanmar Kyat (suffix)
        "NPR" => "₨",    // Nepalese Rupee
        "NZD" => "NZ$",  // New Zealand Dollar
        "PHP" => "₱",    // Philippine Peso
        "SGD" => "S$",   // Singapore Dollar
        "KRW" => "₩",    // South Korean Won
        "LKR" => "Rs",   // Sri Lankan Rupee
        "TWD" => "NT$",  // Taiwan Dollar
        "TJS" => "TJS",  // Tajikistani Somoni
        "THB" => "฿",    // Thai Baht
        "TMT" => "m",    // Turkmenistani Manat
        "UZS" => "so'm", // Uzbekistani so'm
        "VND" => "₫",    // Vietnamese Dong (suffix)
        // Europe
        "ALL" => "L",       // Albanian Lek (suffix)
        "AMD" => "AMD",     // Armenian Dram
        "EUR" => "€",       // Euro
        "AZN" => "₼",       // Azerbaijani Manat
        "BYN" => "Br",      // Belarusian Ruble (suffix)
        "BAM" => "KM",      // Bosnia Convertible Mark
        "BGN" => "лв",      // Bulgarian Lev
        "HRK" => "kn",      // Croatian Kuna
        "CZK" => "Kč",      // Czech Koruna
        "DKK" => "kr",      // Danish Krone
        "GEL" => "₾",       // Georgian Lari
        "HUF" => "HUF",     // Hungarian Forint (suffix; ISO)
        "ISK" => "kr",      // Icelandic Króna
        "MDL" => "L",       // Moldovan Leu (suffix)
        "MKD" => "ден",     // North Macedonian Denar (suffix)
        "NOK" => "kr",      // Norwegian Krone
        "PLN" => "zł",      // Polish Zloty
        "RON" => "lei",     // Romanian leu (suffix, decimals)
        "RUB" => "₽",       // Russian Ruble
        "SEK" => "kr",      // Swedish Krona
        "CHF" => "Fr.",     // Swiss Franc
        "TRY" => "₺",       // Turkish Lira
        "UAH" => "₴",       // Ukrainian Hryvnia
        "GBP" => "£",       // British Pound
        // Latin America / Caribbean
        "XCD" => "EC$",     // Eastern Caribbean Dollar
        "ARS" => "$",       // Argentine Peso
        "BSD" => "B$",      // Bahamian Dollar
        "BBD" => "Bds$",    // Barbadian Dollar
        "BZD" => "BZ$",     // Belize Dollar
        "BOB" => "Bs.",     // Bolivian Boliviano
        "BRL" => "R$",      // Brazilian Real
        "KYD" => "CI$",     // Cayman Islands Dollar
        "CLP" => "$",       // Chilean Peso
        "COP" => "$",       // Colombian Peso
        "CRC" => "₡",       // Costa Rican Colón
        "DOP" => "RD$",     // Dominican Peso
        "GTQ" => "Q",       // Guatemalan Quetzal
        "GYD" => "G$",      // Guyanese Dollar
        "HNL" => "L",       // Honduran Lempira
        "JMD" => "J$",      // Jamaican Dollar
        "MXN" => "$",       // Mexican Peso
        "NIO" => "C$",      // Nicaraguan Córdoba
        "PAB" => "B/.",     // Panamanian Balboa
        "PYG" => "₲",       // Paraguayan guaraní
        "PEN" => "S/.",     // Peruvian Sol
        "SRD" => "$",       // Surinamese dollar
        "TTD" => "TT$",     // Trinidad and Tobago Dollar
        "UYU" => "$",       // Uruguayan peso
        "VES" => "Bs.S",    // Venezuelan Bolívar Soberano
        // North America
        "CAD" => "$",       // Canadian Dollar
        "USD" => "$",       // US Dollar
        // Oceania
        "PGK" => "K",       // Papua New Guinea Kina
        "SBD" => "SI$",     // Solomon Islands dollar
        "TOP" => "T$",      // Tongan paʻanga (suffix)
        "VUV" => "VT",      // Vanuatu vatu (suffix)
        _ => "",
    }
}

fn currency_is_suffix(code: &str) -> bool {
    matches!(
        code,
        // Africa (suffix)
        "DZD" | "AOA" | "BWP" | "GHS" | "KES" | "LSL" | "LYD"
        | "MGA" | "MWK" | "MUR" | "MZN" | "NAD" | "NGN" | "RWF" | "SCR" | "SLL" | "SZL"
        | "TZS" | "UGX" | "XAF" | "XOF" | "ZAR" | "ZMW" | "ZWL" | "KMF" | "CFA" | "CDF"
        // Asia
        | "KHR" | "MMK" | "VND"
        // Oceania
        | "VUV" | "TOP"
        // Europe (suffix)
        | "ALL" | "MKD" | "MDL" | "RON" | "RSD" | "UAH" | "HUF" | "BYN"
    )
}

fn format_amount(amount: f64, code: &str) -> String {
    match code {
        // No decimals for these
        "JPY" | "KRW" | "VND" | "IDR" | "MMK" | "LAK" | "KHR" | "UGX" | "TZS" | "MWK" | "MGA"
        | "CDF" | "RWF" | "GNF" | "XOF" | "XAF" | "KMF" | "MZN" | "BIF" | "VUV" | "SLL" | "BYN"
            => format!("{:.0}", amount),
        // 3 decimals for some Gulf/Arab currencies
        "KWD" | "BHD" | "IQD" | "OMR" | "TND" | "LYD" | "JOD" => format!("{:.3}", amount),
        _ => format!("{:.2}", amount),
    }
}

fn format_price(amount: f64, code: &str) -> String {
    let symbol = currency_symbol(code);
    if currency_is_suffix(code) {
        if !symbol.is_empty() {
            format!("{} {}", format_amount(amount, code), symbol)
        } else {
            format!("{} {}", format_amount(amount, code), code)
        }
    } else {
        if !symbol.is_empty() {
            format!("{}{}", symbol, format_amount(amount, code))
        } else {
            format!("{} {}", code, format_amount(amount, code))
        }
    }
}

async fn get_conversion_rate(base: &str) -> Result<HashMap<String, f64>> {
    let url = format!("https://open.er-api.com/v6/latest/{}", base);
    let res = reqwest::get(&url).await?.json::<Value>().await?;
    let rates = res["rates"]
        .as_object()
        .context("Missing exchange rates in response")?
        .iter()
        .filter_map(|(k, v)| v.as_f64().map(|f| (k.clone(), f)))
        .collect();
    Ok(rates)
}

async fn fetch_app_name(app_id: &str, region_code: &str) -> Option<String> {
    let url = format!("https://apps.apple.com/{}/app/id{}", region_code, app_id);
    let html = reqwest::get(&url).await.ok()?.text().await.ok()?;
    let re = Regex::new(r#"<meta property="og:title" content="([^"]+)""#).ok()?;
    let caps = re.captures(&html)?;
    Some(caps.get(1)?.as_str().to_string())
}

async fn fetch_app_data(app_id: &str, region_code: &str) -> Result<Value> {
    let url = format!("https://apps.apple.com/{}/app/id{}", region_code, app_id);
    let html = reqwest::get(&url).await?.text().await?;
    let re = Regex::new(
        r#"<script[^>]*id="shoebox-media-api-cache-apps"[^>]*>([\s\S]*?)</script>"#
    )?;
    let caps = re
        .captures(&html)
        .context("Unable to find App Store cache script in HTML")?;
    let raw = caps[1].trim();
    let outer: Value = serde_json::from_str(raw)?;
    if let Some(map) = outer.as_object() {
        // Prefer entries with IAP
        for v in map.values() {
            if let Some(s) = v.as_str() {
                if let Ok(val) = serde_json::from_str::<Value>(s) {
                    if val["d"][0]["relationships"]["top-in-apps"]["data"].is_array() {
                        return Ok(val["d"][0].clone());
                    }
                }
            }
        }
        // Fallback
        for v in map.values() {
            if let Some(s) = v.as_str() {
                if let Ok(val) = serde_json::from_str::<Value>(s) {
                    return Ok(val["d"][0].clone());
                }
            }
        }
    }
    Err(anyhow!("Failed to extract App Store JSON data"))
}

async fn collect_iap_pricing(
    app_id: &str,
    region: &Region,
    selected: &Value,
    pricing: &Arc<Mutex<Vec<Pricing>>>,
) {
    if let Ok(app_data) = fetch_app_data(app_id, region.code).await {
        if let Some(arr) = app_data["relationships"]["top-in-apps"]["data"].as_array() {
            for item in arr {
                let attr = &item["attributes"];
                if attr["offerName"] == selected["attributes"]["offerName"] {
                    let offer = &attr["offers"][0];
                    let amount = offer["price"].as_f64().unwrap_or(0.0);
                    let currency = offer["currencyCode"].as_str().unwrap_or("").to_string();
                    let label = format_price(amount, &currency);
                    println!("{} → {} ({})", region.name, label.green(), currency);
                    pricing.lock().unwrap().push(Pricing {
                        region: region.name.to_string(),
                        amount,
                        currency,
                        converted_amount: None,
                    });
                    break;
                }
            }
        }
    }
}

async fn collect_base_app_pricing(
    app_id: &str,
    region: &Region,
    pricing: &Arc<Mutex<Vec<Pricing>>>,
) {
    // Try via shoebox JSON:
    if let Ok(app_data) = fetch_app_data(app_id, region.code).await {
        let attr = &app_data["attributes"];
        if let (Some(amount), Some(curr), Some(_label)) = (
            attr.get("price").and_then(|v| v.as_f64()),
            attr.get("currencyCode").and_then(|v| v.as_str()),
            attr.get("formattedPrice").and_then(|v| v.as_str())
        ) {
            let label = format_price(amount, curr);
            println!("{} → {} ({})", region.name, label.green(), curr);
            pricing.lock().unwrap().push(Pricing {
                region: region.name.to_string(),
                amount,
                currency: curr.to_string(),
                converted_amount: None,
            });
            return;
        }
    }

    // HTML fallback:
    let url = format!("https://apps.apple.com/{}/app/id{}", region.code, app_id);
    let html = match reqwest::get(&url).await {
        Ok(r) => r.text().await.unwrap_or_default(),
        Err(e) => {
            eprintln!("{}: error fetching page: {}", region.name, e);
            return;
        }
    };

    // 1) Try Open Graph price:
    let re_og_amt = Regex::new(r#"<meta property="og:price:amount" content="([^"]+)""#).unwrap();
    let re_og_cur = Regex::new(r#"<meta property="og:price:currency" content="([^"]+)""#).unwrap();
    if let (Some(am), Some(cur)) = (re_og_amt.captures(&html), re_og_cur.captures(&html)) {
        let amount_str = am.get(1).unwrap().as_str();
        let amount = amount_str.parse::<f64>().unwrap_or(0.0);
        let curr = cur.get(1).unwrap().as_str();
        let label = format_price(amount, curr);
        println!("{} → {} ({})", region.name, label.green(), curr);
        pricing.lock().unwrap().push(Pricing {
            region: region.name.to_string(),
            amount,
            currency: curr.to_string(),
            converted_amount: None,
        });
        return;
    }

    // 2) Try JSON-LD:
    let re_ld = Regex::new(r#"<script[^>]*type="application/ld\+json"[^>]*>([\s\S]*?)</script>"#).unwrap();
    let mut ld_amount = None;
    let mut ld_currency = None;
    if let Some(c) = re_ld.captures(&html) {
        let blob = c[1].trim();
        if let Ok(val) = serde_json::from_str::<Value>(blob) {
            if let Some(off) = val.get("offers") {
                ld_amount = off.get("price").and_then(|v| v.as_f64());
                ld_currency = off.get("priceCurrency").and_then(|v| v.as_str()).map(String::from);
            }
        }
    }
    if let (Some(amount), Some(curr)) = (ld_amount, ld_currency.as_deref()) {
        let label = format_price(amount, curr);
        println!("{} → {} ({})", region.name, label.green(), curr);
        pricing.lock().unwrap().push(Pricing {
            region: region.name.to_string(),
            amount,
            currency: curr.to_string(),
            converted_amount: None,
        });
        return;
    }

    // 3) HTML <li> legacy fallback:
    let re_html = Regex::new(
        r#"<li[^>]*class="inline-list__item[^"]*app-header__list__item--price"[^>]*>([^<]+)</li>"#
    ).unwrap();
    if let Some(cap) = re_html.captures(&html) {
        let raw = cap[1].replace("&nbsp;", " ").trim().to_string();
        // We can't extract amount/currency here, just print
        println!("{} → {}", region.name, raw.green());
        return;
    }

    eprintln!("{}: {}", region.name, "No price data available for this region.".bright_red());
}

async fn convert_prices(pricing: &mut [Pricing], rates: &HashMap<String, f64>) {
    for entry in pricing.iter_mut() {
        if let Some(rate) = rates.get(&entry.currency) {
            entry.converted_amount = Some((entry.amount / rate * 100.0).round() / 100.0);
        }
    }
    pricing.sort_by(|a, b| {
        a.converted_amount
            .partial_cmp(&b.converted_amount)
            .unwrap()
    });
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("{}", "App Store Price Preview".cyan().bold());
    println!("Check app or IAP pricing across multiple regions.\n");

    let link_or_id: String = Input::new()
        .with_prompt("App Store URL or App ID:")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.contains("apps.apple.com") || input.chars().all(char::is_numeric) {
                Ok(())
            } else {
                Err("Enter a valid App Store URL or numeric App ID.")
            }
        })
        .interact_text()?;

    let app_id = link_or_id.trim_start_matches("id").split("id").last().unwrap().to_string();

    let base_currency: String = Input::new()
        .with_prompt("Base currency (e.g., USD, EUR, SGD):")
        .validate_with(|input: &String| -> Result<(), &str> {
            if input.len() == 3 && input.chars().all(char::is_alphanumeric) {
                Ok(())
            } else {
                Err("Enter a three-letter currency code.")
            }
        })
        .interact_text()?;

    let base_currency = base_currency.to_uppercase();

    let code = &base_currency[..2];
    let base_region = *REGIONS
        .iter()
        .find(|r| r.code == code)
        .unwrap_or(&REGIONS[0]);
    let display_name = fetch_app_name(&app_id, base_region.code).await
        .unwrap_or_else(|| app_id.clone());

    let confirm = format!(
        "App: {} | Region: {} | Base Currency: {} — {}",
        display_name.green().bold(),
        base_region.name.green().bold(),
        base_currency.green().bold(),
        "continue?".italic()
    );
    if !Confirm::new().with_prompt(&confirm).default(true).interact()? {
        println!("Canceled.");
        return Ok(());
    }

    let primary_data = fetch_app_data(&app_id, base_region.code).await?;
    let iap_list = primary_data["relationships"]["top-in-apps"]["data"]
        .as_array().cloned().unwrap_or_default();

    let pricing = Arc::new(Mutex::new(Vec::new()));

    if !iap_list.is_empty() {
        let choices: Vec<String> = iap_list.iter().map(|p| {
            let a = &p["attributes"];
            format!(
                "{}: {}",
                a["name"].as_str().unwrap_or(""),
                a["offers"][0]["priceFormatted"].as_str().unwrap_or("")
            )
        }).collect();
        let colored_choices: Vec<String> = choices.iter().map(|c| c.green().to_string()).collect();
        let pick = Select::new()
            .with_prompt("Select an in-app purchase:")
            .items(&colored_choices)
            .default(0)
            .interact()?;
        let selected = iap_list[pick].clone();

        println!();

        let mut tasks = FuturesUnordered::new();

        // Base region first
        let pricing_clone = pricing.clone();
        let app_id_clone = app_id.clone();
        let selected_clone = selected.clone();
        let base_region_clone = base_region.clone();
        tasks.push(tokio::spawn(async move {
            collect_iap_pricing(&app_id_clone, &base_region_clone, &selected_clone, &pricing_clone).await;
        }));

        // All other regions
        for region in REGIONS.iter().filter(|r| r.code != base_region.code) {
            let pricing_clone = pricing.clone();
            let app_id = app_id.clone();
            let selected = selected.clone();
            let region = region.clone();
            tasks.push(tokio::spawn(async move {
                collect_iap_pricing(&app_id, &region, &selected, &pricing_clone).await;
            }));
        }

        // Collect all region tasks in parallel
        while let Some(_r) = tasks.next().await {}
    } else {
        println!("{}", "No in-app purchases found; retrieving base app prices…".yellow());
        println!();

        let mut tasks = FuturesUnordered::new();

        let pricing_clone = pricing.clone();
        let app_id_clone = app_id.clone();
        let base_region_clone = base_region.clone();
        tasks.push(tokio::spawn(async move {
            collect_base_app_pricing(&app_id_clone, &base_region_clone, &pricing_clone).await;
        }));

        for region in REGIONS.iter().filter(|r| r.code != base_region.code) {
            let pricing_clone = pricing.clone();
            let app_id = app_id.clone();
            let region = region.clone();
            tasks.push(tokio::spawn(async move {
                collect_base_app_pricing(&app_id, &region, &pricing_clone).await;
            }));
        }

        while let Some(_r) = tasks.next().await {}
    }

    let mut pricing = Arc::try_unwrap(pricing).unwrap().into_inner().unwrap();

    if pricing.is_empty() {
        eprintln!("{}", "No pricing data available.".yellow());
        return Ok(());
    }

    let rates = get_conversion_rate(&base_currency).await?;
    convert_prices(&mut pricing, &rates).await;

    println!("");
    let headers = vec![
        "Region".to_string(),
        "Price".to_string(),
        "Currency".to_string(),
        format!("Converted ({})", base_currency),
    ];
    let rows: Vec<Vec<String>> = pricing.iter().map(|e| {
        vec![
            e.region.clone(),
            format_price(e.amount, &e.currency),
            e.currency.clone(),
            e.converted_amount
                .map_or("N/A".into(), |v| format_price(v, &base_currency))
        ]
    }).collect();

    let mut widths = headers.iter().map(|h| h.len()).collect::<Vec<_>>();
    for row in &rows {
        for (i, cell) in row.iter().enumerate() {
            widths[i] = widths[i].max(cell.len());
        }
    }
    let print_border = |widths: &[usize]| {
        print!("+");
        for w in widths {
            print!("{:-^1$}+", "-", w + 2);
        }
        println!();
    };
    print_border(&widths);
    print!("|");
    for (i, h) in headers.iter().enumerate() {
        print!(" {:^width$} |", h, width = widths[i]);
    }
    println!();
    print_border(&widths);
    for row in rows {
        print!("|");
        for (i, cell) in row.iter().enumerate() {
            print!(" {:^width$} |", cell, width = widths[i]);
        }
        println!();
    }
    print_border(&widths);

    Ok(())
}

