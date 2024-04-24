#[derive(Debug, PartialEq, Clone)]
pub enum BridgeIpErr {
    MqttErr,
    MqttConvertJsoErr,

    TimoutErr,
    RepeatErr,
    OpenFileErr,
    ReadFileErr,
    WriteFileErr,
    ConvertTempErr,

    AddDeviceErr,
    DeviceNullErr,

    CreateSqlErr,
    AddSqlErr,
    GetSqlErr,
    DeleteSqlErr,

    DeviceNotFound,

    GetMacErr,
}
