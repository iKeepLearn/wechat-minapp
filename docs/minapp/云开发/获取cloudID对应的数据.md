<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/cloudbase/others/api_getopendata.html -->

# # 获取cloudID对应的数据

[调试诊断](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_tools)

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：getOpenData

该接口用于换取 cloudID 对应的开放数据

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 调用方法：cloudbase.getOpenData
- 出入参和 HTTPS 调用相同，调用方式可查看 云调用 说明文档。

### # 第三方调用

- 本接口支持第三方平台代商家调用。
- 该接口所属的权限集 id 为：18、49
- 服务商获得其中之一权限集授权后，可通过使用 authorizer_access_token 代商家进行调用，具体可查看 第三方调用 说明文档。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| access_token | string | 是 | 接口调用凭证，可使用 access_token、authorizer_access_token |
| openid | string | 否 | 用户openid（敏感信息需要传入） |

### # 请求体 Request Payload

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| cloudid_list | array | 是 | CloudID 列表 |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |
| data_list | objarray | 开放数据列表 |

### # Res.data_list(Array) Object Payload

开放数据列表

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| cloud_id | string | cloud id |
| json | string | 数据详情 |

## # 4. 注意事项

本接口无特殊注意事项

## # 5. 代码示例

### # 5.1 HTTPS调用

请求示例

返回示例

### # 5.2 云函数调用

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| -1 | system error | 系统繁忙，此时请开发者稍候再试 |

## # 7. 适用范围

