import Chart, { getData } from ".";
import { useEffect, useState } from "react";
import { Pie } from "react-chartjs-2";
import { getColors } from "../../utils/chart";
import { CHART_DEFAULT_OPTIONS } from "../../utils/constants";
import api from "@/utils/api";

export default function AverageTimeInGame() {
  const [typesUsers, setTypesUsers] = useState<string[]>([]);
  const [users, setUsers] = useState<[string, number][]>([]);

  useEffect(() => {
    api.users.getUsersTypes().then((data) => {
      setTypesUsers(data);
    });

    api.users.getAverageTimeInGame().then((data) => {
      setUsers(data);
    });
  }, []);

  return (
    <Chart title="Tiempo promedio en el juego por tipo de usuario">
      <Pie
        data={{
          labels: typesUsers,
          datasets: [
            {
              data: getData(typesUsers, users),
              backgroundColor: getColors(typesUsers.length),
            },
          ],
        }}
        options={CHART_DEFAULT_OPTIONS}
      />
    </Chart>
  );
}
