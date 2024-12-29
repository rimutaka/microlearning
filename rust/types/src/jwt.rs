use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tracing::info;

// these N and E values are exracted manually from JWK found at https://dev-lbpjc402mmk4uxbs.us.auth0.com/.well-known/jwks.json
// match the key by the kid field in the header of the JWT token
const JWK_N: &str ="v6TBTM87ZGheW4dN04giRLKS1cdIHnWV13UgOhsnIJSNDKgp842nbF3NnTYY1hiR8e_umkuzzkWeW8FR_R0OFVozDACaWakqfJB8kcx3A0oVAuiZpQgvt99mkn7TpqujdHkbA_xD-V0Or7mlApX4ZCiCdzyuU_AGQQ1yYsSaDF9paPCi1sTna5-xBHRsDgYOwrx49nhIwrohhHUgecRoYCV0Gs9gVMRLinVIUyvHsHW-qtTnpe3lYxLS_i6-vkLi16eAQ5ocnE0ZRniWeGOgmKM5MjZXAL23VZ_w7D_y5FbIxricz_9BF9R1Kmu8M3nH_4kyqV1wE7LP1Q4F_vXEgw";
const JWK_E: &str = "AQAB";
// this value is ClientID in the application config on Auth0
const AUDIENCE: &str = "p2xjvyoxb8HoKSt1QNDx7CQ8Ka2lXgUJ";

#[derive(Debug, Deserialize)]
struct Claims {
    email: String,
    email_verified: bool,
}

/// Details extracted from the JWT token.
#[derive(Clone, Debug)]
pub struct JwtUser {
    /// The user email in lower case from the given JWT token
    pub email: String,
    /// A hash of the email with a salt for a public user ID
    pub email_hash: String,
}

/// Returns user details, if the token is valid :
/// Otherwise returns None.
/// All errors are logged inside the function.
pub fn get_email_from_token(token: &str) -> Option<JwtUser> {
    // do we have a token?
    if token.is_empty() {
        info!("No token provided: empty");
        return None;
    }

    // try decoding the token
    let header = match decode_header(token) {
        Ok(v) => v,
        Err(e) => {
            info!("Token: {token}");
            info!("Error decoding header: {:?}", e);
            return None;
        }
    };

    // try creating a decoding key
    let decoding_key = match DecodingKey::from_rsa_components(JWK_N, JWK_E) {
        Ok(v) => v,
        Err(e) => {
            info!("Token: {token}");
            info!("Error creating decoding key: {:?}. It's a bug.", e);
            return None;
        }
    };

    // prepare the validation struct for validation as part of the decode
    let validation = {
        let mut validation = Validation::new(header.alg);
        validation.set_audience(&[AUDIENCE]);
        validation.validate_exp = true;
        validation
    };

    // decode and validate in one step
    let claims = match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(v) => v.claims,
        Err(e) => {
            info!("Token: {token}");
            info!("Error decoding token: {:?}", e);
            return None;
        }
    };

    // log!("{:#?}", decoded_token);

    // check if we have an email
    if claims.email.is_empty() {
        info!("No email found in token");
        return None;
    }
    if !claims.email_verified {
        info!("Token: {token}");
        info!("Unverified email: {}", claims.email);
        return None;
    }

    // emails must be normalized to lower case
    let email = claims.email.to_ascii_lowercase();

    // id hashing with the salt
    // do not change the salt ever without converting the existing user IDs
    // the salt is not secret
    // it is used to prevent use of rainbow tables to discover emails
    let mut hasher = Sha256::new();
    hasher.update(crate::PUBLIC_SALT);
    hasher.update(email.clone());
    let email_hash = hex::encode(hasher.finalize());

    Some(JwtUser { email, email_hash })
}
