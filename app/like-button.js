"use client";

import { useContext } from "react";
import { LikeCats, CatsSeenContext } from "./page";

export default function LikeDislike({ image }) {
  const [catsSeen, setCatsSeen] = useContext(CatsSeenContext);
  const [likeCats, setLikeCats] = useContext(LikeCats);
  const [dislikeCats, setDislikeCats] = useContext(LikeCats);

  function handleClick(like) {
    if (like) {
      setLikeCats([...likeCats, image]);
    } else {
      setDislikeCats([...dislikeCats, image]);
    }

    setCatsSeen(catsSeen + 1);
  }

  return (
    <div className="like-dislike">
      <button onClick={() => handleClick(false)}>Dislike</button>
      <button onClick={() => handleClick(true)}>Like ({catsSeen})</button>
    </div>
  );
}
