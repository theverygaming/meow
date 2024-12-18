import { getApiKey, getData, postData, putData, deleteData } from './api';

export interface BrainlogObj {
    body: string;
    log_type: string;
    time: string;
};

export interface BrainlogObjId extends BrainlogObj {
    id: string;
}

export interface BrainlogObjList {
    items: BrainlogObjId[];
    total_items: number;
}

export async function createLog(data: BrainlogObj): Promise<BrainlogObjId> {
    return await postData("/brainlog/create", getApiKey(), data) as BrainlogObjId;
}

export async function getLogsList(page: number, pageSize: number): Promise<BrainlogObjList> {
    const params = new URLSearchParams();
    params.append("page", (page-1).toString());
    params.append("pagesize", (pageSize).toString());

    const data = await getData(`/brainlog?${params.toString()}`, getApiKey()) as BrainlogObjList;
    return data;
}

export async function updateLog(id: string, data: BrainlogObj) {
    await putData(`/brainlog/${id}`, getApiKey(), data);
}

export async function deleteLog(id: string) {
    await deleteData(`/brainlog/${id}`, getApiKey());
}
