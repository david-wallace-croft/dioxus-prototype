use oauth2::basic::BasicErrorResponseType;
use oauth2::basic::BasicTokenType;
use oauth2::revocation::StandardRevocableToken;
use openidconnect::core::*;
use openidconnect::*;

pub type AliasClient = Client<
  EmptyAdditionalClaims,
  CoreAuthDisplay,
  CoreGenderClaim,
  CoreJweContentEncryptionAlgorithm,
  CoreJwsSigningAlgorithm,
  CoreJsonWebKeyType,
  CoreJsonWebKeyUse,
  CoreJsonWebKey,
  CoreAuthPrompt,
  StandardErrorResponse<BasicErrorResponseType>,
  StandardTokenResponse<
    IdTokenFields<
      EmptyAdditionalClaims,
      EmptyExtraTokenFields,
      CoreGenderClaim,
      CoreJweContentEncryptionAlgorithm,
      CoreJwsSigningAlgorithm,
      CoreJsonWebKeyType,
    >,
    BasicTokenType,
  >,
  BasicTokenType,
  StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
  StandardRevocableToken,
  StandardErrorResponse<RevocationErrorResponseType>,
>;
