import type { IMongoID } from './id';

export interface IProject {
	_id: IMongoID;
	name: string;
	users: string[];
}
