import type { PaymentType, Recipe } from "./types";
import { DateTime } from "luxon";

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

export function getInitialPaymentDateAsUTCString(
  paymentType: PaymentType
): string {
  if (paymentType.OneOff) {
    return paymentType.OneOff.start;
  } else if (paymentType.Daily) {
    return paymentType.Daily.start;
  } else if (paymentType.Weekly) {
    return paymentType.Weekly.start;
  } else if (paymentType.Monthly) {
    return paymentType.Monthly.start;
  } else if (paymentType.Annualy) {
    return paymentType.Annualy.start;
  } else {
    throw new Error("Not a valid payment type: " + paymentType);
  }
}

export function getTotalPaymentAmount(record: Record<string, string>): number {
  return Object.values(record).reduce((previous: number, current: string) => {
    return previous + parseFloat(current);
  }, 0);
}

export function paymentTypeToLabel(paymentType: PaymentType):
  | {
      label: "Einmalzahlung";
      value: "OneOff";
    }
  | {
      label: "Täglich";
      value: "Daily";
    }
  | {
      label: "Wöchentlich";
      value: "Weekly";
    }
  | {
      label: "Monatlich";
      value: "Monthly";
    }
  | {
      label: "Jährlich";
      value: "Annualy";
    } {
  if (Object.keys(paymentType).includes("OneOff")) {
    return {
      label: "Einmalzahlung",
      value: "OneOff",
    };
  } else if (Object.keys(paymentType).includes("Daily")) {
    return {
      label: "Täglich",
      value: "Daily",
    };
  } else if (Object.keys(paymentType).includes("Weekly")) {
    return {
      label: "Wöchentlich",
      value: "Weekly",
    };
  } else if (Object.keys(paymentType).includes("Monthly")) {
    return {
      label: "Monatlich",
      value: "Monthly",
    };
  } else if (Object.keys(paymentType).includes("Annualy")) {
    return {
      label: "Jährlich",
      value: "Annualy",
    };
  } else {
    throw new Error(paymentType + " is not a valid payment type.");
  }
}

export function getPaymentTypeDistance(
  paymentType: PaymentType
): number | null {
  const val = Object.values(paymentType)[0];
  if (val) {
    return val.distance ? val.distance : null;
  } else {
    throw new Error("Invalid Paymentype: " + paymentType);
  }
}

export function getPaymentTypeEnd(paymentType: PaymentType): DateTime | null {
  const val = Object.values(paymentType)[0];
  if (val) {
    return val.end ? DateTime.fromISO(val.end) : null;
  } else {
    throw new Error("Invalid Paymentype: " + paymentType);
  }
}

export function getPaymentTypeStart(paymentType: PaymentType): DateTime {
  const val = Object.values(paymentType)[0];
  if (val) {
    return DateTime.fromISO(val.start);
  } else {
    throw new Error("Invalid Paymentype: " + paymentType);
  }
}
