use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;

pub fn connect_mysql() {
  let url = "mysql://root:123456@localhost:3306/MYDB";
  let pool = Pool::new(url).unwrap();
  let mut conn = pool.get_conn().unwrap();
}