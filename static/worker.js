const addResourcesToCache = async (resources) => {
    const cache = await caches.open("v1");
    await cache.addAll(resources);
};

self.addEventListener("install", (event) => {
    event.waitUntil(
        addResourcesToCache([
            "/idn/",
            "/idn/style.css",
            "/idn/data/vocabs.txt",
            "/idn/learn/",
            "/idn/lessons/01-basic-sentence-structure/",
            "/idn/lessons/02-pronouns/",
            "/idn/lessons/03-greetings-and-farewells/",
            "/idn/lessons/04-verb-modification/",
            "/idn/lessons/05-asking-questions/",
            "/idn/privacy/",
            "/idn/lessons/",
            "/idn/review/",
            "/idn/pkg/idn.js",
            "/idn/pkg/idn_bg.wasm",
        ]),
    );
    self.skipWaiting();
});

const serveFile = async (request, event) => {
    const cache = await caches.open("v1")
    const cached = await cache.match(request);
    const headers = {};

    try {
        if (cached) {
            const etag = cached.headers.get("ETag");
            if (etag) headers["If-None-Match"] = etag;
        }

        const response = await fetch(request, { headers });

        if (response.status === 304 && cached) {
            return cached;
        }

        if (response.ok) {
            const resp = response.clone();
            cache.put(request, resp.clone());
            return response;
        } else {
            return cached;
        }
      } catch (error) {
        return await caches.match(request);
    }
}

self.addEventListener("fetch", (event) => {
    event.respondWith(serveFile(event.request, event));
});
