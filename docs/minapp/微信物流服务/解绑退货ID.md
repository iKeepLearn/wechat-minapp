<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/weixin-express/express-return/api_unbindreturnid.html -->

# # 解绑退货ID

[调试诊断](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_tools)

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：unbindReturnId

当商家同意退货申请之后，与用户达成协商「无需退货」时，可以通过本接口可以接触商家退货单与退货 ID的绑定。考虑到预约快递员上门取件的情况在用户侧发生，因此只有当用户是自主填写运单号情况下才支持解绑退货 ID 。

如有开发问题或建议，可前往[微信开放社区-微信物流服务](https://developers.weixin.qq.com/community/minihome/mixflow/1792207662500118536)  发帖提问讨论，官方工作人员会及时回复。

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 本接口不支持云调用。

### # 第三方调用

- 本接口支持第三方平台代商家调用。
- 该接口所属的权限集 id 为：45
- 服务商获得其中之一权限集授权后，可通过使用 authorizer_access_token 代商家进行调用，具体可查看 第三方调用 说明文档。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 示例 | 说明 |
| --- | --- | --- | --- | --- |
| access_token | string | 是 | ACCESS_TOKEN | 接口调用凭证，可使用 access_token、authorizer_access_token |

### # 请求体 Request Payload

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| return_id | string | 是 | 退货ID |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |

## # 4. 注意事项

本接口无特殊注意事项

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 |
| --- | --- |
| -1 | 请联系微信平台解决 |
| 40097 | return_id为空 |
| 931023 | 运单不存在 |

## # 7. 适用范围

本接口暂未明确可调用账号类型，或在业务中根据调用传参自行确定是否可调用，请以实际调用情况为准。

