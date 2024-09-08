import { PickerValue } from "@rnwonder/solid-date-picker";
import { createStore } from "solid-js/store";
import { MeetupEvents } from "./routes/meetups/types";
import { Resource } from "solid-js";

let datetime = new Date();
let month = datetime.getMonth() + 1;

export type State = {
  dateRange: PickerValue;
  query: string | null;
  events: MeetupEvents | null;
};

const [appState, setAppState] = createStore<State>({
  /**
   * Date range for events to occur in
   */
  dateRange: {
    value: {
      startDateObject: {
        day: datetime.getDate(),
        month: month,
        year: datetime.getFullYear(),
      },
      endDateObject: {
        day: datetime.getDate(),
        month: month,
        year: datetime.getFullYear(),
      },
    },
    label: `${datetime.getFullYear()}-${month}-${datetime.getDate()}`,
  },
  query: null,
  events: null,
});

/**
 * Updates the date range for meetup events to occur in global state.
 *
 * At the same time corrects the month by adding one to it. Since node JS `Date()` object uses 0 indexed months
 *
 * @param val - the date range to set
 *
 * @deprecated use the store setter to set the date range directly
 */
export function setDateRange(val: PickerValue) {
  setAppState("dateRange", val);
}

export const useAppState = (): [typeof appState, typeof setAppState] => [
  appState,
  setAppState,
];
