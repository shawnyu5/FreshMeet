import { Navigate, useLocation } from "@solidjs/router";

export default function Home() {
   const location = useLocation()
   const params = new URLSearchParams(location.search)
   return <Navigate href={`/meetups?${params}`}></Navigate>
}
