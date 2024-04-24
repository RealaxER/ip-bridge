use crate::devices::Device;
use crate::error::BridgeIpErr;
use rusqlite::{params, Connection, Result};

pub struct SqlDirver {
    pub conn: Connection,
}

impl SqlDirver {
    pub async fn new(path: String) -> Self {
        let conn = Connection::open(path.to_string())
            .map_err(|_| BridgeIpErr::CreateSqlErr)
            .unwrap();
        SqlDirver { conn: conn }
    }

    pub async fn connect(&mut self) -> Result<(), BridgeIpErr> {
        // Tạo một bảng mới nếu nó chưa tồn tại
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                macbr TEXT NOT NULL,
                hash TEXT NOT NULL
            )",
            [],
        )
            .map_err(|_| BridgeIpErr::CreateSqlErr)?;

        Ok(())
    }

    pub async fn add_device(&mut self, device: Device) -> Result<(), BridgeIpErr>  {

        let hash_json = serde_json::to_string(&device.hash).unwrap();
        self.conn.execute(
            "INSERT INTO users (macbr, hash) VALUES (?1, ?2)",
            params![device.mac, hash_json],
        )
        .map_err(|_| BridgeIpErr::AddSqlErr)
        .unwrap();

        Ok(())
    }

    pub async fn delete_device(&mut self, mac: String) -> Result<(), BridgeIpErr> {
        
        self.conn.execute("DELETE FROM users WHERE macbr = ?1", params![mac])
        .map_err(|_| BridgeIpErr::DeleteSqlErr)
        .unwrap();

        Ok(())
    }

    pub async fn clear(&mut self) -> Result<(), BridgeIpErr> {
        // Xóa toàn bộ dữ liệu từ bảng `users`
        self.conn.execute("DELETE FROM users", [])
        .map_err(|_| BridgeIpErr::DeleteSqlErr).unwrap();

        Ok(())
    }


    pub async fn get_devices(&mut self) -> Result<Vec<Device>> {
        let mut stmt = self.conn.prepare("SELECT macbr, hash FROM users")?;
        let user_iter = stmt.query_map([], |row| {
            Ok(Device {
                mac: row.get(0)?,
                hash: serde_json::from_str(&row.get::<usize, String>(1)?).unwrap(),
            })
        })?;
    
        let mut devices = Vec::new();
        for user in user_iter {
            devices.push(user?);
        }
        Ok(devices)
    }
    
}
