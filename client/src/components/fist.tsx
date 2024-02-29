import * as React from "react";

function Counter({ title }: {title: string}) {
  let counter = 0;
  return(
    <button onClick={counter = counter+1}>{counter}</button>
  );
}



export default Counter;
