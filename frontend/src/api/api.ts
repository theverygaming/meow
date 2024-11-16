const API_URL = "/api";
const API_TIMEOUT = 5000;

export async function getData(route: string): Promise<Object> {
    const url = API_URL + route;
    const response = await fetch(url, { signal: AbortSignal.timeout(API_TIMEOUT) });
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

export async function postData(route: string, data: Object): Promise<Object> {
    const url = API_URL + route;
    const request = new Request(url, {
        method: "POST",
        body: JSON.stringify(data),
        headers: {
            "Content-Type": "application/json",
        }
    });

    const response = await fetch(request, { signal: AbortSignal.timeout(API_TIMEOUT) });
    if (!response.ok) {
        throw new Error(`Failure! Response status: ${response.status}`);
    }

    const json = await response.json();
    console.log(json);
    return json;
}
