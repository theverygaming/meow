import { ro } from "vuetify/locale";

const API_URL = "/api";
const API_TIMEOUT = 5000;

export function getApiKey(): string {
    const api_key = localStorage.getItem("API_KEY");
    console.log(api_key);
    if (!api_key) {
        throw new Error("missing API key");
    }
    return api_key;
}


export async function getData(route: string, api_key: string): Promise<Object> {
    const url = API_URL + route;
    const response = await fetch(
        url,
        {
            signal: AbortSignal.timeout(API_TIMEOUT),
            headers: {
                "X-API-KEY": api_key,
            }
        }
    );
    if (!response.ok) {
        throw new Error(`Failure! Response status: ${response.status}`);
    }
    try {
        const json = await response.json();
        console.log(json);
        return json;
    } catch {
        return {};
    }
}

async function updatePostCore(route: string, api_key: string, data: Object, method: string): Promise<Object> {
    const url = API_URL + route;
    const request = new Request(url, {
        method: method,
        body: JSON.stringify(data),
        headers: {
            "Content-Type": "application/json",
            "X-API-KEY": api_key,
        }
    });

    const response = await fetch(request, { signal: AbortSignal.timeout(API_TIMEOUT) });
    if (!response.ok) {
        throw new Error(`Failure! Response status: ${response.status}`);
    }

    try {
        const json = await response.json();
        console.log(json);
        return json;
    } catch {
        return {};
    }
}


export async function postData(route: string, api_key: string, data: Object): Promise<Object> {
    return await updatePostCore(route, api_key,data, "POST");
}

export async function putData(route: string, api_key: string, data: Object): Promise<Object> {
    return await updatePostCore(route, api_key,data, "PUT");
}

export async function deleteData(route: string, api_key: string): Promise<Object> {
    const url = API_URL + route;
    const request = new Request(url, {
        method: "DELETE",
        headers: {
            "Content-Type": "application/json",
            "X-API-KEY": api_key,
        }
    });

    const response = await fetch(request, { signal: AbortSignal.timeout(API_TIMEOUT) });
    if (!response.ok) {
        throw new Error(`Failure! Response status: ${response.status}`);
    }

    try {
        const json = await response.json();
        console.log(json);
        return json;
    } catch {
        return {};
    }
}