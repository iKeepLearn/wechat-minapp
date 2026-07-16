<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/cityservice/elderMedical/api_cityservice_getmedrealname.html -->

# # 查询用户实名API

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：cityservice_getmedrealname

查询老年患者实名信息

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 本接口不支持云调用。

### # 第三方调用

- 本接口支持第三方平台代商家调用。
- 该接口所属的权限集 id 为：134
- 服务商获得其中之一权限集授权后，可通过使用 authorizer_access_token 代商家进行调用，具体可查看 第三方调用 说明文档。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 示例 | 说明 |
| --- | --- | --- | --- | --- |
| access_token | string | 是 | ACCESS_TOKEN | 接口调用凭证，可使用 access_token、authorizer_access_token |

### # 请求体 Request Payload

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| app_id | string | 是 | 业务方appid |
| open_id | string | 是 | 微信用户openid |
| wxmed_authcode | string | 是 | 实名信息code，对应url中的wxmed_authcode，有效期10分钟 |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |
| cipher_real_name | string | 加密后的实名信息，base64后的加密信息 |
| cipher_algorithm | string | 加密算法，默认：AES_256_ECB_PKCS7Padding |
| key_version | number | 实名信息加密key版本号，初始版本号为0，用于识别后续密钥更换升级 |
| app_id | string | 业务方appid |
| open_id | string | 微信用户openid |

## # 4. 注意事项

用户实名数据属于敏感信息，不能以明文形式传输，所以平台返回的实名信息是经过对称加密后的base64字符串，平台会给进驻的每家医院分配长度为32位(256bit)密钥，解密后可获得明文，默认使用的加解密算法为：AES_256_ECB_PKCS7Padding；解密示例代码[下载](https://share.weiyun.com/HPYWwdQU)

**解密后的实名明文**

**实名信息字段说明**

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| 0 | ok | ok |
| 11200 | wxmed_authcode expired |  |

## # 7. 适用范围

本接口暂未明确可调用账号类型，或在业务中根据调用传参自行确定是否可调用，请以实际调用情况为准。

