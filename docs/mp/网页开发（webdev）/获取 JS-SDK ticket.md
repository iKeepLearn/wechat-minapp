<!-- 来源: https://developers.weixin.qq.com/doc/service/api/webdev/jssdk/api_getticket.html -->

# # 获取sdk临时票据

[调试诊断](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_tools)

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：getTicket

Api_ticket 是用于调用 js-sdk 的临时票据，有效期为7200 秒，通过access_token 来获取。

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 本接口不支持云调用。

### # 第三方调用

- 本接口不支持第三方平台调用。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 示例 | 说明 |
| --- | --- | --- | --- | --- |
| access_token | string | 是 | - | 接口调用凭证，可使用 access_token |
| type | string | 是 | jsapi | ticket 类型，jsapi 为 js-sdk 凭证；wx_card 为微信卡券凭证 |

### # 请求体 Request Payload

无

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| ticket | string | 临时票据 |
| expires_in | number | 有效期（秒） |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |

## # 4. 注意事项

由于获取 api_ticket 的 api 调用次数非常有限，频繁刷新api_ticket 会导致api调用受限，影响自身业务，开发者需在自己的服务存储与更新api_ticket。

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| -1 | system error | 系统繁忙，此时请开发者稍候再试 |
| 40001 | invalid credential  access_token isinvalid or not latest | 获取 access_token 时 AppSecret 错误，或者 access_token 无效。请开发者认真比对 AppSecret 的正确性，或查看是否正在为恰当的公众号调用接口 |

## # 7. 适用范围
