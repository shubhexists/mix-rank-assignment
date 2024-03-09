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
      <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
        Choose SDKs
      </h2>
      <div className="grid grid-cols-2 gap-4 p-6 rounded-lg border border-gray-200 items-start ml-10 mr-10">
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
      <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
        Competitive Matrix
      </h2>
      <Matrix data={checkedSdks} />
      <h2 className="mt-6 text-center text-3xl font-extrabold text-gray-900">
        Normalized (Row) Matrix
      </h2>
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
    <label className="flex items-center space-x-2">
      <input
        type="checkbox"
        checked={isChecked}
        onChange={handleCheckboxChange}
      />
      <div> {label}</div>
    </label>
  );
};
