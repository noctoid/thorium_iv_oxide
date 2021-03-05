import * as tho2 from "thorium_oxide";

const shitButton = document.getElementById("shitButton");
const youSaid = document.getElementById("whatYouSaid");

const shitHappens = (s) => {
    tho2.say_something(s);
};

shitButton.addEventListener("click", event => {
    const times3 = tho2.important_3(youSaid.value);
    shitHappens(times3);
});
