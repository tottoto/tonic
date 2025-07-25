use prost::{DecodeError, Message};
use prost_types::Any;

use crate::{richer_error::FromAnyRef, LocalizedMessage};

use super::super::{pb, FromAny, IntoAny};

/// Used at the `field_violations` field of the [`BadRequest`] struct.
/// Describes a single bad request field.
#[derive(Clone, Debug, Default)]
pub struct FieldViolation {
    /// Path leading to a field in the request body. Value should be a
    /// sequence of dot-separated identifiers that identify a protocol buffer
    /// field.
    pub field: String,

    /// Description of why the field is bad.
    pub description: String,

    /// The reason of the field-level error. Value should be a
    /// SCREAMING_SNAKE_CASE error identifier from the domain of the API
    /// service.
    pub reason: String,

    /// A localized version of the field-level error.
    pub localized_message: Option<LocalizedMessage>,
}

impl FieldViolation {
    /// Creates a new [`FieldViolation`] struct.
    pub fn new(field: impl Into<String>, description: impl Into<String>) -> Self {
        FieldViolation {
            field: field.into(),
            description: description.into(),
            ..Default::default()
        }
    }
}

impl From<pb::bad_request::FieldViolation> for FieldViolation {
    fn from(value: pb::bad_request::FieldViolation) -> Self {
        FieldViolation {
            field: value.field,
            description: value.description,
            reason: value.reason,
            localized_message: value.localized_message.map(Into::into),
        }
    }
}

impl From<FieldViolation> for pb::bad_request::FieldViolation {
    fn from(value: FieldViolation) -> Self {
        pb::bad_request::FieldViolation {
            field: value.field,
            description: value.description,
            ..Default::default()
        }
    }
}

/// Used to encode/decode the `BadRequest` standard error message described in
/// [error_details.proto]. Describes violations in a client request. Focuses
/// on the syntactic aspects of the request.
///
/// [error_details.proto]: https://github.com/googleapis/googleapis/blob/master/google/rpc/error_details.proto
#[derive(Clone, Debug)]
pub struct BadRequest {
    /// Describes all field violations of the request.
    pub field_violations: Vec<FieldViolation>,
}

impl BadRequest {
    /// Type URL of the `BadRequest` standard error message type.
    pub const TYPE_URL: &'static str = "type.googleapis.com/google.rpc.BadRequest";

    /// Creates a new [`BadRequest`] struct.
    pub fn new(field_violations: impl Into<Vec<FieldViolation>>) -> Self {
        BadRequest {
            field_violations: field_violations.into(),
        }
    }

    /// Creates a new [`BadRequest`] struct with a single [`FieldViolation`] in
    /// `field_violations`.
    pub fn with_violation(field: impl Into<String>, description: impl Into<String>) -> Self {
        BadRequest {
            field_violations: vec![FieldViolation {
                field: field.into(),
                description: description.into(),
                ..Default::default()
            }],
        }
    }

    /// Adds a [`FieldViolation`] to [`BadRequest`]'s `field_violations`.
    pub fn add_violation(
        &mut self,
        field: impl Into<String>,
        description: impl Into<String>,
    ) -> &mut Self {
        self.field_violations.append(&mut vec![FieldViolation {
            field: field.into(),
            description: description.into(),
            ..Default::default()
        }]);
        self
    }

    /// Returns `true` if [`BadRequest`]'s `field_violations` vector is empty,
    /// and `false` if it is not.
    pub fn is_empty(&self) -> bool {
        self.field_violations.is_empty()
    }
}

impl IntoAny for BadRequest {
    fn into_any(self) -> Any {
        let detail_data: pb::BadRequest = self.into();

        Any {
            type_url: BadRequest::TYPE_URL.to_string(),
            value: detail_data.encode_to_vec(),
        }
    }
}

impl FromAny for BadRequest {
    #[inline]
    fn from_any(any: Any) -> Result<Self, DecodeError> {
        FromAnyRef::from_any_ref(&any)
    }
}

impl FromAnyRef for BadRequest {
    fn from_any_ref(any: &Any) -> Result<Self, DecodeError> {
        let buf: &[u8] = &any.value;
        let bad_req = pb::BadRequest::decode(buf)?;

        Ok(bad_req.into())
    }
}

impl From<pb::BadRequest> for BadRequest {
    fn from(value: pb::BadRequest) -> Self {
        BadRequest {
            field_violations: value.field_violations.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<BadRequest> for pb::BadRequest {
    fn from(value: BadRequest) -> Self {
        pb::BadRequest {
            field_violations: value.field_violations.into_iter().map(Into::into).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{FromAny, IntoAny};
    use super::BadRequest;

    #[test]
    fn gen_bad_request() {
        let mut br_details = BadRequest::new(Vec::new());
        let formatted = format!("{br_details:?}");

        let expected = "BadRequest { field_violations: [] }";

        assert!(
            formatted.eq(expected),
            "empty BadRequest differs from expected result"
        );

        assert!(
            br_details.is_empty(),
            "empty BadRequest returns 'false' from .is_empty()"
        );

        br_details
            .add_violation("field_a", "description_a")
            .add_violation("field_b", "description_b");

        let formatted = format!("{br_details:?}");

        let expected_filled = "BadRequest { field_violations: [FieldViolation { field: \"field_a\", description: \"description_a\", reason: \"\", localized_message: None }, FieldViolation { field: \"field_b\", description: \"description_b\", reason: \"\", localized_message: None }] }";

        assert!(
            formatted.eq(expected_filled),
            "filled BadRequest differs from expected result"
        );

        assert!(
            !br_details.is_empty(),
            "filled BadRequest returns 'true' from .is_empty()"
        );

        let gen_any = br_details.into_any();
        let formatted = format!("{gen_any:?}");

        let expected = "Any { type_url: \"type.googleapis.com/google.rpc.BadRequest\", value: [10, 24, 10, 7, 102, 105, 101, 108, 100, 95, 97, 18, 13, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 95, 97, 10, 24, 10, 7, 102, 105, 101, 108, 100, 95, 98, 18, 13, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 95, 98] }";

        assert!(
            formatted.eq(expected),
            "Any from filled BadRequest differs from expected result"
        );

        let br_details = match BadRequest::from_any(gen_any) {
            Err(error) => panic!("Error generating BadRequest from Any: {error:?}"),
            Ok(from_any) => from_any,
        };

        let formatted = format!("{br_details:?}");

        assert!(
            formatted.eq(expected_filled),
            "BadRequest from Any differs from expected result"
        );
    }
}
