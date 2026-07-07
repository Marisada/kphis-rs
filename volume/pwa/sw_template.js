const CACHE_NAME = `RUSTY-KPHIS-${VERSION}`
const cachable = [
  'app.min.css',
  'app.js',
  'client.js',
  'client_bg.wasm',
  'favicon.ico',
  'manifest.webmanifest',
  'statics/css/font-awesome.min.css',
  'statics/css/font-awesome-solid.min.css',
  'statics/css/font-awesome-regular.min.css',
  'statics/css/bootstrap.min.css',
  'statics/css/viewer.min.css',
  'statics/icons/favicon-16x16.png',
  'statics/icons/favicon-32x32.png',
  'statics/icons/favicon-192x192.png',
  'statics/icons/maskable_icon_x144.png',
  'statics/picture/favicon/icon17.svg',
  'statics/picture/favicon/icon17.ico',
  'statics/webfonts/fa-solid-900.woff2',
  'statics/webfonts/fa-regular-400.woff2'
]

self.addEventListener('install', event => {
  console.info(`installing service worker "${CACHE_NAME}"`)
  self.skipWaiting()
  const requests = cachable.map(url => new Request(url, { cache: 'no-store' }))
  event.waitUntil(
    caches.open(CACHE_NAME).then(cache => {
      for (const request of requests) {
        const response = fetch(request)
        if (response.ok) {
          cache.put(request, response)
        }
      }
    })
  )
})

self.addEventListener('activate', event => {
  console.info(`activating service worker "${CACHE_NAME}"`)
  const activate = async () => {
    await clients.claim()
    caches.keys().then(keyList => {
      return Promise.all(
        keyList.map(key => {
          if (key !== CACHE_NAME) {
            return caches.delete(key)
          }
        })
      )
    })
  }
  event.waitUntil(activate())
})

self.addEventListener('message', event => {
  if (event.data && event.data.type === 'SKIP_WAITING') {
    self.skipWaiting()
  }
  if (event.data && event.data.type === 'SKIP_WAITING_WHEN_SOLO') {
    self.clients.matchAll({includeUncontrolled: true}).then(clients => {
      if (clients.length < 2) {
        self.skipWaiting()
      }
    })
  }
  if (event.data && event.data.type === 'VERSION') {
    console.info(`Service worker "${CACHE_NAME}" registered`)
    console.info(`App version "${event.data.value}" activated`)
  }
})

let waitTime = 0
let asset_etag

self.addEventListener('fetch', event => {
  if (event.request.headers.get('Accept') === 'text/event-stream') {
    return
  }
  event.respondWith(handleFetch(event.request))
})

async function handleFetch(request) {
  const urls = request.url.split('/')
  if (urls.includes('api')) {
    // NO CACHE
    let fetch_fn = fetch(request).then(resp => {
      if (resp.status === 429) {
        const retryAfter = resp.headers.get('Retry-After') || "1"
        const retryAfterInt = parseInt(retryAfter)
        waitTime = retryAfterInt > 0 ? retryAfterInt * 1000 : 1000
        return delay(waitTime).then(() => {
          waitTime = 0
          return fetch(request).catch(err => console.error(err))
        })
      } else {
        waitTime = 0
        return resp
      }
    }).catch(err => console.error(err))
    if (waitTime > 0) {
      return delay(waitTime).then(() => {
        waitTime = 0
        return fetch_fn
      })
    } else {
      return fetch_fn
    }
  } else if (urls.includes('assets')) {
    // DISK CACHE / NETWORK FIRST + ETAG
    let assetReq
    if (asset_etag) {
      assetReq = fetch(request, {headers: {"If-None-Match": asset_etag}})
    } else {
      assetReq = fetch(request)
    }
    let fetch_fn = assetReq.then(resp => {
      const clone = resp.clone()
      if (resp.status === 304) {
        return caches.open(CACHE_NAME).then(cache => cache.match(request))
      } else if (resp.status === 429) {
        const retryAfter = resp.headers.get('Retry-After') || "1"
        const retryAfterInt = parseInt(retryAfter)
        const waitTime = retryAfterInt > 0 ? retryAfterInt * 1000 : 1000
        return delay(waitTime).then(() => {
          return assetReq.then(resp2 => {
            const clone2 = resp2.clone()
            if (resp2.status === 304) {
              return caches.open(CACHE_NAME).then(cache => cache.match(request))
            } else {
              if (resp2.status === 200) {
                asset_etag = resp2.headers.get('etag')
                caches.open(CACHE_NAME).then(cache => cache.put(request, clone2))
              }
              return resp
            }
          }).catch(err => console.error(err))
        })
      } else {
        if (resp.status === 200) {
          asset_etag = resp.headers.get('etag')
          caches.open(CACHE_NAME).then(cache => cache.put(request, clone))
        }
        return resp
      }
    }).catch(err => console.error(err))
    if (waitTime > 0) {
      return delay(waitTime).then(() => {
        waitTime = 0
        return fetch_fn
      })
    } else {
      return fetch_fn
    }
  } else {
    // CACHE FIRST
    return caches.open(CACHE_NAME)
      .then(cache => cache.match(request))
      .then(response => response || lazyCache(request))
  }
}

async function lazyCache(request) {
  const response = await fetch(request).catch(err => console.error(err))
  const clone = response.clone()
  if (response.status === 200) {
    caches.open(CACHE_NAME).then(cache => cache.put(request, clone))
  }
  return response
}

function delay(t) {
  return new Promise(resolve => setTimeout(resolve, t))
}