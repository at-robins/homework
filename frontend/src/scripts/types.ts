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
  attachments: Array<Attachment>;
  ingredients: Array<Ingredient>;
  creation_date: Date;
};

export type Ingredient = {
  id: string;
  amount: string;
  unit: string;
  text: string;
  recipeReference: string | null | undefined;
};
