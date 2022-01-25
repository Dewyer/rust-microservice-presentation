#[macro_export]
macro_rules! crud_repository {
    ($name: ident,$dbname: ident, $table: ident, $trec: ty, $tnew: ty, $nameStr: expr) => {
        pub trait $name: Send + Sync {
            fn list(
                &self,
                tm: &crate::modules::common::infrastructure::transaction::ITransaction,
            ) -> Result<Vec<$trec>, crate::modules::common::errors::database::DatabaseError>;

            fn insert(
                &self,
                new_rec: &$tnew,
                tm: &crate::modules::common::infrastructure::transaction::ITransaction,
            ) -> Result<$trec, crate::modules::common::errors::database::DatabaseError>;

            fn find_by_id(
                &self,
                id: uuid::Uuid,
                tm: &crate::modules::common::infrastructure::transaction::ITransaction,
            ) -> Result<$trec, crate::modules::common::errors::database::DatabaseError>;

            fn clear(
                &self,
                td: &crate::modules::common::infrastructure::transaction::ITransaction,
            ) -> Result<usize, crate::modules::common::errors::database::DatabaseError>;
        }

        pub struct $dbname {}

        impl $dbname {
            pub fn new() -> Self {
                Self {}
            }
        }

        impl $name for $dbname {
            fn list(
                &self,
                tm: &crate::modules::common::infrastructure::transaction::ITransaction,
            ) -> Result<Vec<$trec>, crate::modules::common::errors::database::DatabaseError> {
                $table::table
                    .select($table::all_columns)
                    .load::<$trec>(tm.get_db_connection())
                    .map_err(|_| {
                        crate::modules::common::errors::database::DatabaseError::RepositoryError(
                            format!("Geting {} failed!", $nameStr),
                        )
                    })
            }

            fn insert(
                &self,
                new_rec: &$tnew,
                tm: &crate::modules::common::infrastructure::transaction::ITransaction,
            ) -> Result<$trec, crate::modules::common::errors::database::DatabaseError> {
                diesel::insert_into($table::table)
                    .values(new_rec)
                    .get_result(tm.get_db_connection())
                    .map_err(|_| {
                        crate::modules::common::errors::database::DatabaseError::RepositoryError(
                            format!("Couldn't insert {}!", $nameStr),
                        )
                    })
            }

            fn find_by_id(
                &self,
                id: uuid::Uuid,
                tm: &crate::modules::common::infrastructure::transaction::ITransaction,
            ) -> Result<$trec, crate::modules::common::errors::database::DatabaseError> {
                $table::table
                    .select($table::all_columns)
                    .filter($table::id.eq(id))
                    .first::<$trec>(tm.get_db_connection())
                    .map_err(|_| {
                        crate::modules::common::errors::database::DatabaseError::RepositoryError(
                            format!("{} not found!", $nameStr),
                        )
                    })
            }

            fn clear(
                &self,
                td: &crate::modules::common::infrastructure::transaction::ITransaction,
            ) -> Result<usize, crate::modules::common::errors::database::DatabaseError> {
                diesel::delete($table::table)
                    .execute(td.get_db_connection())
                    .map_err(|_| {
                        crate::modules::common::errors::database::DatabaseError::RepositoryError(
                            format!("{} table cant be cleared!", $nameStr),
                        )
                    })
            }
        }
    };
}
