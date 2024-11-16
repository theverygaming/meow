import { getData, postData } from './api';

export async function getType(id: string) {
    return await getData(`/brainlog/type/get?id=${id}`);
}

export async function createLog(data) {
    await postData("/brainlog/create", data);
}

export async function getLogsList(page: number, pageSize: number) {
    let data = await getData(`/brainlog/list?page=${page-1}&pagesize=${pageSize}`);
    return data;
}

export async function updateLog(id: string, data) {
    await postData(`/brainlog/update?id=${id}`, data);
}

export async function deleteLog(id: string) {
    await getData(`/brainlog/delete?id=${id}`);
}
