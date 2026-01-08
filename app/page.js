"use client";

import CatView from "./cat-view";
import LikeDislike from "./like-button";

import { fetchCatImage } from "./backend";
import { createContext, useEffect, useState } from "react";

export const CatsSeenContext = createContext(0);
export const LikeCats = createContext([]);
export const DislikeCats = createContext([]);

export default function HomePage() {
  const [image, setImage] = useState("");

  const catsSeenState = useState(0);
  const [catsSeen, _] = catsSeenState;

  const likeCatsState = useState([]);
  const dislikeCatsState = useState([]);

  useEffect(() => {
    fetchCatImage().then((imageUrl) => {
      setImage(imageUrl);
    });
  }, [catsSeen]);

  return (
    <DislikeCats.Provider value={dislikeCatsState}>
      <LikeCats.Provider value={likeCatsState}>
        <CatsSeenContext.Provider value={catsSeenState}>
          <div>
            <CatView image={image} />
            <LikeDislike image={image} />
          </div>
        </CatsSeenContext.Provider>
      </LikeCats.Provider>
    </DislikeCats.Provider>
  );
}
