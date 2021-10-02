import type {
	IMongoDBCollection,
	IMongoDBDocument,
	IMongoDBDocumentCreated,
	IMongoDBDocumentDeleted,
	IMongoDBDocumentUpdated,
} from '$lib/models/mongodb';
import { getClient } from './client';

export async function createCollection(
	projectId: string,
	collectionName: string,
): Promise<boolean> {
	const res = await getClient().post(
		`/projects/services/${encodeURIComponent(projectId)}/mongodb/collections/${encodeURIComponent(
			collectionName,
		)}/create`,
		{},
	);
	return res.data;
}

export async function listCollections(projectId: string): Promise<IMongoDBCollection[]> {
	const res = await getClient().get(
		`/projects/services/${encodeURIComponent(projectId)}/mongodb/collections`,
	);
	return res.data;
}

export async function dropCollection(projectId: string, collectionName: string): Promise<boolean> {
	const res = await getClient().post(
		`/projects/services/${encodeURIComponent(projectId)}/mongodb/collections/${encodeURIComponent(
			collectionName,
		)}/drop`,
		{},
	);
	return res.data;
}

export async function getDocuments<T extends IMongoDBDocument>(
	projectId: string,
	collectionName: string,
	filter?: object,
	options?: object,
): Promise<T[]> {
	const res = await getClient().post(
		`/projects/services/${encodeURIComponent(projectId)}/mongodb/collections/${encodeURIComponent(
			collectionName,
		)}/documents`,
		{ filter, options },
	);
	return res.data;
}

export async function createDocument(
	projectId: string,
	collectionName: string,
	document: object,
): Promise<IMongoDBDocumentCreated> {
	const res = await getClient().post(
		`/projects/services/${encodeURIComponent(projectId)}/mongodb/collections/${encodeURIComponent(
			collectionName,
		)}/documents/create`,
		{ document },
	);
	return res.data;
}

export async function getDocument<T extends IMongoDBDocument>(
	projectId: string,
	collectionName: string,
	documentId: string,
): Promise<T> {
	const res = await getClient().post(
		`/projects/services/${encodeURIComponent(projectId)}/mongodb/collections/${encodeURIComponent(
			collectionName,
		)}/documents/${documentId}/get`,
		{},
	);
	return res.data;
}

export async function deleteDocument(
	projectId: string,
	collectionName: string,
	documentId: string,
): Promise<IMongoDBDocumentDeleted> {
	const res = await getClient().post(
		`/projects/services/${encodeURIComponent(projectId)}/mongodb/collections/${encodeURIComponent(
			collectionName,
		)}/documents/${documentId}/delete`,
		{},
	);
	return res.data;
}

export async function updateDocument(
	projectId: string,
	collectionName: string,
	documentId: string,
	update: object,
): Promise<IMongoDBDocumentUpdated> {
	const res = await getClient().post(
		`/projects/services/${encodeURIComponent(projectId)}/mongodb/collections/${encodeURIComponent(
			collectionName,
		)}/documents/${documentId}/update`,
		{ update },
	);
	return res.data;
}

export async function setDocument(
	projectId: string,
	collectionName: string,
	documentId: string,
	set: object,
	options?: object,
): Promise<IMongoDBDocumentUpdated> {
	const res = await getClient().post(
		`/projects/services/${encodeURIComponent(projectId)}/mongodb/collections/${encodeURIComponent(
			collectionName,
		)}/documents/${documentId}/set`,
		{ set, options },
	);
	return res.data;
}
