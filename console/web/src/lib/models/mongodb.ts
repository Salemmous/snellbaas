export interface IMongoDBCollection {
	name: string;
	type: string;
	options: any;
	info: {
		readOnly: boolean;
		uuid: {
			$binary: {
				base64: string;
				subType: string;
			};
		};
	};
	idIndex: {
		v: number;
		key: { _id: number };
		name: string;
	};
}

export interface IMongoDBDocumentCreated {
	_id: string;
}

export interface IMongoDBDocumentDeleted {
	deletedCount: number;
}

export interface IMongoDBDocumentUpdated {
	matchedCount: number;
	modifiedCount: number;
	upsertedId: any;
}
