<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/openApi-mgnt/api_getapidomainip.html -->

# # 获取微信API服务器IP

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：getApiDomainIp

该接口用于获取微信 api 服务器 ip 地址（开发者服务器主动访问 api.weixin.qq.com 的远端地址）

如果开发者基于安全等考虑，需要获知微信服务器的IP地址列表，以便进行相关限制，可以通过该接口获得微信服务器IP地址列表或者IP网段信息。

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 本接口不支持云调用。

### # 第三方调用

- 本接口支持第三方平台使用 component_access_token 自己调用，同时还支持代商家调用。
- 服务商获得任意权限集授权后，即可通过使用 authorizer_access_token 代商家进行调用，具体可查看 第三方调用 说明文档。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| access_token | string | 是 | 接口调用凭证，可使用 access_token、component_access_token、authorizer_access_token |

### # 请求体 Request Payload

无

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| ip_list | array | 微信服务器IP地址列表 |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |

## # 4. 注意事项

## # 5. 代码示例

### # 5.1 成功响应

请求示例

返回示例

### # 5.2 错误响应

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| 40013 | invalid appid | 不合法的 AppID ，请开发者检查 AppID 的正确性，避免异常字符，注意大小写 |

## # 7. 适用范围

