<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/hardware-device/api_getlicensepkglist.html -->

# # 查询license资源包列表

[调试诊断](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_tools)

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：getLicensePkgList

查询小程序已购买的 license 资源包列表信息。

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
| pkg_type | number | 是 | 资源包类型，0：测试体验包，1：A 类设备，2：B 类设备，3：C 类设备，4：D 类设备，5：E类设备 |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |
| pkg_list | objarray | 资源包列表 |
| max_active_number | number | 最大激活码序号，已废弃。 |

### # Res.pkg_list(Array) Object Payload

资源包列表

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| pkg_id | string | 资源包 ID |
| pkg_type | number | 资源包类型 |
| start_time | number | 资源包下单时间 |
| end_time | number | 资源包过期时间 |
| pkg_status | number | 资源包状态，1为已生效，2为未生效，3为已过期 |
| used | number | 已使用额度 |
| all | number | 资源包总量 |

## # 4. 注意事项

开发者需要先在小程序管理后台购买设备 license 的套餐包后，方可查询到对应的资源包。

本接口将于2024年12月31日正式回收，请开发者及时进行调整适配。

详见[调整公告](https://developers.weixin.qq.com/community/minihome/doc/000428b5bd4d10c54812f7cd466401)

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| 0 | ok | ok |

## # 7. 适用范围

本接口暂未明确可调用账号类型，或在业务中根据调用传参自行确定是否可调用，请以实际调用情况为准。

