const addon = require("./index.node");
// const fs = require('fs');

const data1 = {
  challenge: JSON.stringify({
    date: new Date().toISOString(),
    sender: "4975cec6-ce38-44fb-af04-7e93038e45d8",
    receiver: "dd023442-3c49-4f43-ab10-68253e751933",
    amount: 5325.33,
    hash: "004e9f210e46e07a93de74580bf3149eac8fb58d6980491b9341f4bb49efb651e553c501f9097af9551f3ff94bd53d89cdd76821c0002c6bd5346b4f1475d7533c7531b14b1db3ecf8dc2820fdc2b917d556b2647e862c3bb70c6adc4d1032acfb438100e2e55af4ff483dd2d9eb143d76f3c40ebcd37f5e5228b3d3abc69dd21b004e1c2ca388ca3e2406ef10331ce5727fb604ba9c43660d67b175f03b64516e13b677b2dac83349afeff42915e7260641efdddb899cc6ac9d01c6f968c4180fe86837a279323422374585030e801678d13563ed396517b8a2b26cb7ac8034aeb1884f8522e591a37d960170548e0b51d8e289bd2280b9cd2a6d1ce3e539d6e8cd0076116adab5f6aef52970bddcd636749f13b26db05d8ba28d8ab80888d6f3a25b8088eed6b5af173524b18b6e5e80ed80d32931ec04fd0e23e4c3070ede43ee2e07831805dcb6e2ee6b72f125a07977d57bfff411cbea009dc7b086057fc57095714668b3f75ab2f83decf95ef3526ade070b896d0f3d4a51e6a5f2628893c2d0ffadc8ddb7754767c690c9a5bf3287ca2868d892e3982f88c3be169c10b5db22fd7955209cbc9cfb9ac2ef87842206d3f9a6dd13dd9dfedb50308158350dbf5c870dda4097f743d1fbdff81ec476c1fa6c5e6ec978b296a8ddb656014522b39def7730204abebc2cf9db1d45f7f2a766eb6dd9cb54bb7f18efd98982fa60ff92e1"
  }),
  type_of_proof: "pietrzak", // pietrzak, wesolowski
  length: 2048,
  difficulty: 200,
};

const data2 = {
  ...data1,
  alleged_solution: '',
};

const tmp = {
  date: new Date().toISOString(),
  sender: "4975cec6-ce38-44fb-af04-7e93038e45d8",
  receiver: "dd023442-3c49-4f43-ab10-68253e751933",
  amount: 5325.33,
  hash: "004e9f210e46e07a93de74580bf3149eac8fb58d6980491b9341f4bb49efb651e553c501f9097af9551f3ff94bd53d89cdd76821c0002c6bd5346b4f1475d7533c7531b14b1db3ecf8dc2820fdc2b917d556b2647e862c3bb70c6adc4d1032acfb438100e2e55af4ff483dd2d9eb143d76f3c40ebcd37f5e5228b3d3abc69dd21b004e1c2ca388ca3e2406ef10331ce5727fb604ba9c43660d67b175f03b64516e13b677b2dac83349afeff42915e7260641efdddb899cc6ac9d01c6f968c4180fe86837a279323422374585030e801678d13563ed396517b8a2b26cb7ac8034aeb1884f8522e591a37d960170548e0b51d8e289bd2280b9cd2a6d1ce3e539d6e8cd0076116adab5f6aef52970bddcd636749f13b26db05d8ba28d8ab80888d6f3a25b8088eed6b5af173524b18b6e5e80ed80d32931ec04fd0e23e4c3070ede43ee2e07831805dcb6e2ee6b72f125a07977d57bfff411cbea009dc7b086057fc57095714668b3f75ab2f83decf95ef3526ade070b896d0f3d4a51e6a5f2628893c2d0ffadc8ddb7754767c690c9a5bf3287ca2868d892e3982f88c3be169c10b5db22fd7955209cbc9cfb9ac2ef87842206d3f9a6dd13dd9dfedb50308158350dbf5c870dda4097f743d1fbdff81ec476c1fa6c5e6ec978b296a8ddb656014522b39def7730204abebc2cf9db1d45f7f2a766eb6dd9cb54bb7f18efd98982fa60ff92e1"
};

let infiniti = [];
for (let i = 0; i < 1024; i++) {
  infiniti.push(tmp);
}

const data3 = {
  challenge: JSON.stringify({
    data: infiniti,
    nonce: null,
    hash: 'Consectetur laborum duis mollit adipisicing aute ipsum veniam ipsum proident ut adipisicing ex proident.',
    timestamp: new Date().toISOString(),
  }),
  difficulty: 3,
  stress: 80,
}

const data4 = {
  challenge: data3.challenge,
};

try {
  const createProofOfHistory = addon.createProofOfHistory(data1);
  console.log(createProofOfHistory);
  data2.alleged_solution = createProofOfHistory.solution;
  const verifyProofOfHistory = addon.verifyProofOfHistory(data2)
  console.log(verifyProofOfHistory);
  const createBlockProofOfWork = addon.createBlockProofOfWork(data3.challenge, data3.difficulty, data3.stress);
  console.log(createBlockProofOfWork);
  data4.nonce = createBlockProofOfWork.nonce;
  data4.hash = createBlockProofOfWork.hash;
  const verifyBlockProofOfWork = addon.verifyBlockProofOfWork(data4.challenge, data4.nonce, data4.hash)
  console.log(verifyBlockProofOfWork);

  // console.log('done', data3);
  // const rutaArchivo = './archivo.txt';
  // fs.writeFileSync(rutaArchivo, data3.challenge);

} catch (error) {
  console.error(error.name);
  console.error(error.message);
  console.error(error.stack);
}
