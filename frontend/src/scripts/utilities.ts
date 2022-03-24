import type { Recipe } from "./types";

/**
 * Compares two objects for shallow equality.
 */
export function equality_shallow_object(
  a: Record<string, unknown>,
  b: Record<string, unknown>
): boolean {
  const keys = Object.keys(a);
  if (keys.length !== Object.keys(b).length) {
    return false;
  }
  return keys.some((key) => a[key] !== b[key]) ? false : true;
}

/**
 * Checks if the specified file name corresponds to 
 * an image file.
 */
export function isImage(fileName: string): boolean {
  const lowercaseFileName = fileName.toLocaleLowerCase();
  return (
    lowercaseFileName.endsWith(".png") ||
    lowercaseFileName.endsWith(".jpg") ||
    lowercaseFileName.endsWith(".jpeg") ||
    lowercaseFileName.endsWith(".svg")
  );
}

/**
 * Returns the main image URL for a recipe.
 */
export function getRecipeImageUrl(recipe: Recipe): string {
  const imageAttachment = recipe.attachments.find((attachment) =>
    isImage(attachment.name)
  );
  return imageAttachment
    ? "/api/attachment/" + imageAttachment.id
    : "/icon_recipe.svg";
}
