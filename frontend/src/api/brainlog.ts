import { getData, postData } from './api';

export async function getType(id: string) {
    return await getData(`/brainlog/type/get?id=${id}`);
}

export async function getLogsList(page: number, pageSize: number) {
    await postData("/brainlog/create", {time: "2024-11-10T22:22:24Z", log_type: "something", body: "meow meow meow!"});
    let data = await getData(`/brainlog/list?page=${page-1}&pagesize=${pageSize}`);
    return data;
}
