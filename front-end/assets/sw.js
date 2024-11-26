// sw.js
self.addEventListener('install', event => {
    event.waitUntil(
        caches.open('my-cache').then(cache => {
            return cache.addAll([
                '/',
                '/main.css',
                'tailwind.css',
                'adminHeader.js',
                'canvas.js',
                '/index.html',
                '/styles.css',
                '/main.js'
            ]);
        })
    );
});

self.addEventListener('fetch', event => {
    event.respondWith(
        caches.match(event.request).then(response => {
            return response || fetch(event.request);
        })
    );
});