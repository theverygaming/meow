import { getApiKey, getData, postData, putData, deleteData } from './api';

export interface QuestItemObj {
    quest_id: string;
    attributes: string;
    name: string;
    body: string;
};

export interface QuestItemObjId extends QuestItemObj {
    id: string;
}

export async function createQuestItem(data: QuestItemObj): Promise<QuestItemObjId> {
    return await postData("/quest/items/create", getApiKey(), data) as QuestItemObjId;
}

export async function getQuestItemsList(page: number, pageSize: number): Promise<QuestItemObjId[]> {
    let data = await getData(`/quest/items/list?page=${page-1}&pagesize=${pageSize}`, getApiKey()) as QuestItemObjId[];
    return data;
}

export async function updateQuestItem(id: string, data: QuestItemObj) {
    await putData(`/quest/items/${id}`, getApiKey(), data);
}

export async function deleteQuestItem(id: string) {
    await deleteData(`/quest/items/${id}`, getApiKey());
}
