BASE_TYPES_DIR=../src/blockchain/fuel/types

RED_COLOR="\033[0;31m"
YELLOW_COLOR="\033[0;33m"
CLEAR_COLOR='\033[0m'

CONTRACT_FOLDERS=(
  account-balance
  clearing-house
  insurance-fund
  perp-market
  proxy
  vault
)

echo "${RED_COLOR}#### WARNING ####${CLEAR_COLOR}"
echo "${YELLOW_COLOR}#### src-20 and orderbook contracts from another repository required! ####${CLEAR_COLOR}"

sleep 5

for folder in "${CONTRACT_FOLDERS[@]}"
do
  npx fuels typegen -i "../../${folder}/out/debug/*-abi.json" -o "$BASE_TYPES_DIR/$folder/"
done