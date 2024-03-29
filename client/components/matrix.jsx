"use client";
import React, { useEffect, useState } from "react";

const Matrix = ({ data }) => {
  const [apiResponse, setApiResponse] = useState([]);
  const [examples, setExamples] = useState([]);
  useEffect(() => {
    const slugs = data.map((d) => d.slug);
    fetch("http://localhost:8080/churns", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ slugs }),
    })
      .then((res) => res.json())
      .then((d) => {
        setApiResponse(d);
      })
      .catch((err) => {
        console.error(err);
      });
  }, [data]);
  const extendedData = [...data, { id: "none", name: "None", slug: "None" }];

  const findNumber = (fromSdk, toSdk) => {
    const match = apiResponse.find(
      (entry) => entry.from_sdk === fromSdk && entry.to_sdk === toSdk
    );
    return match ? match.number : 0;
  };

  const getBackgroundColor = (number) => {
    const colorRanges = [
      { min: 0, max: 10, color: "bg-red-100" },
      { min: 11, max: 50, color: "bg-red-200" },
      { min: 51, max: 100, color: "bg-red-300" },
      { min: 101, max: 400, color: "bg-red-400" },
      { min: 401, max: 600, color: "bg-red-500" },
      { min: 601, max: 700, color: "bg-red-600" },
      { min: 701, max: 800, color: "bg-red-700" },
      { min: 801, max: 900, color: "bg-red-800" },
      { min: 1000, max: 10000, color: "bg-red-900" },
    ];
    const matchingRange = colorRanges.find(
      (range) => number >= range.min && number <= range.max
    );
    return matchingRange ? matchingRange.color : "bg-red-500";
  };

  const handleClick = (from_sdk, to_sdk) => {
    const slugs = data.map((d) => d.slug);
    setExamples([]);
    fetch("http://localhost:8080/examples", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        from_sdk_slug: from_sdk.slug,
        to_sdk_slug: to_sdk.slug,
        slugs,
      }),
    })
      .then((res) => res.json())
      .then((d) => {
        let ExamplesSet = new Set(
          d["examples"].map((item) => JSON.stringify(item))
        );
        let Examples = Array.from(ExamplesSet).map((item) => JSON.parse(item));
        setExamples(Examples);
      })
      .catch((err) => {
        console.error(err);
      });
  };

  const get_text_color = (number) => {
    if (number > 801) {
      return "text-white";
    } else {
      return "text-gray-900";
    }
  };

  return (
    <div className="flex">
      <div>
        <table className="border-collapse">
          <thead>
            <tr>
              <th className="w-16 h-16"></th>
              {extendedData.map((col) => (
                <th key={col.id} className="w-16 h-16">
                  {col.name}
                </th>
              ))}
            </tr>
          </thead>

          <tbody>
            {extendedData.map((fromSdk) => (
              <tr key={fromSdk.id} className="h-16">
                <td className="w-16">{fromSdk.name}</td>
                {extendedData.map((toSdk) => (
                  <td
                    key={toSdk.id}
                    className={`w-16 h-16 ${getBackgroundColor(
                      findNumber(fromSdk.slug, toSdk.slug)
                    )}`}
                    onClick={() => handleClick(fromSdk, toSdk)}
                  >
                    <div
                      className={`flex justify-center items-center ${get_text_color(
                        findNumber(fromSdk.slug, toSdk.slug)
                      )}`}
                    >
                      {findNumber(fromSdk.slug, toSdk.slug)}
                    </div>
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <div className="ml-14 mt-10">Examples -</div>
      <div className="ml-6 mt-10">
        {examples.map((example) => (
          <div key={example.id}>
            <div className="flex gap-2">
              <div className="mt-1">
                <div>{example.name}</div>
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default Matrix;
