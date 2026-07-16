<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/user-info/basic-info/api_getpluginopenpid.html -->

# # 获取插件用户openpid

[调试诊断](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_tools)

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：getPluginOpenPId

通过 [wx.pluginLogin](https://developers.weixin.qq.com/miniprogram/dev/api/open-api/login/wx.pluginLogin.html)  接口获得插件用户标志凭证 code 后传到开发者服务器，开发者服务器调用此接口换取插件用户的唯一标识 openpid。

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

### # 请求体 Request Payload

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| code | string | 是 | 通过 wx.pluginLogin 获得的插件用户标志凭证 code，有效时间为5分钟，一个 code 只能获取一次 openpid。 |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| errcode | number | 错误码 |
| errmsg | string | 错误信息 |
| openpid | string | 插件用户的唯一标识。 |

## # 4. 注意事项

本接口无特殊注意事项

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| 0 | ok | ok |
| 1000 | 系统错误 |  |
| 1001 | 请求参数非法 |  |
| 1003 | 请求频率过快 |  |
| 1005 | 插件 appid 与数据不匹配 |  |
| 1007 | openpid数据不存在 |  |
| 1022 | json数据解析错误 |  |
| 40016 | invalid button size | 不合法的按钮个数 |
| 45009 | reach max api daily quota limit | 天级别频率限制，2种解决途径2选1: 1.到小程序mp-开发管理-接口设置-调用额度重置;2.调用限频重置API |

## # 7. 适用范围

