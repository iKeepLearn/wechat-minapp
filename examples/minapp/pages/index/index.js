import {
    request,
    getWxLoginCode
} from "../../utils/util";

Page({
    data: {

        text: "",
    },

    async getOpenid() {
        const code = await getWxLoginCode()

        const result = await request("api/login", {
            code,
        }, "POST")
        console.log("get openid result", result)
        this.setData(result)
    },

    input(e) {
        this.setData({
            text: e.detail.value
        })
    },

    async checkMsg() {
        let text = this.data.text
        if (text.length < 2) {
            wx.showToast({
                title: '字数须大于2',
                icon: "none"
            })
            return
        }
        const code = await getWxLoginCode()

        const {
            result
        } = await request("api/msg_sec_check", {
            code,
            content: text
        }, "POST")
        console.log("msg_sec_check result", result)
        this.setData({
            check_result: result.label
        })
    },

    async getPhoneNumber(e) {
        let code = e.detail.code
        console.log(e.detail.code) // 动态令牌
        console.log(e.detail.errMsg) // 回调信息（成功失败都会返回）
        console.log(e.detail.errno) // 错误码（失败时返回）
        if (!code) {
            wx.showToast({
                title: e.detail.errMsg,
                icon: "none"
            })
            return
        }
        const result = await request("api/get_phone_num", {
            code,
        }, "POST")
        console.log("get openid result", result)
        this.setData(result)
    },

    async getQrCode(){
        const result = await request("api/get_user_qr", {}, "POST")
        console.log("getQrCode result", result)
     
            let data=wx.arrayBufferToBase64(result)
            let qr = 'data:image/png;base64,' + data
            this.setData({qr})
  
        
    }



})