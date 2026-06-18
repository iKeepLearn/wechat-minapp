<!-- 来源: https://developers.weixin.qq.com/doc/service/api/apimanage/api_clearapiquota.html -->

# # 重置指定API调用次数

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：clearApiQuota

本接口使用 access_token 来重置指定接口的每日调用次数

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 本接口不支持云调用。

### # 第三方调用

- 本接口支持第三方平台使用 component_access_token 自己调用，同时还支持代商家调用。
- 服务商获得任意权限集授权后，即可通过使用 authorizer_access_token 代商家进行调用，具体可查看 第三方调用 说明文档。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 示例 | 说明 |
| --- | --- | --- | --- | --- ||
| access_token | string | 是 | ACCESS_TOKEN | 接口调用凭证，可使用 access_token、component_access_token、authorizer_access_token |

### # 请求体 Request Payload

| 参数名 | 类型 | 必填 | 示例 | 说明 |
| --- | --- | --- | --- | --- ||
| cgi_path | string | 是 | /channels/ec/basics/info/get | api的请求地址，cgi_path 必须以"/channels/ec/"开头，不要前缀"https://api.weixin.qq.com"，也不要漏了"/" |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 示例 | 说明 |
| --- | --- | --- | --- ||
| errcode | number | 0 | 错误码 |
| errmsg | string | ok | 错误信息 |

## # 4. 注意事项

- 可用于重置小程序、公众号、微信小店等微信开发生态业务接口，cgi_path 必须以"/"开头，例如"/channels/ec/"
- 如果是第三方服务商代清除quota，则需要用[authorizer_access_token](https://developers.weixin.qq.com/doc/oplatform/developers/dev/AuthorizerAccessToken.html)；
- 每个账号每月共50次清零操作机会，清零生效一次即用掉一次机会；
- 由于指标计算方法或统计时间差异，实时调用量数据可能会出现误差，一般在1%以内。

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- ||
| 40001 | invalid credential  access_token isinvalid or not latest | access_token 无效或不为最新获取的 access_token，请开发者确认access_token的有效性 |
| 41001 | access_token missing | 缺少 access_token 参数 |
| 42001 | access_token expired | access_token 超时，请检查 access_token 的有效期，请参考基础支持 - 获取 access_token 中，对 access_token 的详细机制说明 |
| 44002 | empty post data | POST 的数据包为空。post请求body参数不能为空。 |
| 45009 | reach max api daily quota limit | 超出接口每日调用限制 |
| 50002 | user limited | 用户受限，可能是用户帐号被冻结或注销 |
| 76021 | cgi_path not found, please check | cgi_path填错了 |
| 76022 | could not use this cgi_path，no permission | 当前调用接口使用的token与api所属账号不符，详情可看注意事项的说明 |

## # 7. 适用范围

