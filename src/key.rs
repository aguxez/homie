use crate::db::conn::Pool;
use crate::schema::client_keys;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::sql_query;
use diesel::sql_types::{Integer, Text};

#[derive(QueryableByName, Debug)]
pub struct ClientKey {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub key: String,
    #[diesel(sql_type = Integer)]
    pub is_active: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = client_keys)]
pub struct NewClient {
    #[diesel(sql_type = Text)]
    pub key: String,
    #[diesel(sql_type = Integer)]
    pub is_active: i32,
}

// booleans booleans...
static SQLITE_TRUE: i32 = 1;
static SQLITE_FALSE: i32 = 0;

// * I have no idea why the DSL doesn't work and to keep things simple I just decided
// to write the raw query...
/// Returns a single active key. When we insert a new key we invalidate any other active key
pub fn get_active_key(pool: &mut Pool) -> Vec<ClientKey> {
    sql_query("SELECT * FROM client_keys WHERE is_active = 1 LIMIT 1")
        .load(&mut pool.conn)
        .expect("Could not load client keys")
}

/// Inserts a given key and marks any other active key as inactive
pub fn insert_key(key: String, pool: &mut Pool) -> Result<usize, Error> {
    let new_client = NewClient {
        key,
        is_active: SQLITE_TRUE,
    };
    // These are pretty standard operations so unless an error ocurred in the tx we will just
    // ignore the return value of this whole operation
    pool.conn.transaction::<usize, Error, _>(|conn| {
        diesel::update(client_keys::table.filter(client_keys::is_active.eq(SQLITE_TRUE)))
            .set(client_keys::is_active.eq(SQLITE_FALSE))
            .execute(conn)
            .expect("Error updating keys to false");

        let inserts = diesel::insert_into(client_keys::table)
            .values(&new_client)
            .execute(conn)
            .expect("Error saving new client key");

        Ok(inserts)
    })
}
