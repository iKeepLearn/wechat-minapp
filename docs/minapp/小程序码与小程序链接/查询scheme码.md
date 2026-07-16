<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/qrcode-link/url-scheme/api_queryscheme.html -->

# # 查询scheme码

[调试诊断](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_tools)

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：queryScheme

该接口用于查询小程序 scheme 码，包括加密 scheme 和明文 scheme

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 调用方法：urlscheme.query
- 出入参和 HTTPS 调用相同，调用方式可查看 云调用 说明文档。

### # 第三方调用

- 本接口支持第三方平台代商家调用。
- 该接口所属的权限集 id 为：88
- 服务商获得其中之一权限集授权后，可通过使用 authorizer_access_token 代商家进行调用，具体可查看 第三方调用 说明文档。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| access_token | string | 是 | 接口调用凭证，可使用 access_token、authorizer_access_token |

### # 请求体 Request Payload

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| scheme | string | 否 | 小程序 scheme 码。支持加密 scheme 和明文 scheme |
| query_type | number | 否 | 查询类型。默认值0，查询  scheme 码信息：0， 查询每天剩余访问次数：1 |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |
| scheme_info | object | scheme 信息 |
| quota_info | object | quota 配置 |

### # Res.scheme_info Object Payload

scheme 信息

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| appid | string | 小程序 appid |
| path | string | 小程序页面路径 |
| query | string | 小程序页面query |
| create_time | number | 创建时间，为 Unix 时间戳 |
| expire_time | number | 到期失效时间，为 Unix 时间戳，0 表示永久生效 |
| env_version | string | 要打开的小程序版本。正式版为"release"，体验版为"trial"，开发版为"develop" |

### # Res.quota_info Object Payload

quota 配置

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| remain_visit_quota | number | URL Scheme（加密+明文）/加密 URL Link 单天剩余访问次数 |

## # 4. 注意事项

本接口无特殊注意事项

## # 5. 代码示例

### # 5.1 查询加密 scheme

请求示例

返回示例

### # 5.2 查询明文 scheme

请求示例

返回示例

### # 5.3 查询剩余访问次数

请求示例

返回示例

{
"errcode": 0,
"errmsg": "ok",
"quota_info": {
"remain_visit_quota": 1000000
}
}
}

### # 5.4 云函数调用示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| -1 | system error | 系统繁忙，此时请开发者稍候再试 |
| 0 | ok | ok |
| 40002 | invalid grant_type | 暂无生成权限（非个人主体小程序无权限，未申请 NFC 能力的小程序无权限） |
| 40097 | invalid args | 参数错误 |
| 40165 | invalid weapp pagepath | 参数path填写错误，更正后重试 |
| 40212 | invalid query | 参数query填写错误	，query格式遵循URL标准，即k1=v1&k2=v2 |
| 85402 | invalid env_version | 参数env_version填写错误，更正后重试 |
| 85403 | not found | scheme/url link不存在 |
| 85405 | appid or path not support plain scheme | 小程序 appid 或者 path 未开启明文 scheme |

## # 7. 适用范围

