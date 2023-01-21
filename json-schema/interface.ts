interface InvoiceData {
	items: Item[];
	buyer: Buyer;
	entreprise: Entreprise;
	title: string;
	date: string;
	duration: number;
	logoURL: string;
	duePercentage: number;
	structuredCommunication: string;
}

interface Entity {
	adress: Adress;
	name: string;
	vatNumber: string;
}

interface Adress {
	num: string;
	numSuffix?: string;
	city: string;
	street: string;
	postCode: string;
}

type Buyer = Entity;

interface Entreprise extends Entity {
	phone: string;
	email: string;
}

interface Item {
	vat: '6.0' | '12.0' | '21.0';
	priceHT: number;
	description: string;
	qt: number;
	intra: boolean;
}
