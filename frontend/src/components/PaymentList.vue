<template>
  <div>
    <payment-filter
      :payments="payments"
      @updated-filter="filteredPayments = $event"
    />
    <div class="q-pa-md">
      <q-table
        title="Rechnungen"
        :rows="filteredPayments"
        :columns="columns"
        :row-key="getRowId"
        no-data-label="Keine Daten gefunden."
        :loading="isLoadingPayments"
        selection="multiple"
        v-model:selected="selectedRowsModel"
        :selected-rows-label="getSelectedRowsLabel"
        :pagination="{ sortBy: 'date', descending: true, rowsPerPage: 50 }"
        @row-click="navigateToPayment"
      >
        <template v-slot:top>
          <div class="q-table__title">Rechnungen</div>

          <q-space />

          <delete-button
            tooltip="Rechnungen löschen."
            :loading="isDeletingPayments"
            :error="deletePaymentsErrorMessage"
            @deletion-confirmed="removeSelectedPayments"
          />
        </template>
      </q-table>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Payment, PaymentType } from "@/scripts/types";
import {
  getInitialPaymentDateAsUTCString,
  getTotalPaymentAmount,
} from "@/scripts/utilities";
import axios from "axios";
import { onMounted, ref, type Ref } from "vue";
import PaymentFilter from "./payment/PaymentFilter.vue";
import DeleteButton from "./general/DeleteButton.vue";
import { useRouter } from "vue-router";
const payments: Ref<Array<Payment>> = ref([]);
const filteredPayments: Ref<Array<Payment>> = ref([]);
const isLoadingPayments = ref(false);
const loadPaymentsErrorMessage = ref("");
const isDeletingPayments = ref(false);
const deletePaymentsErrorMessage = ref("");
const currencyFormatter = new Intl.NumberFormat("de-DE", {
  style: "currency",
  currency: "EUR",
});
const router = useRouter();
const selectedRowsModel: Ref<Array<Payment>> = ref([]);

const columns: {
  name: string;
  label: string;
  align: "left" | "right" | "center" | undefined;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  field: string | ((row: any) => any);
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  format: ((val: any) => string) | undefined;
  sortable: boolean | undefined;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  sort: ((a: any, b: any) => number) | undefined;
}[] = [
  {
    name: "name",
    label: "Bezug",
    align: "left",
    field: (row: Payment) => row.target,
    format: undefined,
    sortable: true,
    sort: (a: string, b: string) => {
      return a > b ? 1 : -1;
    },
  },
  {
    name: "date",
    align: "left",
    label: "Rechnungszeitpunk",
    field: (row: Payment) => row.paymentType,
    format: (val: PaymentType) => {
      const date = new Date(getInitialPaymentDateAsUTCString(val));
      return date.toLocaleString();
    },
    sortable: true,
    sort: (a: PaymentType, b: PaymentType) => {
      return getInitialPaymentDateAsUTCString(a) >
        getInitialPaymentDateAsUTCString(b)
        ? 1
        : -1;
    },
  },
  {
    name: "amount",
    align: "right",
    label: "Gesamtbetrag",
    field: (row: Payment) => row.paid,
    format: (val: Record<string, string>) => {
      return currencyFormatter.format(getTotalPaymentAmount(val));
    },
    sortable: true,
    sort: (a: Record<string, string>, b: Record<string, string>) => {
      return getTotalPaymentAmount(a) - getTotalPaymentAmount(b);
    },
  },
  {
    name: "paid",
    align: "left",
    label: "Bezahler",
    field: (row: Payment) => row.paid,
    format: (val: Record<string, string>) => {
      return Object.keys(val).sort().join(", ");
    },
    sortable: true,
    sort: (a: Record<string, string>, b: Record<string, string>) => {
      return Object.keys(a).sort().join(", ") > Object.keys(b).sort().join(", ")
        ? 1
        : -1;
    },
  },
  {
    name: "involved",
    align: "left",
    label: "Beteiligte",
    field: (row: Payment) => row.involved,
    format: (val: Record<string, string>) => {
      return Object.keys(val).sort().join(", ");
    },
    sortable: true,
    sort: (a: Record<string, string>, b: Record<string, string>) => {
      return Object.keys(a).sort().join(", ") > Object.keys(b).sort().join(", ")
        ? 1
        : -1;
    },
  },
];

onMounted(() => {
  loadPayments();
});

function loadPayments() {
  isLoadingPayments.value = true;
  loadPaymentsErrorMessage.value = "";
  axios
    .get("/api/payments")
    .then((response) => {
      payments.value = response.data;
      filteredPayments.value = payments.value;
    })
    .catch((error) => {
      payments.value = [];
      loadPaymentsErrorMessage.value = error;
    })
    .finally(() => {
      isLoadingPayments.value = false;
    });
}

function removeSelectedPayments() {
  if (!isDeletingPayments.value && selectedRowsModel.value.length > 0) {
    isDeletingPayments.value = true;
    deletePaymentsErrorMessage.value = "";
    const currentlySelected = selectedRowsModel.value.map(
      (selected) => selected.id
    );
    const formData = JSON.stringify(currentlySelected);
    const config = {
      headers: {
        "content-type": "application/json",
      },
    };
    axios
      .patch("/api/payments", formData, config)
      .then(() => {
        payments.value = payments.value.filter(
          (payment) => !currentlySelected.includes(payment.id)
        );
        filteredPayments.value = filteredPayments.value.filter(
          (payment) => !currentlySelected.includes(payment.id)
        );
      })
      .catch((error) => {
        deletePaymentsErrorMessage.value = error;
      })
      .finally(() => {
        isDeletingPayments.value = false;
      });
  }
}

function getSelectedRowsLabel(numberOfSelectedRows: number): string {
  return numberOfSelectedRows + " Zahlungen zum Löschen ausgewählt.";
}

function getRowId(row: Payment): string {
  return row.id;
}

function navigateToPayment(_event: Event, payment: Payment) {
  router.push({ name: "payment", params: { id: payment.id } });
}
</script>
<style scoped lang="scss"></style>
