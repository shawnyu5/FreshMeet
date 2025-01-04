import DatePicker, { PickerValue } from "@rnwonder/solid-date-picker";
import { Accessor, createEffect, createSignal, Setter } from "solid-js";
import log from "~/logger";
import { NormalizedDate } from "~/utils";

/**
 * A date picker component
 * @param value - a signal getter representing the initial value of the date picker
 * @param setValue - a signal setter that will contain the beginning and end date of the picker
 */
export function DatePickerComponent(props: {
   value: Accessor<Date>;
   setValue: Setter<[NormalizedDate, NormalizedDate]>;
}) {
   const year = props.value().getFullYear();
   const month = props.value().getMonth();
   const day = props.value().getDate();

   // The picker will start by having the current date selected
   const [pickerValue, setPickerValue] = createSignal<PickerValue>({
      label: `${year}/${month + 1}/${day}`,
      value: {
         startDateObject: {
            year,
            month,
            day
         },
         endDateObject: {
            year,
            month,
            day
         },
      }
   });

   // whenever picker value is updated. Convert the picker value into a string and set it in the setValue prop
   createEffect(() => {
      log.info(
         `User selected picker value: ${JSON.stringify(pickerValue())}`,
      );
      props.setValue(pickerValueToDate(pickerValue()));
   });

   return (
      <DatePicker type="range" value={pickerValue} setValue={setPickerValue} />
   );
}

/**
 * Converts a pickerValue to 2 Date objects
 *
 * @param pickerValue - a picker value with `startDateObject` and `endDateObject` populated
 * @returns 2 Date objects, of the start and end date of the PickerValue
 */
export function pickerValueToDate(
   pickerValue: PickerValue,
): [NormalizedDate, NormalizedDate] {
   const date = new Date()
   // DatePicker relies on date months being 0 indexed
   const startDateObject = pickerValue.value.startDateObject;
   const endDateObject = pickerValue.value.endDateObject;

   const startDate = new NormalizedDate(startDateObject?.year ?? date.getFullYear(), ((startDateObject?.month) ?? date.getMonth()), startDateObject?.day ?? date.getDate())
   const endDate = new NormalizedDate(endDateObject?.year ?? date.getFullYear(), (endDateObject?.month ?? date.getMonth()), endDateObject?.day ?? date.getDate())
   log.info(`Converting picker value to date: ${startDate} - ${endDate}`)
   return [startDate, endDate];
}
