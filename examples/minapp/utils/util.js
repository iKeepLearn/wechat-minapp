import {baseUrl} from "../constants"
const formatTime = date => {
    const year = date.getFullYear()
    const month = date.getMonth() + 1
    const day = date.getDate()
    const hour = date.getHours()
    const minute = date.getMinutes()
    const second = date.getSeconds()

    return `${[year, month, day].map(formatNumber).join('/')} ${[hour, minute, second].map(formatNumber).join(':')}`
}

const formatNumber = n => {
    n = n.toString()
    return n[1] ? n : `0${n}`
}

// 网络请求
async function request(url, data = {}, method, needToken = false, options = null) {
    const defaultOption = {
        showLoading: true,
        loadText: '加载中...'
    }

    options = options ? {
        ...defaultOption,
        ...options
    } : defaultOption

    if (options.showLoading) {
        wx.showLoading({
            title: options.loadText,
        })
    }

    let header = {
        'Content-Type': 'application/json;charset=UTF-8'
    }
    wx.showNavigationBarLoading();
    if (needToken) {
        const token = await getToken()
        header["Authorization"] = `Bearer ${token}`
    }

    const hasQuery = (url.split("?")).length > 1
    const timestamp = hasQuery ? `&ts=${Date.now()}` : `?ts=${Date.now()}`

    const curl = `${baseUrl}/${url}${timestamp}`

    return new Promise(function (resolve, reject) {
        wx.request({
            url: curl,
            method: method,
            header,
            data: data,
            // timeout: 2000,
            success(res) {
                console.log('api', res)
                wx.hideNavigationBarLoading();
                if (res.statusCode >= 200 && res.statusCode < 400) {
                    console.info(`接口【${baseUrl}/${url}】请求成功，请求参数为：`, data, `服务端返回：`, res.data);
                    resolve(res.data)
                } else {
                    console.info(`接口【${baseUrl}/${url}】请求失败，code:${res.statusCode},请求参数为：`, data, `服务端返回：`, res);
                    let errorMessage = res?.data?.message || res?.message || '请稍候再试'
                    if (isArray(errorMessage)) {
                        errorMessage = errorMessage.join(' ')
                    }
                    if (!isString(errorMessage)) {
                        errorMessage = errorMessage.toString()
                    }
                    wx.showToast({
                        title: errorMessage,
                        icon: "none"
                    })
                    setTimeout(() => {
                        reject(res);
                    }, 1200)
                }
            },
            fail(err) {
                console.log(`接口【${baseUrl}/${url}】请求失败，请求参数为：`, data, "失败原因", err)
                reject(err)
            },
            complete() {
                setTimeout(() => {
                    wx.hideLoading()
                }, 3000)
            }
        })
    })
};

function getWxLoginCode() {
    return new Promise((resolve, reject) => {
        wx.login({
            timeout: 2000,
            success: (res) => {
                if (res.code) {
                    resolve(res.code)
                }
            }
        })
    })

}



async function requestWithRetry(url, data = {}, method, needToken = false, options = null) {
    const defaultOption = {
        needToken: false,
        retry: 3,
        delay: 1000,
        showLoading: false,
        loadText: '加载中...'
    }


    options = options ? {
        ...defaultOption,
        ...options
    } : defaultOption

    const result = await retryInvocation(async () => {
        return await request(url, data, method, needToken, options)
    }, options.retry, options.delay)
    return result
}

async function retryInvocation(fn, retries, delay = 1000) {
    console.log({
        retries
    })
    try {
        return await fn()
    } catch (err) {
        console.log('retry failure')
        if (retries === 0) {
            throw err
        }
        // await _delay(delay)
        return await retryInvocation(fn, retries - 1, delay)
    }
}


module.exports = {
    formatTime,
    request,
    requestWithRetry,
    getWxLoginCode
}