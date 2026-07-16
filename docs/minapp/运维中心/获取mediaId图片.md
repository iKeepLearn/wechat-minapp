<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/operation/api_getfeedbackmedia.html -->

# # 获取mediaId图片

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：getFeedbackmedia

该接口用于获取 mediaId 图片

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 调用方法：operation.getFeedbackmedia
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
| record_id | number | 是 | 用户反馈信息的 record_id, 可通过 getFeedback 获取 |
| media_id | string | 是 | 图片的 mediaId |

### # 请求体 Request Payload

无

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| errcode | number | 错误码。如果成功，则直接返回图片实体，不返回errcode |
| errmsg | string | 错误信息。如果成功，则直接返回图片实体，不返回errsmg |

## # 4. 注意事项

本接口无特殊注意事项

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| -1 | system error | 系统繁忙，此时请开发者稍候再试 |
| 46001 | 不存在 mediaid 对应的数据 | 开发者自查 mediaid 是否正确 |

## # 7. 适用范围

