use serde::{Deserialize, Serialize};
use sqlx::{prelude::*, Sqlite};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyUuid(pub Uuid);

impl MyUuid {
    pub fn new() -> Self {
        MyUuid(Uuid::now_v7())
    }
}

impl From<Uuid> for MyUuid {
    fn from(uuid: Uuid) -> Self {
        MyUuid(uuid)
    }
}

impl ToString for MyUuid {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl Type<Sqlite> for MyUuid {
    fn type_info() -> sqlx::sqlite::SqliteTypeInfo {
        <String as Type<Sqlite>>::type_info()
    }
}

impl<'r> Decode<'r, Sqlite> for MyUuid {
    fn decode(value: sqlx::sqlite::SqliteValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
        let value = <String as Decode<Sqlite>>::decode(value)?;
        Uuid::parse_str(&value).map_err(Into::into).map(MyUuid)
    }
}

impl Encode<'_, Sqlite> for MyUuid {
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        <String as Encode<Sqlite>>::encode(self.to_string(), buf)
    }
}
