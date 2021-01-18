use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
// use std::collections::HashMap;
use crate::{CountryCode, Secret};

/// The response from performing a list `institutions` request.
#[derive(Serialize, Deserialize, Debug)]
pub struct ListInstitutionsResponse {
    /// The financial institution accounts associated with the Item.
    #[serde(default)]
    institutions: Vec<Institution>,
    request_id: String,
}

/// The request fields to perform a get `institution`
#[derive(Serialize)]
pub struct GetInstitutionRequest {
    /// The ID of the institution to get details about
    pub institution_id: String,

    /// Plaid Client ID
    pub client_id: String,

    /// Plaid API Secret
    pub secret: Secret,

    /// Specify an array of Plaid-supported country codes this institution supports, using the
    /// ISO-3166-1 alpha-2 country code standard.
    pub country_codes: Vec<CountryCode>,

    /// Specifies optional parameters for /institutions/get_by_id. If provided, must not be null.
    #[serde(serialize_with = "serialize_options")]
    pub options: Vec<InstitutionOption>,
}

/// The response from performing a get `institutions` request.
#[derive(Serialize, Deserialize, Debug)]
pub struct GetInstitutionResponse {
    /// The financial institution accounts associated with the Item.
    institution: Institution,
    request_id: String,
}

impl GetInstitutionResponse {
    /// Public getter for `institution`.
    pub fn institution(&self) -> Institution {
        self.institution.clone()
    }

    /// Public getting for `request_id`
    pub fn request_id(&self) -> String {
        self.request_id.clone()
    }
}

/// Optional list of `options` to retrieve optional metadata about Plaid institutions.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum InstitutionOption {
    /// When true, return an institution's logo, brand color, and URL. When available, the bank's
    /// logo is returned as a base64 encoded 152x152 PNG, the brand color is in hexadecimal format.
    /// The default value is false.
    ///
    /// Note that Plaid does not own any of the logos shared by the API and that by accessing or
    /// using these logos, you agree that you are doing so at your own risk and will, if necessary,
    /// obtain all required permissions from the appropriate rights holders and adhere to any
    /// applicable usage guidelines. Plaid disclaims all express or implied warranties with
    /// respect to the logos.
    IncludeOptionalMetadata,

    /// If true, the response will include status information about the institution.
    /// Default value is false.
    IncludeStatus,
}

/// Metadata about a requested `Institution`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Institution {
    ///Unique identifier for the institution
    institution_id: String,

    ///The official name of the institution
    name: String,

    ///A list of the Plaid products supported by the institution
    ///Possible values: assets, auth, balance, identity, investments, liabilities, payment_initiation, transactions
    products: Vec<String>,

    ///A list of the Plaid products supported by the institution
    country_codes: Vec<CountryCode>,

    ///The URL for the institution's website
    url: Option<String>,

    ///Hexadecimal representation of the primary color used by the institution
    primary_color: Option<String>,

    ///Base64 encoded representation of the institution's logo
    logo: Option<String>,

    ///A partial list of routing numbers associated with the institution. This list is provided
    ///for the purpose of looking up institutions by routing number. It is not comprehensive and
    ///should never be used as a complete list of routing numbers for an institution.
    routing_numbers: Vec<String>,

    ///Indicates that the institution has an OAuth login flow. This is primarily relevant to
    ///institutions with European country codes.
    // oauth: bool,

    ///The status of an institution is determined by the health of its Item logins, Transactions
    ///updates, Auth requests, Balance requests, and Identity requests. A login attempt is conducted
    ///during the initial Item add in Link. If there is not enough traffic to accurately calculate
    ///an institution's status, Plaid will return null rather than potentially inaccurate data.

    ///Institution status is accessible in the Dashboard and via the API using
    ///the /institutions/get_by_id endpoint with the include_status option set to true. Note that
    ///institution status is not available in the Sandbox environment.
    oauth: bool,
    // TODO: finish this
    // status: HashMap<String, Status>,
}

impl Institution {
    /// Public getter for the `institution_id`
    pub fn id(&self) -> String {
        self.institution_id.clone()
    }

    /// Public getter for Plaid's institution `name`.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Public getter for Plaid's institution `logo`.
    pub fn logo(&self) -> Option<String> {
        self.logo.clone()
    }

    /// Public getter for Plaid's instituion `url`.
    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }
}

fn serialize_options<S>(x: &Vec<InstitutionOption>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = s.serialize_map(Some(x.len()))?;

    for option in x {
        let _ = map.serialize_entry(option, &true);
    }
    map.end()
}
