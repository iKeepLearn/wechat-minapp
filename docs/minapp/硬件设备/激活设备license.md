<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/hardware-device/api_activelicensedevice.html -->

# # 激活设备license

[调试诊断](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_tools)

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：activeLicenseDevice

该接口用于批量绑定设备，并消耗相应的资源包中的激活码序号。

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 本接口不支持云调用。

### # 第三方调用

- 本接口支持第三方平台代商家调用。
- 该接口所属的权限集 id 为：118
- 服务商获得其中之一权限集授权后，可通过使用 authorizer_access_token 代商家进行调用，具体可查看 第三方调用 说明文档。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| access_token | string | 是 | 接口调用凭证，可使用 access_token、authorizer_access_token |

### # 请求体 Request Payload

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| device_list | objarray | 是 | 待激活的设备列表 |
| pkg_type | number | 是 | 资源包类型，0：测试体验包（默认），1：A 类设备，2：B 类设备，3：C 类设备，4：D 类设备，5：E类设备 |

### # Body.device_list(Array) Object Payload

待激活的设备列表

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| model_id | string | 是 | 设备型号 id。通过微信公众平台注册设备获得。 |
| sn | string | 是 | 设备唯一序列号。由厂商分配。 |
| active_number | number | 是 | 激活码序号，任意 uint32 整数（需与之前使用过的不重复）。主要用于防止重复请求导致重复激活。 |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |
| device_list | objarray | 设备列表 |

### # Res.device_list(Array) Object Payload

设备列表

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| model_id | string | 设备型号唯一标识 |
| sn | string | 设备的唯一标识 |
| expire_time | number | 设备的过期时间 |
| errcode | number | 错误码 |

## # 4. 注意事项

- 正式license的使用：每次调用最多激活100个设备，且所有设备类型必须属于同一个资源包类型。每个激活码序号只能用于激活一台设备。每个设备最多绑定 3 个激活码序号，即剩余有效时间不能超过 3 年。
- 体验license的使用：详见[平台公告](https://developers.weixin.qq.com/community/minihome/doc/000204860703984b45c02830963c01)

## # 其他说明

其中errcode固定返回0，expire_time固定返回2030-01-01 00:00:00时间戳

详见[调整公告](https://developers.weixin.qq.com/community/minihome/doc/000428b5bd4d10c54812f7cd466401)

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| 0 | ok | ok |
| 9800020 | 设备数超出限制 | 检查设备数量 |
| 9800020 | 设备数超出限制 | 检查设备数量 |
| 9800037 | 激活码序号已使用 | 更换激活码序号 |
| 9800038 | 设备有效期超出限制 | 检查设备有效期 |
| 9800039 | 资源包余额不足 | 检查资源包余额 |
| 9800040 | 资源包类型和设备类型不匹配 | 检查设备类型 |

## # 7. 适用范围

本接口暂未明确可调用账号类型，或在业务中根据调用传参自行确定是否可调用，请以实际调用情况为准。

