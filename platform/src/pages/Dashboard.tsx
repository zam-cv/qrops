import UsersByType from "@/components/charts/UsersByType";
import UsersByGender from "@/components/charts/UsersByGender";
import UsersByAgeRange from "@/components/charts/UsersByAgeRange";
import Details from "@/components/Details";

export default function Dashboard() {
  return (
    <div className="grid grid-cols-3 grid-rows-2 gap-5 h-full">
      <UsersByType />
      <UsersByAgeRange />
      <UsersByGender />
      <Details />
      <div className="bg-gray-300"></div>
      <div className="bg-gray-300"></div>
    </div>
  );
}
