"use client";
import { useEffect, useState } from "react";
import Matrix from "@/components/matrix";
import NormMatrix from "@/components/normalizedMatrix";

export default function Home() {
  const [checkedSdks, setCheckedSdks] = useState([]);
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

  const handleCheckboxChange = (id, name, slug, isChecked) => {
    if (isChecked) {
      setCheckedSdks((prevCheckedSdks) => [
        ...prevCheckedSdks,
        { id, name, slug },
      ]);
    } else {
      setCheckedSdks((prevCheckedSdks) =>
        prevCheckedSdks.filter((item) => item.id !== id)
      );
    }
  };

  return (
    <div>
      <div className="flex gap-10 flex-wrap ml-6 mr-3">
        {sdks.map((d) => (
          <div key={d.id}>
            <div className="flex gap-2">
              <div className="mt-1">
                <Checkbox
                  id={d.id}
                  label={d.name}
                  slug={d.slug}
                  onCheckboxChange={handleCheckboxChange}
                />
              </div>
            </div>
          </div>
        ))}
      </div>
      <Matrix data={checkedSdks} />
      <NormMatrix data={checkedSdks} />
    </div>
  );
}

const Checkbox = ({ id, label, slug, onCheckboxChange }) => {
  const [isChecked, setIsChecked] = useState(false);

  const handleCheckboxChange = () => {
    setIsChecked(!isChecked);
    onCheckboxChange(id, label, slug, !isChecked);
  };

  return (
    <label className="flex gap-2">
      <input
        type="checkbox"
        checked={isChecked}
        onChange={handleCheckboxChange}
      />
      <div> {label}</div>
    </label>
  );
};
