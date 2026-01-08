"use client";

import { useContext } from "react";
import { DislikeCats, LikeCats } from "./page";

export function Summary() {
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
