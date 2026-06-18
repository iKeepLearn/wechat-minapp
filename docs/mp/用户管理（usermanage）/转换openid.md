<!-- 来源: https://developers.weixin.qq.com/doc/service/api/usermanage/changeopenid/api_changeopenid.html -->

# # 转换openid

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：changeopenid

该接口用于公众号、服务号之后进行 openid 转换。

公众号/服务号账号迁移后，关注用户的 openid 会变化，微信用户关注不同的公众号或服务号，对应的openid是不一样的，迁移成功后，关注用户的openid 以目标账号（即新账号）对应的 openid 为准。原账号非个人主体的，可以通过当前接口转换openid。

当账号迁移后，可以通过该接口：

- 原账号：准备要迁移的账号，当审核完成且管理员确认后即被回收。
- 新账号：用来接纳关注用户的账号。新账号在整个流程中均能正常使用。

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
| --- | --- | --- | --- | --- |
| access_token | string | 是 | ACCESS_TOKEN | 接口调用凭证，可使用 access_token |

### # 请求体 Request Payload

| 参数名 | 类型 | 必填 | 示例 | 说明 |
| --- | --- | --- | --- | --- |
| from_appid | string | 是 | gh_91ae50dfeb1c | 原账号的原始id，不是appid |
| openid_list | array | 是 |  | 第1步中拉取的原账号用户列表，这些必须是旧账号目前关注的才行，否则会出错；一次最多100个。 |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| errcode | number | 错误码 |
| errmsg | string | 错误描述 |
| result_list | objarray | openid结果列表 |

### # Res.result_list(Array) Object Payload

openid结果列表

| 参数名 | 类型 | 示例 | 说明 |
| --- | --- | --- | --- |
| ori_openid | string | oEmYbwN-n24jxvk4Sox81qedINkQ | 旧openid |
| new_openid | string | o2FwqwI9xCsVadFah_HtpPfaR-X4 | 新openid |
| err_msg | string | ok | 错误描述。"ori_openid error"则表示这个openid目前没有关注旧账号。 |

## # 4. 注意事项

本接口无特殊注意事项

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 | 解决方案 |
| --- | --- | --- |
| 63178 | appid wrong | from_appid参数错误，和调用的账号并没有迁移关系 |
| 63182 | openid_list empty | openid是空 |
| 63183 | appid error | appid没有迁移关系 |

## # 7. 适用范围

该接口用于公众号、服务号之后进行 openid 转换，并且在账号迁移审核完成后开始调用，并最多保留15天。若账号迁移没完成，调用时无返回结果或报错。账号迁移15天后，该转换接口将会失效、无法拉取到数据。
