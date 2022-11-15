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
  creationTime: string;
};

export type Ingredient = {
  id: string;
  amount: string;
  unit: string;
  text: string;
  recipeReference: string | null | undefined;
  recipeId: string;
  creationTime: string;
};

export type RecipeReferences = {
  error: string | null;
  references: Array<{ label: string; value: string }>;
};

export type Payment = {
  id: string;
  target: string;
  note: string;
  paid: Record<string, string>;
  involved: Record<string, string>;
  paymentType: PaymentType;
  tags: Array<string>;
  attachments: Array<Attachment>;
  creationTime: string;
};

export type PaymentType = {
  OneOff: PaymentTypeValue | undefined;
  Daily: PaymentTypeValue | undefined;
  Weekly: PaymentTypeValue | undefined;
  Monthly: PaymentTypeValue | undefined;
  Annualy: PaymentTypeValue | undefined;
};

export type PaymentTypeValue = {
  distance: number | undefined;
  start: string;
  end: string | undefined;
};
