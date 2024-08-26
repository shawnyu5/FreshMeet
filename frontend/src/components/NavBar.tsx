import "./NavBar.css";
import "@rnwonder/solid-date-picker/dist/style.css";
import DatePicker, { PickerValue } from "@rnwonder/solid-date-picker";
import {
   Accessor,
   createSignal,
   Setter,
} from "solid-js";

export default function() {
   let datetime = new Date();
   // console.log(`Datetime day: ${datetime.getUTCDay()}`);
   // // console.log(`Datetime utc: ${datetime.toUTCString()}`);
   // console.log(`Datetime month: ${datetime.getUTCMonth()}`);
   // console.log(`Datetime year: ${datetime.getUTCFullYear()}`);

   const [dateRange, setDateRange] = createSignal<PickerValue>({
      value: {
         startDateObject: {
            day: datetime.getDay(),
            month: datetime.getMonth(),
            year: datetime.getFullYear(),
         },
         endDateObject: {
            day: datetime.getDay(),
            month: datetime.getMonth(),
            year: datetime.getFullYear(),
         },
      },
      label: `${datetime.getFullYear()}-${datetime.getMonth()}-${datetime.getDay()}`,
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
                  {
                     // TODO: date picker goes here
                  }
                  <DatePickerComponent value={dateRange} setValue={setDateRange} />
               </li>
            </ul>
         </div>

         <div class="top-bar-right">
            <ul class="menu">
               <li>
                  <input type="search" placeholder="Search"></input>
               </li>
               <li>
                  <button type="button" class="button">
                     Search
                  </button>
               </li>
            </ul>
         </div>
      </div>
   );
}

/**
 * A date picker component
 */
function DatePickerComponent(props: {
   value: Accessor<PickerValue>;
   setValue: Setter<PickerValue>;
}) {
   return (
      <DatePicker
         // onChange={(data) => {
         //    if (data.type === "range") {
         //       console.log(data.startDate, data.endDate);
         //    }
         // }}
         type="range"
         value={props.value}
         setValue={props.setValue}
      />
   );
}
