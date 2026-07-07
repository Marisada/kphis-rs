const VERSION = '680716-1810'

import init from './client.js'
init({url:'client_bg.wasm'}).catch(console.error)

const SWHelper = {
    async getWaitingWorker() {
        const registrations = await navigator?.serviceWorker?.getRegistrations() || []
        const registrationWithWaiting = registrations.find(reg => reg.waiting)
        return registrationWithWaiting?.waiting
    },
    async skipWaiting() {
        return (await SWHelper.getWaitingWorker())?.postMessage({ type: 'SKIP_WAITING' })
    },
    async skipWaitingSolo() {
        return (await SWHelper.getWaitingWorker())?.postMessage({ type: 'SKIP_WAITING_WHEN_SOLO' })
    },
}

window.addEventListener('beforeunload', async () => {
    if (window.swNeedUpdate) {
        await SWHelper.skipWaitingSolo()
    }
})

document.addEventListener('keydown', (e) => {
    if (e.key === 'Enter') {
        const focusedElm = document.activeElement
        if (focusedElm.tagName === 'BUTTON') {
            focusedElm.click()
        }
    }
})

const errormessage = document.getElementById('errormessage')
if (!navigator.serviceWorker) {
    errormessage.innerText = 'Service Worker is not supported in this browser.\nThis application cannot function without it.'
} else if (!window.fetch) {
    errormessage.innerText = 'Fetch API is not supported in this browser.\nThis application cannot function without it.'
} else if (!navigator.cookieEnabled) {
    errormessage.innerText = 'Cookies is not enabled in this browser.\nPlease eneble Cookies in Browser config.'
} else {
    const checkUpdate = document.getElementById('checkUpdate')
    navigator.serviceWorker.register('/sw.js').then(reg => {
        if (reg.waiting && reg.active) {
            window.swNeedUpdate = true
        }
        reg.onupdatefound = () => {
            const installingWorker = reg.installing
            if (installingWorker == null) {
                return
            }
            installingWorker.onstatechange = () => {
                if (installingWorker.state === 'activated') {
                    SWHelper.skipWaiting().then()
                    window.localStorage.clear()
                    window.location.reload(true)
                }
                if (installingWorker.state === 'installed') {
                    if (navigator.serviceWorker.controller) {
                        window.swNeedUpdate = true;
                    }
                }
            }
        }
        checkUpdate.onclick = () => reg.update()
    }).catch(err => console.error('Service Worker Registration : ' + err))

    navigator.serviceWorker.ready.then(async reg => {
        if (reg.active !== null) {
            reg.active.postMessage({type:'VERSION', value: VERSION})
        }
    }).catch(err => console.error('Service Worker Ready : ' + err))
}
