use crate::{name_data_to_str, PgOid};

impl crate::FormData_pg_attribute {
    pub fn name(&self) -> &str {
        name_data_to_str(&self.attname)
    }

    pub fn type_oid(&self) -> PgOid {
        PgOid::from(self.atttypid)
    }

    pub fn num(&self) -> i16 {
        self.attnum
    }

    pub fn is_dropped(&self) -> bool {
        self.attisdropped
    }

    pub fn rel_id(&self) -> crate::Oid {
        self.attrelid
    }
}
