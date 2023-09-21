import { once } from "events";
import { createReadStream } from "fs";
import { createInterface } from "readline";

(async function processLineByLine() {
    const fileReadStream = createInterface({
      input: createReadStream("..\\input.txt"),
      crlfDelay: Infinity
    });

    let highestAmountOfCaloriesCarriedByElf = 0;
    let amountOfCaloriesCarriedByCurrentElf = 0;

    fileReadStream.on("line", (currentLine) => {
      if (currentLine) {
        amountOfCaloriesCarriedByCurrentElf += parseInt(currentLine);
      } else {
        highestAmountOfCaloriesCarriedByElf =
        amountOfCaloriesCarriedByCurrentElf > highestAmountOfCaloriesCarriedByElf
        ? amountOfCaloriesCarriedByCurrentElf
        : highestAmountOfCaloriesCarriedByElf;

        amountOfCaloriesCarriedByCurrentElf = 0;
      }
    });

    await once(fileReadStream, "close");

    console.log(highestAmountOfCaloriesCarriedByElf);
})();