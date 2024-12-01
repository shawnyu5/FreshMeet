import "./NavBar.css";
import "@rnwonder/solid-date-picker/dist/style.css";
import { createEffect, createSignal } from "solid-js";
import { DatePickerComponent as DatePicker } from "./DatePicker";
import { useSearchParams } from "@solidjs/router";
import log from "~/logger";
import { dateToMeetupDate, NormalizedDate } from "~/utils";

export default function () {
  const [_searchParams, setSearchParams] = useSearchParams();
  // The initial value of the date picker
  const [initialPickerValue, _setInitialPickerValue] = createSignal(new NormalizedDate());
  // The selected dates of the date picker. The first date is the beginning, second is the end
  const [datePickerValue, setDatePickerValue] = createSignal<[NormalizedDate, NormalizedDate]>([
    new NormalizedDate(),
    new NormalizedDate(),
  ]);

  // On every date picker selection, update the query param with the new selected value
  createEffect(() => {
    const [startDate, endDate] = datePickerValue();
    setSearchParams({
      startDate: dateToMeetupDate(startDate, false),
      endDate: dateToMeetupDate(endDate, true),
    });
  });

  return (
    <div class="top-bar">
      <style></style>
      <div class="top-bar-left">
        <ul class="dropdown menu" data-dropdown-menu>
          <img src="../icon.png" width="50" />
          <li class="menu-text">Fresh meat</li>
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
          // TODO: fix this. this component needs to accept the correct type as props
          // <SearchBar dateRange={datePickerValue()} />
        }
      </div>
    </div>
  );
}
