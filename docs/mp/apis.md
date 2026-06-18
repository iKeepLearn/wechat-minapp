

## 基础能力（base）

| 名称           | URL                                                 |
| ------------ | --------------------------------------------------- |
| 获取接口调用凭据     | /doc/service/api/base/api_getaccesstoken.html       |
| 获取稳定版接口调用凭据  | /doc/service/api/base/api_getstableaccesstoken.html |
| 网络通信检测       | /doc/service/api/base/api_callbackcheck.html        |
| 获取微信API服务器IP | /doc/service/api/base/api_getapidomainip.html       |
| 获取微信推送服务器IP  | /doc/service/api/base/api_getcallbackip.html        |

---

## OpenAPI 管理（apimanage）

| 名称                   | URL                                                       |
| -------------------- | --------------------------------------------------------- |
| 重置API调用次数            | /doc/service/api/apimanage/api_clearquota.html            |
| 查询API调用额度            | /doc/service/api/apimanage/api_getapiquota.html           |
| 查询rid信息              | /doc/service/api/apimanage/api_getridinfo.html            |
| 使用AppSecret重置API调用次数 | /doc/service/api/apimanage/api_clearquotabyappsecret.html |
| 重置指定API调用次数          | /doc/service/api/apimanage/api_clearapiquota.html         |

---

## 自定义菜单（custommenu）

| 名称          | URL                                                         |
| ----------- | ----------------------------------------------------------- |
| 创建自定义菜单     | /doc/service/api/custommenu/api_createcustommenu.html       |
| 查询自定义菜单信息   | /doc/service/api/custommenu/api_getcurrentselfmenuinfo.html |
| 获取自定义菜单配置   | /doc/service/api/custommenu/api_getmenu.html                |
| 删除自定义菜单     | /doc/service/api/custommenu/api_deletemenu.html             |
| 创建个性化菜单     | /doc/service/api/custommenu/api_addconditionalmenu.html     |
| 删除个性化菜单     | /doc/service/api/custommenu/api_deleteconditionalmenu.html  |
| 测试个性化菜单匹配结果 | /doc/service/api/custommenu/api_trymatchmenu.html           |

---

## 消息与通知（notify）

### 群发消息（message）

| 名称           | URL                                                    |
| ------------ | ------------------------------------------------------ |
| 上传发表内容中的图片   | /doc/service/api/notify/message/api_uploadimage.html   |
| 删除群发消息       | /doc/service/api/notify/message/api_deletemassmsg.html |
| 获取群发速度       | /doc/service/api/notify/message/api_getspeed.html      |
| 查询群发消息发送状态   | /doc/service/api/notify/message/api_massmsgget.html    |
| 根据OpenID群发消息 | /doc/service/api/notify/message/api_masssend.html      |
| 预览消息         | /doc/service/api/notify/message/api_preview.html       |
| 根据标签群发消息     | /doc/service/api/notify/message/api_sendall.html       |
| 设置群发速度       | /doc/service/api/notify/message/api_setspeed.html      |
| 上传图文消息素材     | /doc/service/api/notify/message/api_uploadnewsmsg.html |

### 模板消息（template）

| 名称        | URL                                                           |
| --------- | ------------------------------------------------------------- |
| 发送模板消息    | /doc/service/api/notify/template/api_sendtemplatemessage.html |
| 选用模板      | /doc/service/api/notify/template/api_addtemplate.html         |
| 查询拦截模板消息  | /doc/service/api/notify/template/api_queryblocktmplmsg.html   |
| 删除模板      | /doc/service/api/notify/template/api_deletetemplate.html      |
| 获取已选用模板列表 | /doc/service/api/notify/template/api_getalltemplates.html     |
| 获取行业信息    | /doc/service/api/notify/template/api_getindustry.html         |
| 设置所属行业    | /doc/service/api/notify/template/api_setindustry.html         |

### 订阅通知（notify）

| 名称       | URL                                                               |
| -------- | ----------------------------------------------------------------- |
| 删除模板     | /doc/service/api/notify/notify/api_delwxanewtemplate.html         |
| 获取类目     | /doc/service/api/notify/notify/api_getcategory.html               |
| 获取模板关键词  | /doc/service/api/notify/notify/api_getpubnewtemplatekeywords.html |
| 获取类目模板   | /doc/service/api/notify/notify/api_getpubnewtemplatetitles.html   |
| 获取已有模板列表 | /doc/service/api/notify/notify/api_getwxapubnewtemplate.html      |
| 选用模板     | /doc/service/api/notify/notify/api_addwxanewtemplate.html         |
| 发送订阅通知   | /doc/service/api/notify/notify/api_sendnewsubscribemsg.html       |

### 一次性订阅

| 名称        | URL                                                          |
| --------- | ------------------------------------------------------------ |
| 发送一次性订阅消息 | /doc/service/api/notify/subscribe/api_templatesubscribe.html |

### 自动回复

| 名称       | URL                                                                  |
| -------- | -------------------------------------------------------------------- |
| 获取自动回复规则 | /doc/service/api/notify/autoreplies/api_getcurrentautoreplyinfo.html |

---

## 客服能力（customer）

### 客服管理

| 名称       | URL                                                               |
| -------- | ----------------------------------------------------------------- |
| 设置客服头像   | /doc/service/api/customer/servicermanage/api_uploadkfheadimg.html |
| 删除客服账号   | /doc/service/api/customer/servicermanage/api_delkfaccount.html    |
| 邀请绑定客服账号 | /doc/service/api/customer/servicermanage/api_invitekfworker.html  |
| 获取所有客服账号 | /doc/service/api/customer/servicermanage/api_getkflist.html       |
| 添加客服账号   | /doc/service/api/customer/servicermanage/api_addkfaccount.html    |
| 获取在线客服列表 | /doc/service/api/customer/servicermanage/api_getonlinekflist.html |
| 修改客服账号   | /doc/service/api/customer/servicermanage/api_updatekfaccount.html |

### 会话控制

| 名称        | URL                                                          |
| --------- | ------------------------------------------------------------ |
| 获取客服会话列表  | /doc/service/api/customer/messctrl/api_getkfsessionlist.html |
| 关闭会话      | /doc/service/api/customer/messctrl/api_closesession.html     |
| 创建会话      | /doc/service/api/customer/messctrl/api_createkfsession.html  |
| 获取客户会话状态  | /doc/service/api/customer/messctrl/api_getkfsession.html     |
| 获取未接入会话列表 | /doc/service/api/customer/messctrl/api_getwaitcase.html      |

### 客服消息

| 名称     | URL                                                          |
| ------ | ------------------------------------------------------------ |
| 获取聊天记录 | /doc/service/api/customer/message/api_getmsglist.html        |
| 客服输入状态 | /doc/service/api/customer/message/api_typing.html            |
| 发送客服消息 | /doc/service/api/customer/message/api_sendcustommessage.html |

---

## 素材管理（material）

### 永久素材

| 名称     | URL                                                           |
| ------ | ------------------------------------------------------------- |
| 获取永久素材 | /doc/service/api/material/permanent/api_getmaterial.html      |
| 获取素材总数 | /doc/service/api/material/permanent/api_getmaterialcount.html |
| 获取素材列表 | /doc/service/api/material/permanent/api_batchgetmaterial.html |
| 上传图片   | /doc/service/api/material/permanent/api_uploadimage.html      |
| 上传永久素材 | /doc/service/api/material/permanent/api_addmaterial.html      |
| 删除永久素材 | /doc/service/api/material/permanent/api_delmaterial.html      |

### 临时素材

| 名称       | URL                                                          |
| -------- | ------------------------------------------------------------ |
| 获取临时素材   | /doc/service/api/material/temporary/api_getmedia.html        |
| 新增临时素材   | /doc/service/api/material/temporary/api_uploadtempmedia.html |
| 获取高清语音素材 | /doc/service/api/material/temporary/api_gethdvoice.html      |

---

## 草稿与发布

### 草稿

| 名称     | URL                                                           |
| ------ | ------------------------------------------------------------- |
| 新增草稿   | /doc/service/api/draftbox/draftmanage/api_draft_add.html      |
| 更新草稿   | /doc/service/api/draftbox/draftmanage/api_draft_update.html   |
| 获取草稿列表 | /doc/service/api/draftbox/draftmanage/api_draft_batchget.html |
| 获取草稿详情 | /doc/service/api/draftbox/draftmanage/api_getdraft.html       |
| 删除草稿   | /doc/service/api/draftbox/draftmanage/api_draft_delete.html   |
| 获取草稿总数 | /doc/service/api/draftbox/draftmanage/api_draft_count.html    |

### 发布能力

| 名称      | URL                                                    |
| ------- | ------------------------------------------------------ |
| 发布草稿    | /doc/service/api/public/api_freepublish_submit.html    |
| 删除发布文章  | /doc/service/api/public/api_freepublishdelete.html     |
| 获取已发布列表 | /doc/service/api/public/api_freepublish_batchget.html  |
| 发布状态查询  | /doc/service/api/public/api_freepublish_get.html       |
| 获取图文信息  | /doc/service/api/public/api_freepublishgetarticle.html |

---

## 用户管理（usermanage）

### 标签

| 名称     | URL                                                     |
| ------ | ------------------------------------------------------- |
| 创建标签   | /doc/service/api/usermanage/tag/api_createtag.html      |
| 编辑标签   | /doc/service/api/usermanage/tag/api_updatetag.html      |
| 删除标签   | /doc/service/api/usermanage/tag/api_deletetag.html      |
| 获取标签   | /doc/service/api/usermanage/tag/api_gettags.html        |
| 获取标签粉丝 | /doc/service/api/usermanage/tag/api_gettagfans.html     |
| 批量打标签  | /doc/service/api/usermanage/tag/api_batchtagging.html   |
| 批量取消标签 | /doc/service/api/usermanage/tag/api_batchuntagging.html |
| 获取用户标签 | /doc/service/api/usermanage/tag/api_gettagidlist.html   |

### 用户信息

| 名称       | URL                                                            |
| -------- | -------------------------------------------------------------- |
| 获取用户信息   | /doc/service/api/usermanage/userinfo/api_userinfo.html         |
| 批量获取用户信息 | /doc/service/api/usermanage/userinfo/api_batchuserinfo.html    |
| 获取关注用户   | /doc/service/api/usermanage/userinfo/api_getfans.html          |
| 拉黑用户     | /doc/service/api/usermanage/userinfo/api_batchblacklist.html   |
| 取消拉黑     | /doc/service/api/usermanage/userinfo/api_batchunblacklist.html |
| 获取黑名单    | /doc/service/api/usermanage/userinfo/api_getblacklist.html     |
| 设置备注名    | /doc/service/api/usermanage/userinfo/api_updateremark.html     |

### openid

| 名称       | URL                                                            |
| -------- | -------------------------------------------------------------- |
| 转换openid | /doc/service/api/usermanage/changeopenid/api_changeopenid.html |

---

## 二维码与短链

### 二维码规则

| 名称      | URL                                                           |
| ------- | ------------------------------------------------------------- |
| 获取二维码规则 | /doc/service/api/qrcode/qrcodejump/api_qrcodejumpget.html     |
| 增加/修改规则 | /doc/service/api/qrcode/qrcodejump/api_qrcodejumpadd.html     |
| 发布规则    | /doc/service/api/qrcode/qrcodejump/api_qrcodejumppublish.html |
| 删除规则    | /doc/service/api/qrcode/qrcodejump/api_qrcodejumpdelete.html  |

### 二维码

| 名称    | URL                                                   |
| ----- | ----------------------------------------------------- |
| 生成二维码 | /doc/service/api/qrcode/qrcodes/api_createqrcode.html |

### 短链

| 名称    | URL                                                   |
| ----- | ----------------------------------------------------- |
| 长链转短链 | /doc/service/api/qrcode/shorten/api_genshortkey.html  |
| 短链转长链 | /doc/service/api/qrcode/shorten/api_fetchshorten.html |

---

## 数据统计（wedata）

（如果你需要我可以再帮你拆成“可视化报表结构/数据库结构/API SDK结构”三种版本）

### 用户数据

| 名称     | URL                                                   |
| ------ | ----------------------------------------------------- |
| 用户增减数据 | /doc/service/api/wedata/user/api_getusersummary.html  |
| 累计用户数据 | /doc/service/api/wedata/user/api_getusercumulate.html |

### 图文数据

| 名称       | URL                                                         |
| -------- | ----------------------------------------------------------- |
| 图文群发每日数据 | /doc/service/api/wedata/news/api_getarticlesummary.html     |
| 阅读概况     | /doc/service/api/wedata/news/api_getuserread.html           |
| 阅读分时     | /doc/service/api/wedata/news/api_getuserreadhour.html       |
| 转发概况     | /doc/service/api/wedata/news/api_getusershare.html          |
| 转发分时     | /doc/service/api/wedata/news/api_getusersharehour.html      |
| 群发总数据    | /doc/service/api/wedata/news/api_getarticletotal.html       |
| 内容阅读     | /doc/service/api/wedata/news/api_getarticleread.html        |
| 内容分享     | /doc/service/api/wedata/news/api_getarticleshare.html       |
| 内容概况     | /doc/service/api/wedata/news/api_getbizsummary.html         |
| 内容详情     | /doc/service/api/wedata/news/api_getarticletotaldetail.html |

### 消息数据

| 名称   | URL                                                           |
| ---- | ------------------------------------------------------------- |
| 消息概况 | /doc/service/api/wedata/mess/api_getupstreammsg.html          |
| 月数据  | /doc/service/api/wedata/mess/api_getupstreammsgmonth.html     |
| 周数据  | /doc/service/api/wedata/mess/api_getupstreammsgweek.html      |
| 分时数据 | /doc/service/api/wedata/mess/api_getupstreammsghour.html      |
| 分布数据 | /doc/service/api/wedata/mess/api_getupstreammsgdist.html      |
| 周分布  | /doc/service/api/wedata/mess/api_getupstreammsgdistweek.html  |
| 月分布  | /doc/service/api/wedata/mess/api_getupstreammsgdistmonth.html |

### 接口数据

| 名称     | URL                                                          |
| ------ | ------------------------------------------------------------ |
| 被动回复概要 | /doc/service/api/wedata/api/api_getinterfacesummary.html     |
| 被动回复分布 | /doc/service/api/wedata/api/api_getinterfacesummaryhour.html |

---

## 网页开发（webdev）

### 网页授权

| 名称     | URL                                                     |
| ------ | ------------------------------------------------------- |
| 换取授权凭证 | /doc/service/api/webdev/access/api_snsaccesstoken.html  |
| 检验授权凭证 | /doc/service/api/webdev/access/api_snsauth.html         |
| 获取用户信息 | /doc/service/api/webdev/access/api_snsuserinfo.html     |
| 刷新授权凭证 | /doc/service/api/webdev/access/api_snsrefreshtoken.html |

### JS-SDK

| 名称               | URL                                              |
| ---------------- | ------------------------------------------------ |
| 获取 JS-SDK ticket | /doc/service/api/webdev/jssdk/api_getticket.html |

---

## 智能能力（openpoc）

### AI

| 名称     | URL                                                         |
| ------ | ----------------------------------------------------------- |
| 微信翻译   | /doc/service/api/openpoc/ai/api_translatecontent.html       |
| 上传语音   | /doc/service/api/openpoc/ai/api_addvoicetorecofortext.html  |
| 语音识别结果 | /doc/service/api/openpoc/ai/api_queryrecoresultfortext.html |

### OCR

| 名称     | URL                                                     |
| ------ | ------------------------------------------------------- |
| 菜单识别   | /doc/service/api/openpoc/ocr/api_menuocr.html           |
| 通用识别   | /doc/service/api/openpoc/ocr/api_commocr.html           |
| 身份证识别  | /doc/service/api/openpoc/ocr/api_idcardocr.html         |
| 银行卡识别  | /doc/service/api/openpoc/ocr/api_bankcardocr.html       |
| 营业执照识别 | /doc/service/api/openpoc/ocr/api_bizlicenseocr.html     |
| 驾驶证识别  | /doc/service/api/openpoc/ocr/api_drivinglicenseocr.html |
| 行驶证识别  | /doc/service/api/openpoc/ocr/api_drivingocr.html        |

### 图像处理

| 名称    | URL                                               |
| ----- | ------------------------------------------------- |
| 图片裁剪  | /doc/service/api/openpoc/image/api_imgaicrop.html |
| 二维码识别 | /doc/service/api/openpoc/image/api_imgqrcode.html |
