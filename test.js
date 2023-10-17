const addon = require("./index.node");
const data = {
  challenge: JSON.stringify({
    date: new Date().toISOString(),
    sender: "4975cec6-ce38-44fb-af04-7e93038e45d8",
    receiver: "dd023442-3c49-4f43-ab10-68253e751933",
    amount: 5325.33,
  }),
  // type_of_proof: "wesolowski",
};

try {
  console.log(addon.createProofOfHistory(data));
} catch (error) {
  console.error(error.name);
  console.error(error.message);
  console.error(error.stack);
}
