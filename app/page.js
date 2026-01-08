"use client";

import CatView from "./cat-view";
import LikeDislike from "./like-button";

import { fetchCatImage } from "./backend";
import { createContext, useContext, useEffect, useState } from "react";

export const CatsSeenContext = createContext(0);
export const LikeCats = createContext([]);
export const DislikeCats = createContext([]);

function Summary() {
  const [likeCats, _setLikeCats] = useContext(LikeCats);
  const [dislikeCats, _setDislikeCats] = useContext(DislikeCats);

  let likes = [];

  for (let i = 0; i < likeCats.length; i++) {
    console.log(likeCats[i]);
    likes.push(
      <img
        className="w-[500px] h-[500px] object-contain p-5"
        src={likeCats[i]}
      ></img>,
    );
  }

  let dislikes = [];
  for (let i = 0; i < dislikeCats.length; i++) {
    dislikes.push(
      <img
        className="w-[500px] h-[500px] object-contain p-5"
        src={dislikeCats[i]}
      ></img>,
    );
  }

  return (
    <div className="grid grid-cols-2">
      <div className="flex flex-col items-center text-center">
        <h1 className="text-5xl bold">Likes</h1>
        {likes}
      </div>
      <div className="flex flex-col items-center text-center">
        <h1 className="text-5xl bold">Dislikes</h1>
        {dislikes}
      </div>
    </div>
  );
}

export default function HomePage() {
  const [image, setImage] = useState(null);

  const catsSeenState = useState(0);
  const [catsSeen, _] = catsSeenState;

  const likeCatsState = useState([]);
  const dislikeCatsState = useState([]);

  const catLimit = 3;

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
          <Summary></Summary>
        </LikeCats.Provider>
      </DislikeCats.Provider>
    );
  }

  return (
    <DislikeCats.Provider value={dislikeCatsState}>
      <LikeCats.Provider value={likeCatsState}>
        <CatsSeenContext.Provider value={catsSeenState}>
          <div className="flex items-center justify-center mb-5">
            <CatView image={image} />
          </div>
          <div className="flex items-center justify-center mb-5">
            <LikeDislike image={image} />
          </div>
        </CatsSeenContext.Provider>
      </LikeCats.Provider>
    </DislikeCats.Provider>
  );
}
