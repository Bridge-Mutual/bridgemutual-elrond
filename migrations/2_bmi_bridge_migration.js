const { logTransaction } = require("./helpers/logger.js");
const Proxy = artifacts.require("TransparentUpgradeableProxy");
const BMIBridge = artifacts.require("BMIBridge");

const proxyAdmin = "0x56fEB55FFD9365D42D0a5321a3a029C4640Bd8DC";

module.exports = async (deployer, network, accounts) => {
  await deployer.deploy(BMIBridge);
  const bmiBridge = await BMIBridge.deployed();

  await deployer.deploy(Proxy, bmiBridge.address, proxyAdmin, []);
  const proxy = await Proxy.deployed();

  logTransaction(
    await (await BMIBridge.at(proxy.address)).__BMIBridge_init(),
    "Init BMIBridge"
  );
};
