"use client"
import { useEffect, useState } from "react";

export default function Home() {
  const [sdks, setSdks] = useState([]);
  useEffect(() => {
    fetch("http://localhost:8080/sdks")
      .then((res) => res.json())
      .then((d) => {
        setSdks(d);
      })
      .catch((err) => {
        console.error(err);
      });
  }, []);
  return (
    <div>
      {sdks.map((d) => (
        <div key={d.id}>
          {d.id}
          <div>
            {d.name}
            {d.slug}
          </div>
        </div>
      ))}
    </div>
  );
}
