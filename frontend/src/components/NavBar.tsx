import "./NavBar.css";
import "@rnwonder/solid-date-picker/dist/style.css";
import { PickerValue } from "@rnwonder/solid-date-picker";
import { createSignal } from "solid-js";
import { DatePickerComponent } from "./DatePicker";

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
            <DatePickerComponent value={dateRange} setValue={setDateRange} />
          </li>
        </ul>
      </div>

      <div class="top-bar-right">
        <form
          onSubmit={(e) => {
            e.preventDefault();
          }}
        >
          <ul class="menu">
            <li>
              <input type="search" placeholder="Search"></input>
            </li>
            <li>
              <button type="submit" class="button">
                Search
              </button>
            </li>
          </ul>
        </form>
      </div>
    </div>
  );
}
