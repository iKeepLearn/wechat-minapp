## 接口调用凭证

| 名称 | URL |
| --- | --- |
| 获取接口调用凭据 | /miniprogram/dev/server/API/mp-access-token/api_getaccesstoken.html |
| 获取稳定版接口调用凭据 | /miniprogram/dev/server/API/mp-access-token/api_getstableaccesstoken.html |
## openApi管理

| 名称 | URL |
| --- | --- |
| 查询API调用额度 | /miniprogram/dev/server/API/openApi-mgnt/api_getapiquota.html |
| 重置API调用次数 | /miniprogram/dev/server/API/openApi-mgnt/api_clearquota.html |
| 重置指定API调用次数 | /miniprogram/dev/server/API/openApi-mgnt/api_clearapiquota.html |
| 使用AppSecret重置API调用次数 | /miniprogram/dev/server/API/openApi-mgnt/api_clearquotabyappsecret.html |
| 查询rid信息 | /miniprogram/dev/server/API/openApi-mgnt/api_getridinfo.html |
| 网络通信检测 | /miniprogram/dev/server/API/openApi-mgnt/api_callbackcheck.html |
| 获取微信API服务器IP | /miniprogram/dev/server/API/openApi-mgnt/api_getapidomainip.html |
| 获取微信推送服务器IP | /miniprogram/dev/server/API/openApi-mgnt/api_getcallbackip.html |
## 用户信息

| 名称 | URL |
| --- | --- |
| 获取插件用户openpid | /miniprogram/dev/server/API/user-info/basic-info/api_getpluginopenpid.html |
| 检查加密信息 | /miniprogram/dev/server/API/user-info/basic-info/api_checkencrypteddata.html |
| 支付后获取Unionid | /miniprogram/dev/server/API/user-info/basic-info/api_getpaidunionid.html |
| 获取用户encryptKey | /miniprogram/dev/server/API/user-info/internet/api_getuserencryptkey.html |
| 获取手机号 | /miniprogram/dev/server/API/user-info/phone-number/api_getphonenumber.html |
## 小程序登录

| 名称 | URL |
| --- | --- |
| 小程序登录凭证校验 | /miniprogram/dev/server/API/user-login/api_code2session.html |
| 检验登录态 | /miniprogram/dev/server/API/user-login/api_checksessionkey.html |
| 重置登录态 | /miniprogram/dev/server/API/user-login/api_resetusersessionkey.html |
## 小程序码与小程序链接

| 名称 | URL |
| --- | --- |
| 获取小程序码 | /miniprogram/dev/server/API/qrcode-link/qr-code/api_getqrcode.html |
| 获取不限制的小程序码 | /miniprogram/dev/server/API/qrcode-link/qr-code/api_getunlimitedqrcode.html |
| 获取小程序二维码 | /miniprogram/dev/server/API/qrcode-link/qr-code/api_createqrcode.html |
| 查询scheme码 | /miniprogram/dev/server/API/qrcode-link/url-scheme/api_queryscheme.html |
| 获取加密scheme码 | /miniprogram/dev/server/API/qrcode-link/url-scheme/api_generatescheme.html |
| 获取NFC的小程序scheme | /miniprogram/dev/server/API/qrcode-link/url-scheme/api_generatenfcscheme.html |
| 获取加密URLLink | /miniprogram/dev/server/API/qrcode-link/url-link/api_generateurllink.html |
| 查询加密URLLink | /miniprogram/dev/server/API/qrcode-link/url-link/api_queryurllink.html |
| 获取ShortLink | /miniprogram/dev/server/API/qrcode-link/short-link/api_generateshortlink.html |
## 小程序客服

| 名称 | URL |
| --- | --- |
| 添加客服账号 | /miniprogram/dev/server/API/kf-mgnt/kf-message/api_addkfaccount.html |
| 获取所有客服账号 | /miniprogram/dev/server/API/kf-mgnt/kf-message/api_getkflist.html |
| 获取在线客服列表 | /miniprogram/dev/server/API/kf-mgnt/kf-message/api_getonlinekflist.html |
| 删除客服账号 | /miniprogram/dev/server/API/kf-mgnt/kf-message/api_delkfaccount.html |
| 发送客服消息 | /miniprogram/dev/server/API/kf-mgnt/kf-message/api_sendcustommessage.html |
| 客服输入状态 | /miniprogram/dev/server/API/kf-mgnt/kf-message/api_typing.html |
| 设置客服管理员 | /miniprogram/dev/server/API/kf-mgnt/kf-message/api_setkfadmin.html |
| 取消客服管理员 | /miniprogram/dev/server/API/kf-mgnt/kf-message/api_cancelkfadmin.html |
| 新增临时素材 | /miniprogram/dev/server/API/kf-mgnt/kf-message/api_uploadtempmedia.html |
| 获取临时素材 | /miniprogram/dev/server/API/kf-mgnt/kf-message/api_getmedia.html |
| 客服子商户创建商户 | /miniprogram/dev/server/API/kf-mgnt/kf-management/api_registerbusiness.html |
| 客服子商户更新商户信息 | /miniprogram/dev/server/API/kf-mgnt/kf-management/api_updatebusiness.html |
| 客服子商户拉取单个商户信息 | /miniprogram/dev/server/API/kf-mgnt/kf-management/api_getbusiness.html |
| 客服子商户拉取多个商户信息 | /miniprogram/dev/server/API/kf-mgnt/kf-management/api_listbusiness.html |
## 微信客服

| 名称 | URL |
| --- | --- |
| 查询绑定情况 | /miniprogram/dev/server/API/kf-work/api_getkfworkbound.html |
| 绑定微信客服 | /miniprogram/dev/server/API/kf-work/api_bindkfwork.html |
| 解除绑定微信客服 | /miniprogram/dev/server/API/kf-work/api_unbindkfwork.html |
## 消息相关

| 名称 | URL |
| --- | --- |
| 创建activity_id | /miniprogram/dev/server/API/mp-message-management/updatable-message/api_createactivityid.html |
| 修改动态消息 | /miniprogram/dev/server/API/mp-message-management/updatable-message/api_setupdatablemsg.html |
| 修改小程序聊天工具的动态卡片消息 | /miniprogram/dev/server/API/mp-message-management/updatable-message/api_setchattoolmsg.html |
| 删除模板 | /miniprogram/dev/server/API/mp-message-management/subscribe-message/api_delwxanewtemplate.html |
| 获取类目 | /miniprogram/dev/server/API/mp-message-management/subscribe-message/api_getcategory.html |
| 获取模板中的关键词 | /miniprogram/dev/server/API/mp-message-management/subscribe-message/api_getpubnewtemplatekeywords.html |
| 获取类目下的公共模板 | /miniprogram/dev/server/API/mp-message-management/subscribe-message/api_getpubnewtemplatetitles.html |
| 获取已有模板列表 | /miniprogram/dev/server/API/mp-message-management/subscribe-message/api_getwxapubnewtemplate.html |
| 发送订阅消息 | /miniprogram/dev/server/API/mp-message-management/subscribe-message/api_sendmessage.html |
| 选用模板 | /miniprogram/dev/server/API/mp-message-management/subscribe-message/api_addwxanewtemplate.html |
| 激活与更新服务卡片 | /miniprogram/dev/server/API/mp-message-management/subscribe-message/api_setusernotify.html |
| 更新服务卡片扩展信息 | /miniprogram/dev/server/API/mp-message-management/subscribe-message/api_setusernotifyext.html |
| 查询服务卡片状态 | /miniprogram/dev/server/API/mp-message-management/subscribe-message/api_getusernotify.html |
## 用工关系

| 名称 | URL |
| --- | --- |
| 推送用工消息 | /miniprogram/dev/server/API/laboruse/api_sendemployeerelationmsg.html |
| 解绑用工关系 | /miniprogram/dev/server/API/laboruse/api_unbinduserb2cauthinfo.html |
## 小程序安全

| 名称 | URL |
| --- | --- |
| 文本内容安全识别 | /miniprogram/dev/server/API/sec-center/sec-check/api_msgseccheck.html |
| 多媒体内容安全识别 | /miniprogram/dev/server/API/sec-center/sec-check/api_mediacheckasync.html |
| 获取用户安全等级 | /miniprogram/dev/server/API/sec-center/safety-control-capability/api_getuserriskrank.html |
## 数据分析

| 名称 | URL |
| --- | --- |
| 获取用户访问小程序周留存 | /miniprogram/dev/server/API/data-analysis/visit-retain/api_getweeklyretain.html |
| 获取用户访问小程序月留存 | /miniprogram/dev/server/API/data-analysis/visit-retain/api_getmonthlyretain.html |
| 获取用户访问小程序日留存 | /miniprogram/dev/server/API/data-analysis/visit-retain/api_getdailyretain.html |
| 获取用户访问小程序数据月趋势 | /miniprogram/dev/server/API/data-analysis/visit-trend/api_getmonthlyvisittrend.html |
| 获取用户访问小程序数据日趋势 | /miniprogram/dev/server/API/data-analysis/visit-trend/api_getdailyvisittrend.html |
| 获取用户访问小程序数据周趋势 | /miniprogram/dev/server/API/data-analysis/visit-trend/api_getweeklyvisittrend.html |
| 获取用户访问小程序数据概况 | /miniprogram/dev/server/API/data-analysis/others/api_getdailysummary.html |
| 获取访问页面数据 | /miniprogram/dev/server/API/data-analysis/others/api_getvisitpage.html |
| 获取小程序用户画像分布 | /miniprogram/dev/server/API/data-analysis/others/api_getuserportrait.html |
| 获取小程序性能数据 | /miniprogram/dev/server/API/data-analysis/others/api_getperformancedata.html |
| 获取用户小程序访问分布数据 | /miniprogram/dev/server/API/data-analysis/others/api_getvisitdistribution.html |
## 硬件设备

| 名称 | URL |
| --- | --- |
| 发送设备消息 | /miniprogram/dev/server/API/hardware-device/api_sendhardwaredevicemessage.html |
| 获取设备票据 | /miniprogram/dev/server/API/hardware-device/api_getsnticket.html |
| 创建设备组 | /miniprogram/dev/server/API/hardware-device/api_createiotgroupid.html |
| 查询设备组信息 | /miniprogram/dev/server/API/hardware-device/api_getiotgroupinfo.html |
| 设备组添加设备 | /miniprogram/dev/server/API/hardware-device/api_addiotgroupdevice.html |
| 设备组删除设备 | /miniprogram/dev/server/API/hardware-device/api_removeiotgroupdevice.html |
| 查询license资源包列表 | /miniprogram/dev/server/API/hardware-device/api_getlicensepkglist.html |
| 激活设备license | /miniprogram/dev/server/API/hardware-device/api_activelicensedevice.html |
| 查询设备激活详情 | /miniprogram/dev/server/API/hardware-device/api_getlicensedeviceinfo.html |
## 运维中心

| 名称 | URL |
| --- | --- |
| 查询域名配置 | /miniprogram/dev/server/API/operation/api_getdomaininfo.html |
| 获取性能数据 | /miniprogram/dev/server/API/operation/api_getperformance.html |
| 获取访问来源 | /miniprogram/dev/server/API/operation/api_getscenelist.html |
| 获取客户端版本 | /miniprogram/dev/server/API/operation/api_getversionlist.html |
| 查询实时日志 | /miniprogram/dev/server/API/operation/api_realtimelogsearch.html |
| 获取用户反馈列表 | /miniprogram/dev/server/API/operation/api_getfeedback.html |
| 获取mediaId图片 | /miniprogram/dev/server/API/operation/api_getfeedbackmedia.html |
| 查询js错误详情 | /miniprogram/dev/server/API/operation/api_getjserrdetail.html |
| 查询错误列表 | /miniprogram/dev/server/API/operation/api_getjserrlist.html |
| 获取分阶段发布详情 | /miniprogram/dev/server/API/operation/api_getgrayreleaseplan.html |
## 插件管理

| 名称 | URL |
| --- | --- |
| 插件申请管理 | /miniprogram/dev/server/API/plugin-management/api_managepluginapplication.html |
| 插件管理 | /miniprogram/dev/server/API/plugin-management/api_manageplugin.html |
## 云开发

| 名称 | URL |
| --- | --- |
| 触发云函数 | /miniprogram/dev/server/API/cloudbase/functions/api_invokecloudfunction.html |
| 延时调用云函数 | /miniprogram/dev/server/API/cloudbase/functions/api_adddelayedfunctiontask.html |
| 数据库插入记录 | /miniprogram/dev/server/API/cloudbase/database/api_adddatabaseitem.html |
| 数据库聚合 | /miniprogram/dev/server/API/cloudbase/database/api_aggregatedatabase.html |
| 新增集合 | /miniprogram/dev/server/API/cloudbase/database/api_adddatabasecollection.html |
| 删除集合 | /miniprogram/dev/server/API/cloudbase/database/api_deletedatabasecollection.html |
| 获取集合信息 | /miniprogram/dev/server/API/cloudbase/database/api_getdatabasecollection.html |
| 统计集合记录数 | /miniprogram/dev/server/API/cloudbase/database/api_getdatabasecount.html |
| 数据库删除记录 | /miniprogram/dev/server/API/cloudbase/database/api_deletedatabaseitem.html |
| 数据库导出 | /miniprogram/dev/server/API/cloudbase/database/api_exportdatabaseitem.html |
| 数据库导入 | /miniprogram/dev/server/API/cloudbase/database/api_importdatabaseitem.html |
| 数据库迁移状态查询 | /miniprogram/dev/server/API/cloudbase/database/api_getdatabasemigratestatus.html |
| 数据库查询记录 | /miniprogram/dev/server/API/cloudbase/database/api_getdatabaserecord.html |
| 数据库更新记录 | /miniprogram/dev/server/API/cloudbase/database/api_updatedatabaserecord.html |
| 更新数据库索引 | /miniprogram/dev/server/API/cloudbase/database/api_updatedatabaseindexs.html |
| 获取文件上传链接 | /miniprogram/dev/server/API/cloudbase/storage/api_getuploadtcbfilelink.html |
| 获取文件下载链接 | /miniprogram/dev/server/API/cloudbase/storage/api_getdownloadtcbfilelink.html |
| 删除文件 | /miniprogram/dev/server/API/cloudbase/storage/api_deletetcbcloudfile.html |
| 发送短信v2 | /miniprogram/dev/server/API/cloudbase/others/api_newsendcloudbasesms.html |
| 发送短信 | /miniprogram/dev/server/API/cloudbase/others/api_sendcloudbasesms.html |
| 创建发短信任务 | /miniprogram/dev/server/API/cloudbase/others/api_createsendsmstask.html |
| 云开发上报接口 | /miniprogram/dev/server/API/cloudbase/others/api_cloudbasereportapi.html |
| 查询短信记录 | /miniprogram/dev/server/API/cloudbase/others/api_describesmsrecords.html |
| 描述扩展上传文件信息 | /miniprogram/dev/server/API/cloudbase/others/api_describeextensionuploadinfo.html |
| 获取云开发数据 | /miniprogram/dev/server/API/cloudbase/others/api_getcloudbasestatistics.html |
| 获取cloudID对应的数据 | /miniprogram/dev/server/API/cloudbase/others/api_getopendata.html |
| 获取实时语音签名 | /miniprogram/dev/server/API/cloudbase/others/api_getcloudbasevoipsign.html |
| 获取腾讯云API调用凭证 | /miniprogram/dev/server/API/cloudbase/others/api_getcloudtoken.html |
## 物流助手

| 名称 | URL |
| --- | --- |
| 绑定/解绑物流账号 | /miniprogram/dev/server/API/express/express-by-business/api_bindaccount.html |
| 获取所有绑定的物流账号 | /miniprogram/dev/server/API/express/express-by-business/api_getallaccount.html |
| 获取支持的快递公司列表 | /miniprogram/dev/server/API/express/express-by-business/api_getalldelivery.html |
| 取消运单 | /miniprogram/dev/server/API/express/express-by-business/api_cancelorder.html |
| 配置面单打印员 | /miniprogram/dev/server/API/express/express-by-business/api_updateprinter.html |
| 获取电子面单余额 | /miniprogram/dev/server/API/express/express-by-business/api_getquota.html |
| 获取运单数据 | /miniprogram/dev/server/API/express/express-by-business/api_getorder.html |
| 模拟更新订单状态 | /miniprogram/dev/server/API/express/express-by-business/api_testupdateorder.html |
| 获取打印员 | /miniprogram/dev/server/API/express/express-by-business/api_getprinter.html |
| 查询运单轨迹 | /miniprogram/dev/server/API/express/express-by-business/api_getpath.html |
| 批量获取运单数据 | /miniprogram/dev/server/API/express/express-by-business/api_batchgetorder.html |
| 生成运单 | /miniprogram/dev/server/API/express/express-by-business/api_addorder.html |
| 更新商户审核结果 | /miniprogram/dev/server/API/express/express-by-provider/api_updatebusiness.html |
| 更新运单轨迹 | /miniprogram/dev/server/API/express/express-by-provider/api_updatepath.html |
| 预览面单模板 | /miniprogram/dev/server/API/express/express-by-provider/api_previewtemplate.html |
| 获取面单联系人信息 | /miniprogram/dev/server/API/express/express-by-provider/api_getcontact.html |
| 运力取消订单 | /miniprogram/dev/server/API/express/express-by-provider/api_scatterdeliverycancel.html |
| 运力更新需支付的运费 | /miniprogram/dev/server/API/express/express-by-provider/api_scatterupdateorderfee.html |
| 运力公司退款 | /miniprogram/dev/server/API/express/express-by-provider/api_scatterdeliveryrefund.html |
| 对账单下载 | /miniprogram/dev/server/API/express/express-by-provider/api_scatter_get_bill.html |
| 运力返回用户投诉处理结果 | /miniprogram/dev/server/API/express/express-by-provider/api_scatterupdatecomplainresult.html |
| 运力更新订单状态 | /miniprogram/dev/server/API/express/express-by-provider/api_scatterupdateorderstatus.html |
## 即时配送

| 名称 | URL |
| --- | --- |
| 获取已支持的配送公司列表 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_getallimmedelivery.html |
| 预下配送单 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_preaddorder.html |
| 拉取已绑定账号 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_getbindaccount.html |
| 预取消配送单 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_precancelorder.html |
| 申请开通即时配送 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_opendelivery.html |
| 发起绑定请求 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_bindlocalaccount.html |
| 重新下单 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_reorder.html |
| 模拟更新配送单状态 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_realmockupdateorder.html |
| 模拟配送公司更新配送单状态 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_mockupdateorder.html |
| 拉取配送单信息 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_getlocalorder.html |
| 异常件退回商家确认 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_abnormalconfirm.html |
| 取消配送单 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_cancellocalorder.html |
| 添加小费 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_addtips.html |
| 添加配送单 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-business/api_addlocalorder.html |
| 更新配送单状态 | /miniprogram/dev/server/API/immediate-delivery/deliver-by-provider/api_updateorder.html |
## 微信物流服务

| 名称 | URL |
| --- | --- |
| 查询运单 | /miniprogram/dev/server/API/weixin-express/express-search/api_query_trace.html |
| 获取运力id列表 | /miniprogram/dev/server/API/weixin-express/express-search/api_get_delivery_list.html |
| 传运单 | /miniprogram/dev/server/API/weixin-express/express-search/api_trace_waybill.html |
| 更新物流信息 | /miniprogram/dev/server/API/weixin-express/express-search/api_update_waybill_goods.html |
| 更新物品信息接口 | /miniprogram/dev/server/API/weixin-express/express-msg/api_update_follow_waybill_goods.html |
| 查运单 | /miniprogram/dev/server/API/weixin-express/express-msg/api_query_follow_trace.html |
| 传运单 | /miniprogram/dev/server/API/weixin-express/express-msg/api_follow_waybill.html |
| 获取运力id列表 | /miniprogram/dev/server/API/weixin-express/express-msg/api_get_delivery_list.html |
| 无忧退货 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_open.html |
| 开通无忧退货 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_open.html |
| 查询开通状态 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_query_open.html |
| 发货时投保 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_createorder.html |
| 无忧退理赔 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_claim.html |
| 申请充值订单号 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_createchargeid.html |
| 申请支付 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_applypay.html |
| 拉取充值订单信息 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_getpayorderlist.html |
| 充值保费退款 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_refund.html |
| 拉取理赔摘要 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_getsummary.html |
| 拉取保单信息 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_getorderlist.html |
| 设置保费告警余额 | /miniprogram/dev/server/API/weixin-express/freight/api_insurance_freight_update_notify_funds.html |
| 解绑退货ID | /miniprogram/dev/server/API/weixin-express/express-return/api_unbindreturnid.html |
| 查询退货ID状态 | /miniprogram/dev/server/API/weixin-express/express-return/api_getreturnid.html |
| 创建退货ID | /miniprogram/dev/server/API/weixin-express/express-return/api_addreturnid.html |
| 开通门店权限 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_apply.html |
| 创建门店 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_createstore.html |
| 查询门店 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_querystore.html |
| 更新门店 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_updatestore.html |
| 门店运费充值 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_storecharge.html |
| 门店运费退款 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_store_refund.html |
| 门店运费流水查询 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_queryflow.html |
| 门店余额查询 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_balancequery.html |
| 查询运费 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_preaddorder.html |
| 创建配送单 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_addorder.html |
| 查询配送单 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_queryorder.html |
| 取消配送单 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_cancelorder.html |
| 设置扣费主体 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_setpaymode.html |
| 查询扣费主体 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_getpaymode.html |
| 查询支持同城配送的城市 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_getcity.html |
| 模拟回调接口 | /miniprogram/dev/server/API/weixin-express/same_city_distribution/api_intracity_mocknotify.html |
| 用户手机状态查询 | /miniprogram/dev/server/API/weixin-express/msgpush/api_deliveryuserquery.html |
| 推送已绑定物流轨迹信息 | /miniprogram/dev/server/API/weixin-express/msgpush/api_deliverypathnotify.html |
## 付费管理

| 名称 | URL |
| --- | --- |
| 查询购买资源包的用量情况 | /miniprogram/dev/server/API/charge/api_getusagedetail.html |
| 获取小程序某个付费能力的最近用量数据 | /miniprogram/dev/server/API/charge/api_getrecentaverageusage.html |
## 小程序交易管理服务

| 名称 | URL |
| --- | --- |
| 发货信息录入 | /miniprogram/dev/server/API/order_shipping/api_uploadshippinginfo.html |
| 发货信息合单录入 | /miniprogram/dev/server/API/order_shipping/api_uploadcombinedshippinginfo.html |
| 查询订单发货状态 | /miniprogram/dev/server/API/order_shipping/api_getorder.html |
| 查询订单列表 | /miniprogram/dev/server/API/order_shipping/api_getorderlist.html |
| 确认收货提醒 | /miniprogram/dev/server/API/order_shipping/api_notifyconfirmreceive.html |
| 消息跳转路径设置 | /miniprogram/dev/server/API/order_shipping/api_setmsgjumppath.html |
| 查询小程序是否已开通发货信息管理服务 | /miniprogram/dev/server/API/order_shipping/api_istrademanaged.html |
| 查询小程序是否已完成交易结算管理确认 | /miniprogram/dev/server/API/order_shipping/api_istrademanagementconfirmationcompleted.html |
| 特殊发货报备 | /miniprogram/dev/server/API/order_shipping/api_opspecialorder.html |
| 品牌申请 | /miniprogram/dev/server/API/order_shipping/api_famousbrandapply.html |
| 小程序品牌申请状态查询 | /miniprogram/dev/server/API/order_shipping/api_getfamousbrandapplystatus.html |
| 小程序交易类型变更申请 | /miniprogram/dev/server/API/order_shipping/api_setwxatradetypecgi.html |
## 交易保障

| 名称 | URL |
| --- | --- |
| 获取小程序交易体验分违规记录 | /miniprogram/dev/server/API/transaction-guarantee/basic/api_getpenaltylist.html |
| 获取交易保障标状态 | /miniprogram/dev/server/API/transaction-guarantee/basic/api_getguaranteestatus.html |
| 查询评价列表 | /miniprogram/dev/server/API/transaction-guarantee/comment/api_getccommentlist.html |
| 查询评论列表 | /miniprogram/dev/server/API/transaction-guarantee/comment/api_getcommentreplylist.html |
| 查询评价详情 | /miniprogram/dev/server/API/transaction-guarantee/comment/api_getcommentinfo.html |
| 创建评论 | /miniprogram/dev/server/API/transaction-guarantee/comment/api_addreply.html |
| 删除评论 | /miniprogram/dev/server/API/transaction-guarantee/comment/api_deletereply.html |
| 创建回复 | /miniprogram/dev/server/API/transaction-guarantee/comment/api_addcommentreply.html |
| 删除回复 | /miniprogram/dev/server/API/transaction-guarantee/comment/api_deletecommentreply.html |
| 重置Api客服quota | /miniprogram/dev/server/API/transaction-guarantee/comment/api_resetapikfquota.html |
| 确认和解 | /miniprogram/dev/server/API/transaction-guarantee/comment/api_confirmcompromise.html |
| 商家回应投诉 | /miniprogram/dev/server/API/transaction-guarantee/complaint/api_respondcomplaint.html |
| 商家补充凭证 | /miniprogram/dev/server/API/transaction-guarantee/complaint/api_supplyproof.html |
| 商家提交退款凭证 | /miniprogram/dev/server/API/transaction-guarantee/complaint/api_submitrefund.html |
| 查询投诉单详情 | /miniprogram/dev/server/API/transaction-guarantee/complaint/api_getorderdetail.html |
| 商家申诉 | /miniprogram/dev/server/API/transaction-guarantee/complaint/api_busiappeal.html |
## B2b门店助手

| 名称 | URL |
| --- | --- |
| 开通流程 | /miniprogram/dev/server/API/B2b/store_assistant/api_retailbusinessapply.html |
| 预录入门店信息 | /miniprogram/dev/server/API/B2b/store_assistant/api_batchcreateretail.html |
| 门店信息查询 | /miniprogram/dev/server/API/B2b/store_assistant/api_getretailinfo.html |
| 全量授权门店查询 | /miniprogram/dev/server/API/B2b/store_assistant/api_getretailopenidlist.html |
| 模板消息列表及下发 | /miniprogram/dev/server/API/B2b/notify/api_retailnotifybusiness.html |
| 消息效果数据 | /miniprogram/dev/server/API/B2b/notify/api_getretailmessagelist.html |
| 商户号进件 | /miniprogram/dev/server/API/B2b/bill/api_retailregistermch.html |
| 上传商户图片 | /miniprogram/dev/server/API/B2b/bill/api_retailuploadmchfile.html |
| 查询商户号开通状态 | /miniprogram/dev/server/API/B2b/bill/api_retailgetmchorder.html |
| 申请开通银行转账 | /miniprogram/dev/server/API/B2b/bill/api_registeronlywqf.html |
| 跳转银行转账页面 | /miniprogram/dev/server/API/B2b/bill/api_createwqflink.html |
| 获取小程序下所有商户的信息 | /miniprogram/dev/server/API/B2b/bill/api_getmchinfo.html |
| 报名微信支付技术服务费优惠活动 | /miniprogram/dev/server/API/B2b/bill/api_setmchprofitrate.html |
| 报名银行转账技术服务费优惠活动 | /miniprogram/dev/server/API/B2b/bill/api_updatewqfchargefee.html |
| 查询银行转账的技术服务费率 | /miniprogram/dev/server/API/B2b/bill/api_getwqfchargefee.html |
| 查询订单 | /miniprogram/dev/server/API/B2b/bill/api_getorder.html |
| 关闭订单 | /miniprogram/dev/server/API/B2b/bill/api_closeb2border.html |
| 退款 | /miniprogram/dev/server/API/B2b/bill/api_refundorder.html |
| 查询退款 | /miniprogram/dev/server/API/B2b/bill/api_getrefund.html |
| 获取密钥AppKey | /miniprogram/dev/server/API/B2b/bill/api_getappkey.html |
| 接口下载交易账单与资金账单 | /miniprogram/dev/server/API/B2b/bill/api_downloadbill.html |
| 查询账户余额 | /miniprogram/dev/server/API/B2b/bill/api_getmchbalance.html |
| 发起手动提现 | /miniprogram/dev/server/API/B2b/bill/api_manualwithdraw.html |
| 查询提现状态 | /miniprogram/dev/server/API/B2b/bill/api_querywithdraw.html |
| 微信支付自动提现接口 | /miniprogram/dev/server/API/B2b/bill/api_setautowithdraw.html |
| 添加分账方 | /miniprogram/dev/server/API/B2b/bill/api_addprofitsharingaccount.html |
| 删除分账方 | /miniprogram/dev/server/API/B2b/bill/api_delprofitsharingaccount.html |
| 查询分账方 | /miniprogram/dev/server/API/B2b/bill/api_queryprofitsharingaccount.html |
| 请求分账 | /miniprogram/dev/server/API/B2b/bill/api_createprofitsharingorder.html |
| 查询分账结果 | /miniprogram/dev/server/API/B2b/bill/api_queryprofitsharingorder.html |
| 查询分账剩余金额 | /miniprogram/dev/server/API/B2b/bill/api_queryprofitsharingremainamt.html |
| 完成分账 | /miniprogram/dev/server/API/B2b/bill/api_finishprofitsharingorder.html |
| 请求分账回退 | /miniprogram/dev/server/API/B2b/bill/api_refundprofitsharing.html |
| 查询分账回退结果 | /miniprogram/dev/server/API/B2b/bill/api_queryrefundprofitsharingorder.html |
## 短剧媒资管理

| 名称 | URL |
| --- | --- |
| 单个文件上传 | /miniprogram/dev/server/API/minidrama/vod_fileupload/api_singlefileupload.html |
| 拉取上传 | /miniprogram/dev/server/API/minidrama/vod_fileupload/api_pullupload.html |
| 查询任务 | /miniprogram/dev/server/API/minidrama/vod_fileupload/api_gettask.html |
| 申请分片上传 | /miniprogram/dev/server/API/minidrama/vod_fileupload/api_applyupload.html |
| 上传分片 | /miniprogram/dev/server/API/minidrama/vod_fileupload/api_uploadpart.html |
| 确认上传 | /miniprogram/dev/server/API/minidrama/vod_fileupload/api_commitupload.html |
| 获取媒资列表 | /miniprogram/dev/server/API/minidrama/vod_media/api_listmedia.html |
| 获取媒资详细信息 | /miniprogram/dev/server/API/minidrama/vod_media/api_getmedia.html |
| 获取媒资播放链接 | /miniprogram/dev/server/API/minidrama/vod_media/api_getmedialink.html |
| 删除媒资 | /miniprogram/dev/server/API/minidrama/vod_media/api_deletemedia.html |
| 剧目提审 | /miniprogram/dev/server/API/minidrama/auditdrama/api_auditdrama.html |
| 获取剧目列表 | /miniprogram/dev/server/API/minidrama/auditdrama/api_listdramas.html |
| 获取剧目信息 | /miniprogram/dev/server/API/minidrama/auditdrama/api_getdrama.html |
| 提交替换剧集审核 | /miniprogram/dev/server/API/minidrama/auditdrama/api_submitreplacemedias.html |
| 替换审核通过的剧集 | /miniprogram/dev/server/API/minidrama/auditdrama/api_replacedramamedia.html |
| 修改剧目基本信息 | /miniprogram/dev/server/API/minidrama/auditdrama/api_submitmodifydramabasicinforeq.html |
| 查询剧目审核信息 | /miniprogram/dev/server/API/minidrama/auditdrama/api_getdramalatestauditinfo.html |
| 查询CDN用量数据 | /miniprogram/dev/server/API/minidrama/usagedata/api_getcdnusagedata.html |
| 查询CDN日志下载链接列表 | /miniprogram/dev/server/API/minidrama/usagedata/api_getcdnlogs.html |
| 查询流量包详情 | /miniprogram/dev/server/API/minidrama/usagedata/api_listpackages.html |
| 查询被授权信息 | /miniprogram/dev/server/API/minidrama/authorizedrama/api_getauthorizedobjects.html |
| 增加剧目授权 | /miniprogram/dev/server/API/minidrama/authorizedrama/api_authorizedrama.html |
| 解除剧目授权 | /miniprogram/dev/server/API/minidrama/authorizedrama/api_deauthorizedrama.html |
| 查询授权信息 | /miniprogram/dev/server/API/minidrama/authorizedrama/api_getauthorizeobjects.html |
| 增加账号授权 | /miniprogram/dev/server/API/minidrama/authorizeapp/api_authorizeapp.html |
| 解除账号授权 | /miniprogram/dev/server/API/minidrama/authorizeapp/api_deauthorizeapp.html |
| 查询账号授权信息 | /miniprogram/dev/server/API/minidrama/authorizeapp/api_getauthorizeapps.html |
| 查询被授权信息 | /miniprogram/dev/server/API/minidrama/authorizeapp/api_getauthorizedobjects.html |
| 增加版权授权 | /miniprogram/dev/server/API/minidrama/authorizecopyright/api_authorizecopyright.html |
| 解除版权授权 | /miniprogram/dev/server/API/minidrama/authorizecopyright/api_deauthorizecopyright.html |
| 查询版权授权信息 | /miniprogram/dev/server/API/minidrama/authorizecopyright/api_getcopyrightauthorizationlist.html |
| 查询被版权授权信息 | /miniprogram/dev/server/API/minidrama/authorizecopyright/api_getcopyrightauthorizedlist.html |
## 短剧播放器

| 名称 | URL |
| --- | --- |
| 播放原始视频-推荐位控制接口 | /miniprogram/dev/server/API/dramaOthersAPI/api_setplayerdramarecmdswitch.html |
| 刷剧剧目设置 | /miniprogram/dev/server/API/dramaOthersAPI/api_developersetflushdrama.html |
| 推荐剧目 | /miniprogram/dev/server/API/dramaOthersAPI/api_developersetrecmddrama.html |
| 短剧上架 | /miniprogram/dev/server/API/dramaOthersAPI/api_developerpublishdrama.html |
| 获取已上架短剧 | /miniprogram/dev/server/API/dramaOthersAPI/api_developergetpublisheddrama.html |
| 设置短剧变现类型 | /miniprogram/dev/server/API/dramaOthersAPI/api_developersetiaadrama.html |
| 获取短剧变现类型 | /miniprogram/dev/server/API/dramaOthersAPI/api_developergetiaadrama.html |
| 批处理短剧合作推广计划 | /miniprogram/dev/server/API/dramaOthersAPI/api_batchprocessdramapromotion.html |
| 获取短剧合作推广活动 | /miniprogram/dev/server/API/dramaOthersAPI/api_getfinderevent.html |
## 小说作品管理

| 名称 | URL |
| --- | --- |
| 创建作品 | /miniprogram/dev/server/API/novel/business/api_createbook.html |
| 编辑作品 | /miniprogram/dev/server/API/novel/business/api_updatebook.html |
| 删除作品 | /miniprogram/dev/server/API/novel/business/api_deletebook.html |
| 获取作品列表 | /miniprogram/dev/server/API/novel/business/api_listbook.html |
| 获取作品信息 | /miniprogram/dev/server/API/novel/business/api_getbook.html |
| 上传章节 | /miniprogram/dev/server/API/novel/business/api_createchapter.html |
| 批量上传章节 | /miniprogram/dev/server/API/novel/business/api_batchcreatechapter.html |
| 删除章节 | /miniprogram/dev/server/API/novel/business/api_deletechapter.html |
| 替换章节 | /miniprogram/dev/server/API/novel/business/api_replacechapter.html |
| 获取章节列表 | /miniprogram/dev/server/API/novel/business/api_listchapter.html |
| 获取章节信息 | /miniprogram/dev/server/API/novel/business/api_getchapter.html |
| 调整章节顺序 | /miniprogram/dev/server/API/novel/business/api_reorderchapter.html |
| 调整章节相对顺序 | /miniprogram/dev/server/API/novel/business/api_updatechapterseq.html |
| 作品提审 | /miniprogram/dev/server/API/novel/business/api_auditbook.html |
| 新增账号-小说授权 | /miniprogram/dev/server/API/novel/auth/api_addbookauth.html |
| 查看主授权关系列表-查看被授权关系列表 | /miniprogram/dev/server/API/novel/auth/api_querybookauth.html |
| 删除指定的授权关系 | /miniprogram/dev/server/API/novel/auth/api_delbookauth.html |
| 新增账号-账号授权 | /miniprogram/dev/server/API/novel/auth/api_addbookauthbyappid.html |
| 查看账号主授权关系列表 | /miniprogram/dev/server/API/novel/auth/api_querybookauthv2.html |
| 删除指定的账号授权关系 | /miniprogram/dev/server/API/novel/auth/api_delbookauthbyappid.html |
| 预览设置修改 | /miniprogram/dev/server/API/novel/preview/api_setpreviewsetting.html |
| 预览设置获取 | /miniprogram/dev/server/API/novel/preview/api_getpreviewsetting.html |
| 读后推荐 | /miniprogram/dev/server/API/novel/other/api_novelreadersetrecmdnovel.html |
## 微信学生身份快速验证

| 名称 | URL |
| --- | --- |
| 快速获取学生身份 | /miniprogram/dev/server/API/student/api_quickcheckstudentidentity.html |
## 城市服务

| 名称 | URL |
| --- | --- |
| 获取城市服务限定页面链接 | /miniprogram/dev/server/API/cityservice/basic/api_cityserviceservicehomepath.html |
| 消息通路发消息 | /miniprogram/dev/server/API/cityservice/basic/api_cityservice_sendmsgdata.html |
| 校验实名信息 | /miniprogram/dev/server/API/cityservice/basic/api_checkrealnameinfo.html |
| 仿原生跳转 | /miniprogram/dev/server/API/cityservice/basic/api_transportcode_getbusinessview.html |
| 消息推送接口 | /miniprogram/dev/server/API/cityservice/medicalassistant/api_cityservice_sendchannelmsg.html |
| 查询用户实名API | /miniprogram/dev/server/API/cityservice/elderMedical/api_cityservice_getmedrealname.html |
| 查询用户是否开通 | /miniprogram/dev/server/API/cityservice/elderMedical/api_cityservice_getmsgrelation.html |
| 查询所有公告 | /miniprogram/dev/server/API/cityservice/elderMedical/api_intp_eldermed_gethospnoticelist.html |
| 公告草稿预览设置 | /miniprogram/dev/server/API/cityservice/elderMedical/api_previewhopsnotice.html |
| 正式发布公告 | /miniprogram/dev/server/API/cityservice/elderMedical/api_intp_eldermed_publichopsnotice.html |
| 添加公告草稿 | /miniprogram/dev/server/API/cityservice/elderMedical/api_sethopsnotice.html |
## 附近小程序

| 名称 | URL |
| --- | --- |
| 添加地点 | /miniprogram/dev/server/API/nearby-poi/api_addnearbypoi.html |
| 删除地点 | /miniprogram/dev/server/API/nearby-poi/api_deletenearbypoi.html |
| 查看地点 | /miniprogram/dev/server/API/nearby-poi/api_getnearbypoilist.html |
| 设置展示状态 | /miniprogram/dev/server/API/nearby-poi/api_setshowstatus.html |
## 小程序直播

| 名称 | URL |
| --- | --- |
| 创建直播间 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_createroom.html |
| 获取直播间列表和回放 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_getliveinfo.html |
| 删除直播间 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_deleteroom.html |
| 导入商品 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_importgoods.html |
| 编辑直播间 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_editroom.html |
| 获取直播间推流地址 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_getpushurl.html |
| 获取直播间分享二维码 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_getsharedcode.html |
| 获取主播副号 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_getsubanchor.html |
| 修改主播副号 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_modifysubanchor.html |
| 删除主播副号 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_deletesubanchor.html |
| 添加主播副号 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_addsubanchor.html |
| 删除直播间商品 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_deletedoods.html |
| 推送商品 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_pushgoods.html |
| 上下架商品 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_salegoods.html |
| 直播间商品排序 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_sortgoods.html |
| 修改直播间小助手 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_modifyassistant.html |
| 查询直播间小助手 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_getassistantlist.html |
| 删除直播间小助手 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_removeassistant.html |
| 添加管理直播间小助手 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_addveassistant.html |
| 禁言管理 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_updatecomment.html |
| 官方收录管理 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_updatefeedpublic.html |
| 客服功能管理 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_updatekf.html |
| 回放功能管理 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_updatereplay.html |
| 下载商品讲解视频 | /miniprogram/dev/server/API/livebroadcast/studio-management/api_downloadgoodsvideo.html |
| 直播挂件设置全局KEY | /miniprogram/dev/server/API/livebroadcast/studio-management/api_setdefault_goodskey.html |
| 直播挂件获取全局KEY | /miniprogram/dev/server/API/livebroadcast/studio-management/api_getdefault_goodskey.html |
| 添加并提审商品 | /miniprogram/dev/server/API/livebroadcast/commodity-management/api_addgoods.html |
| 重新提交商品审核 | /miniprogram/dev/server/API/livebroadcast/commodity-management/api_resubmitaudit.html |
| 获取商品的信息与审核状态 | /miniprogram/dev/server/API/livebroadcast/commodity-management/api_getgoodsauditinfo.html |
| 撤回商品审核 | /miniprogram/dev/server/API/livebroadcast/commodity-management/api_resetaudit.html |
| 更新商品 | /miniprogram/dev/server/API/livebroadcast/commodity-management/api_updategoodsinfo.html |
| 获取商品列表 | /miniprogram/dev/server/API/livebroadcast/commodity-management/api_getgoodsinfo.html |
| 删除商品 | /miniprogram/dev/server/API/livebroadcast/commodity-management/api_deletegoodsinfo.html |
| 设置成员角色 | /miniprogram/dev/server/API/livebroadcast/role-management/api_getrolelistdw.html |
| 移除成员角色 | /miniprogram/dev/server/API/livebroadcast/role-management/api_deleterole.html |
| 查询成员列表 | /miniprogram/dev/server/API/livebroadcast/role-management/api_getrolelist.html |
| 发送直播开始事件 | /miniprogram/dev/server/API/livebroadcast/subscribe-management/api_pushmessage.html |
| 获取长期订阅用户 | /miniprogram/dev/server/API/livebroadcast/subscribe-management/api_getfollowers.html |
## 图像处理与文字识别

| 名称 | URL |
| --- | --- |
| 图片智能裁剪 | /miniprogram/dev/server/API/img-ocr/img/api_imgaicrop.html |
| 二维码/条码识别 | /miniprogram/dev/server/API/img-ocr/img/api_imgqrcode.html |
| 图片高清化 | /miniprogram/dev/server/API/img-ocr/img/api_imgsuperresolution.html |
| 通用印刷体识别 | /miniprogram/dev/server/API/img-ocr/ocr/api_commocr.html |
| 行驶证识别 | /miniprogram/dev/server/API/img-ocr/ocr/api_drivingocr.html |
| 银行卡识别 | /miniprogram/dev/server/API/img-ocr/ocr/api_bankcardocr.html |
| 营业执照识别 | /miniprogram/dev/server/API/img-ocr/ocr/api_bizlicenseocr.html |
| 驾驶证识别 | /miniprogram/dev/server/API/img-ocr/ocr/api_drivinglicenseocr.html |
| 身份证识别 | /miniprogram/dev/server/API/img-ocr/ocr/api_idcardocr.html |
## 微信红包封面

| 名称 | URL |
| --- | --- |
| 获取微信红包封面 | /miniprogram/dev/server/API/red-packet-cover/api_getredpacketcoverurl.html |
## 微信服务市场

| 名称 | URL |
| --- | --- |
| 调用服务市场接口 | /miniprogram/dev/server/API/wx-service-market/api_invokeservice.html |
| 异步获取处理数据 | /miniprogram/dev/server/API/wx-service-market/api_servicemarketretrieve.html |
## 生物认证

| 名称 | URL |
| --- | --- |
| 生物认证秘钥签名验证 | /miniprogram/dev/server/API/soter/api_verifysignature.html |
## 微信人脸核身

| 名称 | URL |
| --- | --- |
| 获取用户人脸核身会话唯一标识 | /miniprogram/dev/server/API/face/api_getverifyid.html |
| 查询用户人脸核身真实验证结果 | /miniprogram/dev/server/API/face/api_queryverifyinfo.html |
## 微信搜一搜

| 名称 | URL |
| --- | --- |
| 搜一搜数据推送 | /miniprogram/dev/server/API/wxsearch/api_submitpages.html |
## 虚拟支付

| 名称 | URL |
| --- | --- |
| 查询代币余额 | /miniprogram/dev/server/API/VirtualPayment/api_query_user_balance.html |
| 扣减代币 | /miniprogram/dev/server/API/VirtualPayment/api_currency_pay.html |
| 查询创建的订单 | /miniprogram/dev/server/API/VirtualPayment/api_query_order.html |
| 代币支付退款 | /miniprogram/dev/server/API/VirtualPayment/api_cancel_currency_pay.html |
| 通知已发货完成 | /miniprogram/dev/server/API/VirtualPayment/api_notify_provide_goods.html |
| 代币赠送 | /miniprogram/dev/server/API/VirtualPayment/api_present_currency.html |
| 下载小程序账单 | /miniprogram/dev/server/API/VirtualPayment/api_download_bill.html |
| 启动订单退款任务 | /miniprogram/dev/server/API/VirtualPayment/api_refund_order.html |
| 创建提现单 | /miniprogram/dev/server/API/VirtualPayment/api_create_withdraw_order.html |
| 查询提现单 | /miniprogram/dev/server/API/VirtualPayment/api_query_withdraw_order.html |
| 批量上传道具 | /miniprogram/dev/server/API/VirtualPayment/api_start_upload_goods.html |
| 查询批量上传道具任务 | /miniprogram/dev/server/API/VirtualPayment/api_query_upload_goods.html |
| 启动批量发布道具任务 | /miniprogram/dev/server/API/VirtualPayment/api_start_publish_goods.html |
| 查询批量发布道具任务 | /miniprogram/dev/server/API/VirtualPayment/api_query_publish_goods.html |
| 查询商家账户可提现余额 | /miniprogram/dev/server/API/VirtualPayment/api_query_biz_balance.html |
| 查询广告金充值账户 | /miniprogram/dev/server/API/VirtualPayment/api_query_transfer_account.html |
| 查询广告金发放记录 | /miniprogram/dev/server/API/VirtualPayment/api_query_adver_funds.html |
| 充值广告金 | /miniprogram/dev/server/API/VirtualPayment/api_create_funds_bill.html |
| 绑定广告金充值账户 | /miniprogram/dev/server/API/VirtualPayment/api_bind_transfer_accout.html |
| 查询广告金充值记录 | /miniprogram/dev/server/API/VirtualPayment/api_query_funds_bill.html |
| 查询广告金回收记录 | /miniprogram/dev/server/API/VirtualPayment/api_query_recover_bill.html |
| 获取投诉列表 | /miniprogram/dev/server/API/VirtualPayment/api_get_complaint_list.html |
| 获取投诉详情 | /miniprogram/dev/server/API/VirtualPayment/api_get_complaint_detail.html |
| 获取协商历史 | /miniprogram/dev/server/API/VirtualPayment/api_get_negotiation_history.html |
| 回复用户 | /miniprogram/dev/server/API/VirtualPayment/api_response_complaint.html |
| 完成投诉处理 | /miniprogram/dev/server/API/VirtualPayment/api_complete_complaint.html |
| 上传媒体文件 | /miniprogram/dev/server/API/VirtualPayment/api_upload_vp_file.html |
| 获取微信支付反馈投诉图片的签名头部 | /miniprogram/dev/server/API/VirtualPayment/api_get_upload_file_sign.html |
| 下载广告金对应的商户订单信息 | /miniprogram/dev/server/API/VirtualPayment/api_download_adverfunds_order.html |
| 查询签约关系 | /miniprogram/dev/server/API/VirtualPayment/api_query_subscribe_contract.html |
| 预通知扣款 | /miniprogram/dev/server/API/VirtualPayment/api_send_subscribe_pre_payment.html |
| 发起订阅扣款 | /miniprogram/dev/server/API/VirtualPayment/api_submit_subscribe_pay_order.html |
| 商家解约 | /miniprogram/dev/server/API/VirtualPayment/api_cancel_subscribe_contract.html |
| 下载支付订单 | /miniprogram/dev/server/API/VirtualPayment/api_start_download_order.html |
| 查询下载订单任务 | /miniprogram/dev/server/API/VirtualPayment/api_query_download_order.html |
| 下载虚拟支付ios月结账单 | /miniprogram/dev/server/API/VirtualPayment/api_download_ios_settlement_bill.html |
| 商户被管控原因查询 | /miniprogram/dev/server/API/VirtualPayment/api_query_punishment_reasons.html |
