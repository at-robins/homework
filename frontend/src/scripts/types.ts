export type Attachment = {
  id: string;
  name: string;
  creation_date: Date;
};

export type Recipe = {
  id: string;
  title: string;
  instructions: string;
  reference: string;
  rating: number;
  tags: Array<string>;
  creation_date: Date;
};
