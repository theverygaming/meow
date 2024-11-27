import { getApiKey, getData, postData, putData, deleteData } from './api';

export interface QuestObj {
    name: string;
};

export interface QuestObjId extends QuestObj {
    id: string;
}

export async function createQuest(data: QuestObj): Promise<QuestObjId> {
    return await postData("/quest/create", getApiKey(), data) as QuestObjId;
}

export async function getQuestsList(page: number, pageSize: number): Promise<QuestObjId[]> {
    let data = await getData(`/quest?page=${page-1}&pagesize=${pageSize}`, getApiKey()) as QuestObjId[];
    return data;
}

export async function updateQuest(id: string, data: QuestObj) {
    await putData(`/quest/${id}`, getApiKey(), data);
}

export async function deleteQuest(id: string) {
    await deleteData(`/quest/${id}`, getApiKey());
}
