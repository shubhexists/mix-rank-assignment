import React, { useEffect, useState } from "react";

const NormMatrix = ({ data }) => {
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
      { min: 0, max: 10, color: "bg-blue-100" },
      { min: 11, max: 20, color: "bg-blue-200" },
      { min: 21, max: 30, color: "bg-blue-300" },
      { min: 31, max: 40, color: "bg-blue-400" },
      { min: 41, max: 60, color: "bg-blue-500" },
      { min: 61, max: 70, color: "bg-blue-600" },
      { min: 71, max: 80, color: "bg-blue-700" },
      { min: 81, max: 90, color: "bg-blue-800" },
      { min: 91, max: 100, color: "bg-blue-900" },
    ];
    const matchingRange = colorRanges.find(
      (range) => number >= range.min && number <= range.max
    );
    return matchingRange ? matchingRange.color : "bg-blue-500";
  };
  const calculateRowSum = (fromSdk) => {
    return extendedData.reduce((sum, toSdk) => {
      return sum + findNumber(fromSdk.slug, toSdk.slug);
    }, 0);
  };

  const normalizedData = extendedData.map((fromSdk) => {
    const rowSum = calculateRowSum(fromSdk);
    return extendedData.map((toSdk) => {
      const originalValue = findNumber(fromSdk.slug, toSdk.slug);
      const normalizedValue = rowSum !== 0 ? originalValue / rowSum : 0;
      const roundedValue = Math.round(normalizedValue * 100);
      return roundedValue;
    });
  });

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
    if (number > 81) {
      return "text-white";
    } else {
      return "text-gray-900";
    }
  }

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
            {extendedData.map((fromSdk, rowIndex) => (
              <tr key={fromSdk.id} className="h-16">
                <td className="w-16">{fromSdk.name}</td>
                {extendedData.map((toSdk, colIndex) => (
                  <td
                    key={toSdk.id}
                    className={`w-16 h-16 ${getBackgroundColor(
                      normalizedData[rowIndex][colIndex]
                    )}`}
                    onClick={() => handleClick(fromSdk, toSdk)}
                  >
                    <div className={`flex justify-center items-center ${get_text_color(
                      normalizedData[rowIndex][colIndex]
                    )}`}>
                      {normalizedData[rowIndex][colIndex] + "%"}
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

export default NormMatrix;
