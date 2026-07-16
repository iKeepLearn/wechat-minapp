<!-- 来源: https://developers.weixin.qq.com/miniprogram/dev/server/API/minidrama/usagedata/api_getcdnusagedata.html -->

# # 查询CDN用量数据

> 接口应在服务器端调用，不可在前端（小程序、网页、APP等）直接调用，具体可参考接口调用指南。

接口英文名：getCdnUsageData

该接口用于查询点播 CDN 的流量数据。

## # 1. 调用方式

### # HTTPS 调用

### # 云调用

- 本接口不支持云调用。

### # 第三方调用

- 本接口支持第三方平台代商家调用。
- 该接口所属的权限集 id 为：153
- 服务商获得其中之一权限集授权后，可通过使用 authorizer_access_token 代商家进行调用，具体可查看 第三方调用 说明文档。

## # 2. 请求参数

### # 查询参数 Query String Parameters

| 参数名 | 类型 | 必填 | 示例 | 说明 |
| --- | --- | --- | --- | --- |
| access_token | string | 是 | ACCESS_TOKEN | 接口调用凭证，可使用 access_token、authorizer_access_token |

### # 请求体 Request Payload

| 参数名 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| start_time | number | 是 | 起始时间戳。 |
| end_time | number | 是 | 截止时间戳。 |
| data_interval | string | 是 | 用量数据的时间粒度，单位：分钟，取值有：5：5 分钟粒度，返回指定查询时间内5分钟粒度的明细数据。60：小时粒度，返回指定查询时间内1小时粒度的数据。1440：天粒度，返回指定查询时间内1天粒度的数据。默认值为1440，返回天粒度的数据。 |
| query_type | number | 否 | 查询类型，0：通用播放流量和短剧播放器定向流量；1：短剧播放器定向流量；2：通用播放流量。默认值为0。 |

## # 3. 返回参数

### # 返回体 Response Payload

| 参数名 | 类型 | 示例 | 说明 |
| --- | --- | --- | --- |
| errcode | number | - | 错误码 |
| errmsg | string | ok | 错误信息 |
| data_interval | number | - | 时间粒度，单位：分钟。 |
| item_list | objarray | - | CDN 统计数据。 |

### # Res.item_list(Array) Object Payload

CDN 统计数据。

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| time | number | 数据所在时间区间的开始时间戳。 |
| value | number | 数据大小，单位：字节。 |

## # 4. 注意事项

## # 5. 代码示例

请求示例

返回示例

## # 6. 错误码

以下是本接口的错误码列表，其他错误码可参考 [通用错误码](https://developers.weixin.qq.com/doc/oplatform/developers/errCode/) ；调用接口遇到报错，可使用官方提供的 [API 诊断工具](https://developers.weixin.qq.com/console/devtools/debug?utm_source=api_errcode)  辅助定位和分析问题。

| 错误码 | 错误描述 |
| --- | --- |
| -2 | 初始化未完成，请稍后再试 |
| -1 | 系统错误 |
| 43002 | HTTP请求必须使用POST方法 |
| 44002 | POST内容为空 |
| 47001 | 输入格式错误 |
| 47003 | 参数不符合要求 |
| 10090001 | 视频类型不支持 |
| 10090002 | 图片类型不支持 |
| 10090003 | 图片URL无效 |
| 10090005 | resource_type无效 |
| 10090038 | 被授权账号没有【文娱-微短剧】类目 |
| 10090039 | 已经被解除授权 |
| 10090040 | 剧集已经被占用 |
| 10090041 | 剧目名称不符合规范 |
| 10090042 | 剧集名称不符合规范 |
| 10090043 | 不存在授权关系 |
| 10093011 | 操作失败 |
| 10093014 | 参数错误（包括参数格式、类型等错误） |
| 10093023 | 操作过于频繁 |
| 10093030 | 资源不存在 |

## # 7. 适用范围

本接口暂未明确可调用账号类型，或在业务中根据调用传参自行确定是否可调用，请以实际调用情况为准。

