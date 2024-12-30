import "./NavBar.css";
import "@rnwonder/solid-date-picker/dist/style.css";
import { createEffect, createSignal } from "solid-js";
import { DatePickerComponent as DatePicker } from "./DatePicker";
import { useSearchParams } from "@solidjs/router";
import { dateToMeetupDate, NormalizedDate } from "~/utils";
import { SearchBar } from "./SearchBar";

export default function () {
  const [searchParams, setSearchParams] = useSearchParams();
  // The initial value of the date picker
  const [initialPickerValue, _setInitialPickerValue] = createSignal(
    // DatePicker relies on the Date object having 0 indexed months
    new Date(),
  );
  // The selected dates of the date picker. The first date is the beginning, second is the end
  const [datePickerValue, setDatePickerValue] = createSignal<
    [NormalizedDate, NormalizedDate]
  >([new NormalizedDate(), new NormalizedDate()]);

  // On every date picker selection, update the query param with the new selected value
  createEffect(() => {
    const [startDate, endDate] = datePickerValue();
    if (
      searchParams.startDate !== dateToMeetupDate(startDate, false) ||
      searchParams.endDate !== dateToMeetupDate(endDate, true)
    ) {
      setSearchParams({
        startDate: dateToMeetupDate(startDate, false),
        endDate: dateToMeetupDate(endDate, true),
      });
    }
  });

  return (
    <div class="top-bar">
      <style></style>
      <div class="top-bar-left">
        <ul class="dropdown menu" data-dropdown-menu>
          <img src="../icon.png" width="50" />
          <li class="menu-text">Fresh meet</li>
          <li>
            <a href="/">Today</a>
          </li>
          <li>
            <DatePicker
              value={initialPickerValue}
              setValue={setDatePickerValue}
            />
          </li>
        </ul>
      </div>

      <div class="top-bar-right">
        {
          <SearchBar />
        }
      </div>
    </div>
  );
}
