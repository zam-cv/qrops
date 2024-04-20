import {
  ComposableMap,
  Geographies,
  Geography,
  Marker,
  ZoomableGroup,
} from "react-simple-maps";
import { Payment } from "./UsersTable";

export default function Map({ userInfo }: { userInfo: Payment | null }) {
  if (!userInfo) return <div></div>;

  return (
    <ComposableMap className="w-full h-full">
      <ZoomableGroup center={[0, 0]} zoom={1}>
        <Geographies geography="/features.json">
          {({ geographies }) =>
            geographies.map((geo) => (
              <Geography key={geo.rsmKey} fill="#214770" geography={geo} />
            ))
          }
        </Geographies>
        <Marker coordinates={[userInfo.longitude, userInfo.latitude]}>
          <circle r={3} fill="#F00" />
        </Marker>
      </ZoomableGroup>
    </ComposableMap>
  );
}
