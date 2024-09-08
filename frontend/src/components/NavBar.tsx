import "./NavBar.css";
import "@rnwonder/solid-date-picker/dist/style.css";
import { createEffect, createSignal } from "solid-js";
import { DatePickerComponent as DatePicker } from "./DatePicker";
import { SearchBar } from "./SearchBar";
import { useAppState } from "~/state";
import { useSearchParams } from "@solidjs/router";

export default function () {
  const [appState, setAppState] = useAppState();
  const [searchParams, setSearchParams] = useSearchParams();
  const [datePickerValue, setDatePickerValue] = createSignal(
    appState.dateRange,
  );

  createEffect(() => {
    setAppState("dateRange", datePickerValue());
    const startDate = `${datePickerValue().value.startDateObject?.year}-${datePickerValue().value.startDateObject?.month ?? 0 + 1}-${datePickerValue().value.startDateObject?.day}[US/Eastern]`;
    const endDate = `${datePickerValue().value.endDateObject?.year}-${datePickerValue().value.endDateObject?.month ?? 0 + 1}-${datePickerValue().value.endDateObject?.day}[US/Eastern]`;
    setSearchParams({ startDate: startDate, endDate: endDate });
  });

  return (
    <div class="top-bar">
      <style></style>
      <div class="top-bar-left">
        <ul class="dropdown menu" data-dropdown-menu>
          <img src="../icon.png" width="50" />
          <li class="menu-text">Fresh meat</li>
          {
            // <li>
            //   <a href="/">Today</a>
            // </li>
            // <li>
            //   <DatePicker value={datePickerValue} setValue={setDatePickerValue} />
            // </li>
          }
        </ul>
      </div>

      <div class="top-bar-right">
        <SearchBar dateRange={datePickerValue()} />
      </div>
    </div>
  );
}
