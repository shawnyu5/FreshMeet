import { PickerValue } from "@rnwonder/solid-date-picker";
import { createStore } from "solid-js/store";
import { MeetupEvents } from "./routes/meetups/types";

let datetime = new Date();
let month = datetime.getMonth() + 1;

export type State = {
  /**
   * @deprecated - get this from the query param, instead of the app state
   */
  dateRange: PickerValue;
  /**
   * @deprecated - get this from the query param, instead of the app state
   */
  query: string | null;
  events: MeetupEvents | null;
};

const [appState, setAppState] = createStore<State>({
  /**
   * Date range for events to occur in
  * @deprecated - this should be stored in the query param
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
  /**
   * Search query of the search bar
   */
  query: null,
  /**
   * Meetup events currently displayed
   */
  events: null,
});

/**
 * @deprecated - all information in state should come from the query param
 *
 */
export const useAppState = (): [typeof appState, typeof setAppState] => [
  appState,
  setAppState,
];
