"use client";

import { createContext, useEffect, useState } from "react";
import { fetchCatImage } from "./backend";

import CatView from "./cat_view";
import { Summary } from "./summary";

export const CatsSeenContext = createContext(0);
export const LikeCats = createContext([]);
export const DislikeCats = createContext([]);

export default function HomePage() {
  const [image, setImage] = useState(null);

  const catsSeenState = useState(0);
  const [catsSeen, _] = catsSeenState;

  const likeCatsState = useState([]);
  const dislikeCatsState = useState([]);

  const catLimit = 10;

  useEffect(() => {
    if (catsSeen < catLimit) {
      fetchCatImage().then((imageUrl) => {
        setImage(imageUrl);
      });
    }
  }, [catsSeen]);

  if (catsSeen >= catLimit) {
    return (
      <DislikeCats.Provider value={dislikeCatsState}>
        <LikeCats.Provider value={likeCatsState}>
          <h1 className="flex items-center justify-center mb-5">
            Paws and Preferences
          </h1>
          <Summary />
        </LikeCats.Provider>
      </DislikeCats.Provider>
    );
  }

  return (
    <DislikeCats.Provider value={dislikeCatsState}>
      <h1 className="flex items-center justify-center mb-5 text-5xl mt-5">
        Paws and Preferences
      </h1>
      <LikeCats.Provider value={likeCatsState}>
        <CatsSeenContext.Provider value={catsSeenState}>
          <div className="flex items-center justify-center mb-5">
            <CatView image={image} />
          </div>
        </CatsSeenContext.Provider>
      </LikeCats.Provider>
    </DislikeCats.Provider>
  );
}
