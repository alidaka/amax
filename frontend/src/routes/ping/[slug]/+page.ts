import { debug } from "svelte/internal";
import type { PageLoad } from "./$types";

export const load = (async ({ params, fetch }) => {
    return {
        slug: params.slug,
        answer: new Promise(resolve => {
            fetch(`/api/ping/${params.slug}`)
            .then(r => r.json())
            .then(r => {
                resolve(r.data);
            })
        })
    };
}) satisfies PageLoad;