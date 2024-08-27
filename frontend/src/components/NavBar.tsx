import "./NavBar.css";
import "@rnwonder/solid-date-picker/dist/style.css";
import { PickerValue } from "@rnwonder/solid-date-picker";
import { createSignal } from "solid-js";
import { DatePickerComponent as DatePicker } from "./DatePicker";
import { SearchForum as SearchBar } from "./SearchBar";

export default function () {
  let datetime = new Date();
  const [dateRange, setDateRange] = createSignal<PickerValue>({
    value: {
      startDateObject: {
        day: datetime.getDate(),
        month: datetime.getMonth(),
        year: datetime.getFullYear(),
      },
      endDateObject: {
        day: datetime.getDate(),
        month: datetime.getMonth(),
        year: datetime.getFullYear(),
      },
    },
    label: `${datetime.getFullYear()}-${datetime.getMonth()}-${datetime.getDate()}`,
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
            <DatePicker value={dateRange} setValue={setDateRange} />
          </li>
        </ul>
      </div>

      <div class="top-bar-right">
        <SearchBar />
      </div>
    </div>
  );
}
