"use client";

import { useContext } from "react";
import { LikeCats, CatsSeenContext, DislikeCats } from "../page";

export default function LikeDislike({ image }) {
  const [catsSeen, setCatsSeen] = useContext(CatsSeenContext);
  const [likeCats, setLikeCats] = useContext(LikeCats);
  const [dislikeCats, setDislikeCats] = useContext(DislikeCats);

  function handleClick(like) {
    if (like) {
      setLikeCats([...likeCats, image]);
    } else {
      setDislikeCats([...dislikeCats, image]);
    }

    setCatsSeen(catsSeen + 1);
  }

  return (
    <div className="flex items-center justify-center mt-5">
      <button
        className="mr-5 hover:bg-red-500 p-5 text-4xl border border-solid"
        onClick={() => handleClick(false)}
      >
        Dislike
      </button>
      <button
        className="ml-5 hover:bg-violet-300 p-5 text-4xl border border-solid"
        onClick={() => handleClick(true)}
      >
        Like
      </button>
    </div>
  );
}
