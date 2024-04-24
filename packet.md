
# Packet install from app 
# Bảng tin thêm thiết bị từ device -> hc

mosquitto_pub -h localhost -t component/zigbee/A4:C1:38:1F:22:4C:97:F7/config -u component -P 123 -m '{
  "cmd": "post",
  "objects": [
    {
      "bridge_key": "zigbee",
      "data": [
        {
          "attr": {
            "deviceInfo": {
              "Manufacturer": "Lumi R&D",
              "ModelId": "LM-2I2O"
            },
            "sceneConfig": {
              "input": {
                "mode": 2
              },
              "link_state": true,
              "output": {
                "delay": 1,
                "mode": 0
              },
              "state_default": 2
            },
            "sceneSetting": [
              {
                "name": "link_state"
              },
              {
                "name": "state_default"
              },
              {
                "max": 3,
                "min": 0,
                "name": "input_mode"
              },
              {
                "max": 5,
                "min": 0,
                "name": "output_mode"
              }
            ]
          },
          "brigde_key": "zigbee",
          "config": {},
          "deviceInfo": {
            "ApplicationVer": "",
            "DataCode": "",
            "HardwareVer": "",
            "Manufacturer": "Lumi R&D",
            "ModelId": "LM-INOUT",
            "ZigbeeProtocolVer": "3.0",
            "ZigbeeStackVer": ""
          },
          "hash": "zigbee-A4:C1:38:1F:22:4C:97:F7-1",
          "mac": "A4:C1:38:1F:22:4C:97:F7",
          "macdev": "A4:C1:38:1F:22:4C:97:F7",
          "traits": [
            {
              "is_main": true,
              "name": "OnOff"
            },
            {
              "is_main": false,
              "max": 5,
              "min": 0,
              "name": "Act",
              "options": [8, 9, 7]
            }
          ],
          "type": "INPUTV2"
        }
      ],
      "type": "devices_local"
    }
  ],
  "reqid": "MSignUYdArlZnvU",
  "source": "core"
}'



mosquitto_pub -h localhost -t /component/deviceIP/A4:C1:38:84:90:07:F3:A1/config -u component -P 123 -m '{
  "cmd": "post",
  "objects": [
    {
      "bridge_key": "zigbee",
      "data": [
        {
          "attr": {
            "deviceInfo": {
              "Manufacturer": "Lumi R&D",
              "ModelId": "LM-2I2O"
            },
            "sceneConfig": {
              "input": {
                "mode": 2
              },
              "link_state": true,
              "output": {
                "delay": 1,
                "mode": 0
              },
              "state_default": 2
            },
            "sceneSetting": [
              {
                "name": "link_state"
              },
              {
                "name": "state_default"
              },
              {
                "max": 3,
                "min": 0,
                "name": "input_mode"
              },
              {
                "max": 5,
                "min": 0,
                "name": "output_mode"
              }
            ]
          },
          "brigde_key": "zigbee",
          "config": {},
          "deviceInfo": {
            "ApplicationVer": "",
            "DataCode": "",
            "HardwareVer": "",
            "Manufacturer": "Lumi R&D",
            "ModelId": "LM-INOUT",
            "ZigbeeProtocolVer": "3.0",
            "ZigbeeStackVer": ""
          },
          "hash": "zigbee-A4:C1:38:1F:22:4C:97:F7-1",
          "mac": "A4:C1:38:1F:22:4C:97:F7",
          "macdev": "A4:C1:38:1F:22:4C:97:F7",
          "traits": [
            {
              "is_main": true,
              "name": "OnOff"
            },
            {
              "is_main": false,
              "max": 5,
              "min": 0,
              "name": "Act",
              "options": [8, 9, 7]
            }
          ],
          "type": "INPUTV2"
        }
      ],
      "type": "devices_local"
    }
  ],
  "reqid": "MSignUYdArlZnvU",
  "source": "deviceIp"
}'

# Bảng tin status 
```bash
mosquitto_pub -h localhost -t component/zigbee/status -u component -P 123 -m '{
  "cmd": "status",
  "objects": [
    {
      "bridge_key": "deviceIP",
      "type": "devices",
      "data": [
        {
          "hash": "deviceIP-A4:C1:38:1F:22:4C:97:F7-1",
          "type": "SWITCH",
          "states": {
            "OnOff": {
              "on": false
            }
          }
        }
      ]
    }
  ],
  "reqid": "abcxyz",
  "source": "core"
}'
```

# Bảng tin status
mosquitto_pub -h localhost -t component/zigbee/A4:C1:38:84:90:07:F3:A1/status -u component -P 123 -m '{
  "cmd": "status",
  "objects": [
    {
      "bridge_key": "deviceIP",
      "type": "devices",
      "data": [
        {
          "hash": "deviceIP-A4:C1:38:84:90:07:F3:A1-1",
          "type": "SWITCH",
          "states": {
            "OnOff": {
              "on": false
            }
          }
        },
        {
          "hash": "deviceIP-A4:C1:38:84:90:07:F3:A1-3",
          "type": "SWITCH",
          "states": {
            "OnOff": {
              "on": false
            }
          }
        }
      ]
    }
  ],
  "reqid": "abcxyz",
  "source": "deviceIp"
}'




# Bảng tin status khi control 
mosquitto_pub -h localhost -t /component/deviceIP/A4:C1:38:84:90:07:F3:A1/status -m '{
  "cmd": "status",
  "control_source": {
    "id": "android-SM A736B-tainv@lumi.biz-1695175429715-23",
    "previous_control_reqid": "",
    "type": "app"
  },
  "objects": [
    {
      "data": [
        {
          "devid": "1_deviceIP-F4:12:FA:CF:4E:B4-1",
          "hash": "F4:12:FA:CF:4E:B4-1",
          "states": {
            "OnOff": {
              "on": false
            }
          },
          "status": 1
        }
      ],
      "type": "devices"
    }
  ],
  "reqid": "android-SM A736B-tainv@lumi.biz-1695175429715-23",
  "source": "deviceIP"
}'





# Bản tin keepalive từ bridge ip

```bash
mosquitto_pub -h localhost -t component/keepalive/bridge_ip -m '{
    "cmd": "status",
    "objects": [
        {
            "bridge_key": "wifi",
            "data": [],
            "type": "keepalive"
        }
    ],
    "reqid": "LKHlMfunx1Ed",
    "source": "wifi"
}'
```

# bảng tin xóa 
mosquitto_pub -h localhost -t /component/bridge_ip/delete -m '{
  "cmd": "delete",
  "objects": [
    {
      "type": "devices",
      "data": [
        "1_deviceIP-A4:C1:38:84:90:07:F3:A1-1",
        "1_deviceIP-A4:C1:38:84:90:07:F3:A1-3",
        "1_deviceIP-A4:C1:38:84:90:07:F3:A1-5",
        "1_deviceIP-A4:C1:38:84:90:07:F3:A1-7"
      ]
    }
  ],
  "reqid": "android-Unknown-tainv@lumi.biz-1693882123757-17",
  "source": "android-Unknown-1693882105170-0"
}'




# Bảng tin ctrl
mosquitto_pub -h localhost -t /component/bridge_ip/set -m '{
  "cmd": "set",
  "control_source": {
    "id": "android-SM A736B-tainv@lumi.biz-1695175209450-14",
    "previous_control_reqid": "",
    "type": "app"
  },
  "objects": [
    {
      "data": [
        "deviceIP-A4:C1:38:84:90:07:F3:A1-1"
      ],
      "execution": [
        {
          "command": "OnOff",
          "params": {
            "on": true
          }
        }
      ],
      "type": "devices"
    }
  ],
  "reqid": "android-SM A736B-tainv@lumi.biz-1695175209450-14",
  "source": "core"
}'



mosquitto_pub -h localhost -t component/zigbee/status -u component -P 123 -m '{
  "cmd": "status",
  "objects": [
    {
      "bridge_key": "zigbee",
      "data": [
        {
          "hash": "zigbee-A4:C1:38:84:90:07:F3:A1-3",
          "states": {
            "OnOff": {
              "on": true
            }
          },
          "type": "SWITCH"
        }
      ],
      "type": "devices"
    }
  ],
  "reqid": "fzN5jdeJ45n8DC0",
  "source": "zigbee"
}'


ipBridge 

mosquitto_pub -h localhost -t component/zigbee/A4:C1:38:84:90:07:F3:A1/config -u component -P 123 -m '{
  "cmd": "post",
  "objects": [
    {
      "bridge_key": "zigbee",
      "data": [
        {
          "attr": {
            "deviceInfo": {
              "Manufacturer": "Lumi R&D",
              "ModelId": "LM-SZ3"
            }
          },
          "bridge_key": "zigbee",
          "config": {},
          "deviceInfo": {

            "ApplicationVer": "",
            "DataCode": "",
            "HardwareVer": "",
            "Manufacturer": "Lumi R&D",
            "ModelId": "LM-SZ3",
            "ZigbeeProtocolVer": "3.0",
            "ZigbeeStackVer": ""
          },
          "hash": "zigbee-A4:C1:38:84:90:07:F3:A1-1",
          "mac": "A4:C1:38:84:90:07:F3:A1",
          "macdev": "A4:C1:38:84:90:07:F3:A1",
          "traits": [
            {
              "is_main": true,
              "name": "OnOff"
            }
          ],
          "type": "SWITCH"
        },
        {
          "attr": {
            "deviceInfo": {
              "Manufacturer": "Lumi R&D",
              "ModelId": "LM-SZ3"
            }
          },
          "bridge_key": "zigbee",
          "config": {},
          "deviceInfo": {},
          "hash": "zigbee-A4:C1:38:84:90:07:F3:A1-3",
          "mac": "A4:C1:38:84:90:07:F3:A1",
          "macdev": "A4:C1:38:84:90:07:F3:A1",
          "traits": [
            {
              "is_main": true,
              "name": "OnOff"
            }
          ],
          "type": "SWITCH"
        },
        {
          "attr": {
            "deviceInfo": {
              "Manufacturer": "Lumi R&D",
              "ModelId": "LM-SZ3"
            }
          },
          "bridge_key": "zigbee",
          "config": {},
          "deviceInfo": {},
          "hash": "zigbee-A4:C1:38:84:90:07:F3:A1-5",
          "mac": "A4:C1:38:84:90:07:F3:A1",
          "macdev": "A4:C1:38:84:90:07:F3:A1",
          "traits": [
            {
              "is_main": true,
              "name": "OnOff"
            }
          ],
          "type": "SWITCH"
        }
      ],
      "type": "devices_local"
    }
  ],
  "reqid": "ylxkif1csTDp0hf",
  "source": "core"
}'


mosquitto_pub -h localhost -t component/zigbee/A4:C1:38:84:90:07:F3:A1/status -u component -P 123 -m '{
  "cmd": "status",
  "objects": [
    {
      "bridge_key": "zigbee",
      "type": "devices",
      "data": [
        {
          "hash": "zigbee-A4:C1:38:84:90:07:F3:A1-5",
          "type": "SWITCH",
          "states": {
            "OnOff": {
              "on": true
            }
          }
        }
      ]
    }
  ],
  "reqid": "abcxyz",
  "source": "deviceIp"
}'


mosquitto_pub -h localhost -t component/zigbee/A4:C1:38:84:90:07:F3:A1/status -u component -P 123  -m '{
  "cmd": "status",
  "objects": [
    {
      "bridge_key": "zigbee",
      "data": [
        {
          "hash": "zigbee-A4:C1:38:84:90:07:F3:A1-1",
          "states": {
            "OnOff": {
              "on": false
            }
          }
        },
        {
          "hash": "zigbee-A4:C1:38:84:90:07:F3:A1-3",
          "states": {
            "OnOff": {
              "on": false
            }
          }
        },
        {
          "hash": "zigbee-A4:C1:38:84:90:07:F3:A1-5",
          "states": {
            "OnOff": {
              "on": false
            }
          }
        }
      ],
      "type": "devices"
    }
  ],
  "reqid": "lYJKTIBB5qhnYw9",
  "source": "core"
}'


mosquitto_pub -h localhost -t component/zigbee/A4:C1:38:84:90:07:F3:A1/status -u component -P 123  -m '{
  "cmd": "status",
  "control_source": {
    "id": "android-Unknown-os@lumi.vn-1712820852334-89",
    "previous_control_reqid": "",
    "type": "app"
  },
  "objects": [
    {
      "bridge_key": "zigbee",
      "data": [
        {
          "hash": "zigbee-A4:C1:38:84:90:07:F3:A1-5",
          "states": {
            "OnOff": {
              "on": false
            }
          },
          "type": "SWITCH"
        }
      ],
      "type": "devices"
    }
  ],
  "reqid": "android-Unknown-os@lumi.vn-1712820852334-89",
  "source": "deviceIp"
}'



mosquitto_pub -h localhost -t component/zigbee/control -m '{
  "cmd": "set",
  "control_source": {
    "id": "android-Unknown-os@lumi.vn-1712820852334-89",
    "previous_control_reqid": "",
    "type": "app"
  },
  "objects": [
    {
      "data": ["zigbee-A4:C1:38:84:90:07:F3:A1-3"],
      "execution": {
        "command": "OnOff",
        "params": {
          "on": false
        }
      },
      "type": "devices"
    }
  ],
  "reqid": "android-Unknown-os@lumi.vn-1712820852334-89",
  "source": "core"
}'



mosquitto_pub -h localhost -t component/zigbee/config -u component -P 123 -m '{"cmd":"delete","objects":[{"data":["zigbee-A4:C1:38:84:90:07:F3:A1-1","zigbee-A4:C1:38:84:90:07:F3:A1-3","zigbee-A4:C1:38:84:90:07:F3:A1-5"],"type":"devices"}],"reqid":"android-Unknown-os@lumi.vn-1713149909341-38","source":"core"}'

mosquitto_pub -h localhost -t component/zigbee/config -u component -P 123 -m '{"cmd":"set","objects":[{"data":[{"brigde_key":"zigbee","machc":"88:e6:28:f8:2e:64"}],"type":"reset_brigde"}],"reqid":"android-Unknown-NoUser-1713155679405-23","source":"core"}'


mosquitto_pub -h localhost -t component/ip/A4:C1:38:84:90:07:F3:A1/config -u component -P 123 -m '{
  "cmd": "post",
  "objects": [
    {
      "bridge_key": "ip",
      "data": [
        {
          "attr": {
            "deviceInfo": {
              "Manufacturer": "Lumi R&D",
              "ModelId": "LM-SZ3"
            }
          },
          "bridge_key": "ip",
          "config": {},
          "deviceInfo": {

            "ApplicationVer": "",
            "DataCode": "",
            "HardwareVer": "",
            "Manufacturer": "Lumi R&D",
            "ModelId": "LM-SZ3",
            "ipProtocolVer": "3.0",
            "ipStackVer": ""
          },
          "hash": "ip-A4:C1:38:84:90:07:F3:A1-1",
          "mac": "A4:C1:38:84:90:07:F3:A1",
          "macdev": "A4:C1:38:84:90:07:F3:A1",
          "traits": [
            {
              "is_main": true,
              "name": "OnOff"
            }
          ],
          "type": "SWITCH"
        },
        {
          "attr": {
            "deviceInfo": {
              "Manufacturer": "Lumi R&D",
              "ModelId": "LM-SZ3"
            }
          },
          "bridge_key": "ip",
          "config": {},
          "deviceInfo": {},
          "hash": "ip-A4:C1:38:84:90:07:F3:A1-3",
          "mac": "A4:C1:38:84:90:07:F3:A1",
          "macdev": "A4:C1:38:84:90:07:F3:A1",
          "traits": [
            {
              "is_main": true,
              "name": "OnOff"
            }
          ],
          "type": "SWITCH"
        },
        {
          "attr": {
            "deviceInfo": {
              "Manufacturer": "Lumi R&D",
              "ModelId": "LM-SZ3"
            }
          },
          "bridge_key": "ip",
          "config": {},
          "deviceInfo": {},
          "hash": "ip-A4:C1:38:84:90:07:F3:A1-5",
          "mac": "A4:C1:38:84:90:07:F3:A1",
          "macdev": "A4:C1:38:84:90:07:F3:A1",
          "traits": [
            {
              "is_main": true,
              "name": "OnOff"
            }
          ],
          "type": "SWITCH"
        }
      ],
      "type": "devices_local"
    }
  ],
  "reqid": "ylxkif1csTDp0hf",
  "source": "core"
}'


mosquitto_pub -h localhost -t component/ip/A4:C1:38:84:90:07:F3:A1/status -u component -P 123  -m '{
  "cmd": "status",
  "objects": [
    {
      "bridge_key": "ip",
      "data": [
        {
          "hash": "ip-A4:C1:38:84:90:07:F3:A1-1",
          "states": {
            "OnOff": {
              "on": false
            }
          }
        },
        {
          "hash": "ip-A4:C1:38:84:90:07:F3:A1-3",
          "states": {
            "OnOff": {
              "on": false
            }
          }
        },
        {
          "hash": "ip-A4:C1:38:84:90:07:F3:A1-5",
          "states": {
            "OnOff": {
              "on": false
            }
          }
        }
      ],
      "type": "devices"
    }
  ],
  "reqid": "lYJKTIBB5qhnYw9",
  "source": "core"
}'


mosquitto_pub -h localhost -t component/ip/A4:C1:38:84:90:07:F3:A1/status -u component -P 123  -m '{
  "cmd": "status",
  "control_source": {
    "id": "android-Unknown-os@lumi.vn-1712820852334-89",
    "previous_control_reqid": "",
    "type": "app"
  },
  "objects": [
    {
      "bridge_key": "ip",
      "data": [
        {
          "hash": "ip-A4:C1:38:84:90:07:F3:A1-1",
          "states": {
            "OnOff": {
              "on": true
            }
          },
          "type": "SWITCH"
        }
      ],
      "type": "devices"
    }
  ],
  "reqid": "android-Unknown-os@lumi.vn-1712820852334-89",
  "source": "deviceIp"
}'

mosquitto_pub -h localhost -t component/deviceIP/00:0B:57:FF:FE:5A:AC:CC/config -u component -P 123  -m '{
  "cmd": "set",
  "objects": [
    {
      "data": [
        {
          "brigde_key": "deviceIP",
          "command": "get_scene",
          "hash": "deviceIP-00:0B:57:FF:FE:5A:AC:CC-1",
          "machc": "9c:65:f9:48:ad:31"
        }
      ],
      "type": "advanced"
    }
  ],
  "reqid": "ios-iPhone-phuongnp.tdh@gmail.com-1684482322813-96",
  "source": "core"
}



mosquitto_pub -h localhost -t component/deviceIP/12:34:45:32:32:34/status -u component -P 123  -m '{
  "cmd": "status",
  "objects": [
    {
      "type": "advanced",
      "data": [
        {
         "brigde_key": "deviceIP",
         "command": "status_scene",
         "params":[
          {
             "ruleConfig":
             [
               {
                  "iconkey": "2",
                  "enable":1,
                  "name": "Về nhà",
                  "ruleid": "s147EfbXhI0xqp9kMkl9l0qjCmCiIkdA"
               },
               {
                  "iconkey": "2",
                  "enable":1,
                  "name": "Về nhà",
                  "ruleid": "s147EfbXhI0xqp9kMkl9l0qjCmCiIkdA"
               }
             ]
          }
         ],
         "hash":"deviceIP-12:34:45:32:32:34-1" 
        }
      ]
    }
  ],
  "reqid": "android-Unknown-tainv@lumi.biz-1693383202352-27",
  "source": "deviceIP"
}'


mosquitto_pub -h localhost -t component/deviceIP/84:2E:14:FF:FE:F6:4B:A9/config -u component -P 123  -m '{
  "cmd": "post",
  "objects": [
    {
      "bridge_key": "deviceIP",
      "data": [
        {
          "attr": {
            "deviceInfo": {
              "ApplicationVer": "1",
              "Manufacturer": "Lumi R&D",
              "ModelId": "..."
            },
            "sceneConfig": {
              "ruleConfig": [
                {
                  "MaxRule": "5"
                }
              ]
            },
            "sceneSetting": [
              {
                "name": "ruleConfig"
              }
            ]
          },
          "brigde_key": "deviceIP",
          "config": {},
          "deviceInfo": {
            "ApplicationVer": "1",
            "DataCode": "",
            "HardwareVer": "",
            "Manufacturer": "Lumi R&D",
            "ModelId": "...",
            "DeviceIPVersion": "3.0"
          },
          "hash": "deviceIP-84:2E:14:FF:FE:F6:4B:A9-1",
          "devid": "deviceIP-84:2E:14:FF:FE:F6:4B:A9-1",
          "macdev": "84:2E:14:FF:FE:F6:4B:A9",
          "traits": [
            {
              "is_main": true,
              "name": "OnOff"
            }
          ],
          "type": "SWITCH"
        },
        {
          "attr": {
            "deviceInfo": {
              "ApplicationVer": "1",
              "Manufacturer": "Lumi R&D",
              "ModelId": "..."
            },
          "brigde_key": "deviceIP",
          "config": {},
          "deviceInfo": {},
          "hash": "deviceIP-84:2E:14:FF:FE:F6:4B:A9-3",
          "macdev": "84:2E:14:FF:FE:F6:4B:A9",
          "traits": [
            {
              "is_main": true,
              "name": "OnOff"
            }
          ],
          "type": "SWITCH"
        }
       }
      ],
      "type": "devices_local"
     
    }
  ],
  "reqid": "qHwImdKVeaPE6Dp",
  "source": "deviceIP"
}'