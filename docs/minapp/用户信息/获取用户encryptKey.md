<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/user-info/internet/api_getuserencryptkey.html -->

# # 获取用户encryptKey

[调试诊断](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_tools)

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：getUserEncryptKey

该接口用于获取用户encryptKey。 会获取用户最近3次的key，每个key的存活时间为3600s

## # 1. 调用方式

### # HTTPS 调用

> 支持加密请求： 本接口支持服务通信二次加密和签名，可有效防止数据篡改与泄露。查看详情

### # 云调用

- 本接口不支持云调用。

### # 第三方调用

- 本接口支持第三方平台代商家调用。
- 该接口所属的权限集 id 为：18
- 服务商获得其中之一权限集授权后，可通过使用 authorizer_access_token 代商家进行调用，具体可查看 第三方调用 说明文档。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| access_token | string | 是 | 接口调用凭证，可使用 access_token、authorizer_access_token |
| openid | string | 是 | 用户的openid |
| signature | string | 是 | 用sessionkey作为密钥对空字符串签名得到的结果。session_key可通过code2Session接口获得。
伪代码：signature = hmac_sha256(session_key, "") |
| sig_method | string | 是 | 签名方法，只支持 hmac_sha256 |

### # 请求体 Request Payload

无

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |
| key_info_list | objarray | 用户最近三次的加密key列表 |

### # Res.key_info_list(Array) Object Payload

用户最近三次的加密key列表

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| encrypt_key | string | 加密key |
| version | number | key的版本号 |
| expire_in | number | 剩余有效时间 |
| iv | string | 加密iv |
| create_time | number | 创建key的时间戳 |

## # 4. 注意事项

本接口无特殊注意事项

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| 40001 | invalid credential  access_token isinvalid or not latest | 获取 access_token 时 AppSecret 错误，或者 access_token 无效。请开发者认真比对 AppSecret 的正确性，或查看是否正在为恰当的公众号调用接口 |
| 87007 | session_key is not existd or expired | 加密key不存在或已过期 |
| 87008 | invalid sig_method | 无效的签名方法 |
| 87009 | invalid signature | 无效的签名 |

## # 7. 适用范围

