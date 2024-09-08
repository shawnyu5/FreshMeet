import DatePicker, { PickerValue } from "@rnwonder/solid-date-picker";
import { Accessor, createSignal, Setter } from "solid-js";

/**
 * A date picker component
 * @param value - a signal getter representing the initial value of the date picket
 * @param setValue - a signal setter that will contain the value of the selected date
 */
export function DatePickerComponent(props: {
  value: Accessor<PickerValue>;
  setValue: Setter<PickerValue>;
}) {
  return (
    <DatePicker
      // onChange={(data) => {
      //   if (data.type === "range") {
      //     console.log(data.startDate, data.endDate);
      //   }
      //   if (data.type === "single") {
      //     console.log(data.selectedDate);
      //   }
      //   if (data.type === "multiple") {
      //     console.log(data.multipleDates);
      //   }
      // }}
      type="range"
      value={props.value}
      setValue={props.setValue}
    />
  );
}
