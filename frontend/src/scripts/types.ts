export type Attachment = {
  id: string;
  name: string;
  creationTime: Date;
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
  creationTime: Date;
};

export type Ingredient = {
  id: string;
  amount: string;
  unit: string;
  text: string;
  recipeReference: string | null | undefined;
  recipeId: string;
  creationTime: Date;
};

export type RecipeReferences = {
  error: string | null;
  references: Array<{ label: string; value: string }>;
};
