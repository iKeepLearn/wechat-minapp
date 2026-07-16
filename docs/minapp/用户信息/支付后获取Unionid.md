<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/user-info/basic-info/api_getpaidunionid.html -->

# # 支付后获取Unionid

[调试诊断](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_tools)

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：getPaidUnionid

该接口用于在用户支付完成后，获调用本接口前需要用户完成支付，用户支付完成后，取该用户的 UnionId，无需用户授权。本接口支付后的五分钟内有效。

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 调用方法：auth.getPaidUnionId
- 出入参和 HTTPS 调用相同，调用方式可查看 云调用 说明文档。

### # 第三方调用

- 本接口支持第三方平台代商家调用。
- 该接口所属的权限集 id 为：18
- 服务商获得其中之一权限集授权后，可通过使用 authorizer_access_token 代商家进行调用，具体可查看 第三方调用 说明文档。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| access_token | string | 是 | 接口调用凭证，可使用 access_token、authorizer_access_token |

### # 请求体 Request Payload

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| openid | string | 是 | 支付用户唯一标识 |
| transaction_id | string | 否 | 微信支付订单号 |
| mch_id | string | 否 | 微信支付分配的商户号，和商户订单号配合使用 |
| out_trade_no | string | 否 | 微信支付商户订单号，和商户号配合使用 |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| unionid | string | 用户唯一标识，调用成功后返回 |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |

## # 4. 注意事项

- 调用前需要用户完成支付，且在支付后的五分钟内有效。
- 使用微信支付订单号（transaction_id）和微信支付商户订单号和微信支付商户号（out_trade_no 及 mch_id），二选一

## # 5. 代码示例

### # 5.1 使用微信支付订单号

请求示例

返回示例

### # 5.2 使用微信支付商户订单号和微信支付商户号（out_trade_no 及 mch_id）

请求示例

返回示例

### # 5.3 云函数示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| -1 | system error | 系统繁忙，此时请开发者稍候再试 |
| 0 | ok | ok |
| 40001 | invalid credential  access_token isinvalid or not latest | 获取 access_token 时 AppSecret 错误，或者 access_token 无效。请开发者认真比对 AppSecret 的正确性，或查看是否正在为恰当的公众号调用接口 |
| 40003 | invalid openid | 不合法的 OpenID ，请开发者确认 OpenID （该用户）是否已关注公众号，或是否是其他公众号的 OpenID |
| 89002 | open not exists | open not exists，该公众号/小程序未绑定微信开放平台帐号。 |
| 89300 | invalid trade | 订单无效 |

## # 7. 适用范围

本接口支持「小程序（仅认证）」账号类型调用。其他账号类型如无特殊说明，均不可调用。

