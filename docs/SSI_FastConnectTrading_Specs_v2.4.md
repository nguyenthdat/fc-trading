# FastConnectTrading API Specs

**Version 2.4**

|Menu|
|---|
|**1**<br>**Description .............................................................................................................................................. 5**|
|**2**<br>**Integration process ................................................................................................................................ 5**|
|**3**<br>**Environment information: ...................................................................................................................... 5**|
|**4**<br>**Key Credential ........................................................................................................................................ 5**|
|**5**<br>**TradingAPI ............................................................................................................................................... 5**|
|<br>**5.1**<br>**Config request ................................................................................................................................ 5**|
|**5.2**<br>**GetOTP ............................................................................................................................................ 5**|
|5.2.1<br>GetOTP Request ...................................................................................................................... 5|
|<br>5.2.2<br>GetOTP Response ................................................................................................................... 6|
|<br>**5.3**<br>**VerifyCode ....................................................................................................................................... 6**|
|5.3.1<br>VerifyCode Request ................................................................................................................. 7|
|5.3.2<br>VerifyCode Response ............................................................................................................... 7|
|**5.4**<br>**AccessToken................................................................................................................................... 7**|
|5.4.1<br>AccessToken Request .............................................................................................................. 7|
|5.4.2<br>AccessToken Response ........................................................................................................... 8|
|**5.5**<br>**New Order ........................................................................................................................................ 9**|
|5.5.1<br>NewOrder Request ................................................................................................................... 9|
|5.5.2<br>NewOrder Response .............................................................................................................. 10|
|**5.6**<br>**Modify Order ................................................................................................................................. 12**|
|5.6.1<br>ModifyOrder Request ............................................................................................................. 12|
|<br>5.6.2<br>ModifyOrder Response ........................................................................................................... 13|
|<br>**5.7**<br>**Cancel Order ................................................................................................................................. 14**|
|5.7.1<br>CancelOrder Request ............................................................................................................. 14|
|5.7.2<br>CancelOrderResponse ........................................................................................................... 15|
|**5.8**<br>**DerNewOrder................................................................................................................................. 16**|
|5.8.1<br>DerNewOrder Request ........................................................................................................... 16|
|5.8.2<br>DerNewOrder Response ........................................................................................................ 17|
|**5.9**<br>**DerModifyOrder ............................................................................................................................ 18**|
|5.9.1<br>DerModifyOrder Request ........................................................................................................ 19|
|5.9.2<br>DerModifyOrder Response ..................................................................................................... 19|
|**5.10**<br>**DerCancelOrder ............................................................................................................................ 20**|
|5.10.1<br>CancelOrder Request ............................................................................................................. 21|
|5.10.2<br>DerCancelOrderResponse ..................................................................................................... 21|
|**5.11**<br>**Order History................................................................................................................................. 22**|
|5.11.1<br>OrderHistory Request ............................................................................................................. 22|
|<br>5.11.2<br>OrderHistory Response .......................................................................................................... 22|
|**5.12**<br>**Stock Position ............................................................................................................................... 23**|
|5.12.1<br>StockPosition Request ........................................................................................................... 24|
|5.12.2<br>StockPosition Response ......................................................................................................... 24|
|**5.13**<br>**Derivatives Position ..................................................................................................................... 26**|
|<br>5.13.1<br>DerPosition Request ............................................................................................................... 26|
|<br>5.13.2<br>DerPosition Response ............................................................................................................ 26|
|<br>**5.14**<br>**Max Buy Quantity ......................................................................................................................... 27**|
|5.14.1<br>MaxBuyQty Request ............................................................................................................... 28|
|5.14.2<br>MaxBuyQty Response ............................................................................................................ 28|
|**5.15**<br>**Max Sell Quantity .......................................................................................................................... 29**|
|5.15.1<br>MaxSellQty Request ............................................................................................................... 29|
|<br>5.15.2<br>MaxSellQty Response ............................................................................................................ 29|

|**5.16**|**Account Balance .......................................................................................................................... 30**|
|---|---|
|5.16|.1<br>AccountBalance Request ....................................................................................................... 30|
|5.16|.2<br>AccountBalance Response .................................................................................................... 30|
|**5.17**|**Purchasing power Margin of Account ........................................................................................ 31**|
|5.17|.1<br>ppmmrAccount Request ......................................................................................................... 32|
|5.17|.2<br>ppmmrAccount Response ...................................................................................................... 32|
|**5.18**|**Derivartives Account Balance ..................................................................................................... 34**|
|5.18|.1<br>DerAccountBalance Request ................................................................................................. 35|
|5.18|.2<br>DerAccountBalance Response ............................................................................................... 35|
|**5.19**|**AuditOrderBook ............................................................................................................................ 37**|
|5.19|.1<br>auditOrderBook Request ........................................................................................................ 38|
|5.19|.2<br>auditOrderBook Response ..................................................................................................... 38|
|**5.20**|**OrderBook ..................................................................................................................................... 40**|
|5.20|.1<br>OrderBook Request ................................................................................................................ 40|
|5.20|.2<br>OrderBook Response ............................................................................................................. 40|
|**6**<br>**TAP**|**I Streaming ..................................................................................................................................... 41**|
|**6.1**|**Order Streaming ........................................................................................................................... 41**|
|6.1.|1<br>Order Event Response ........................................................................................................... 42|
|6.1.|2<br>Order Error .............................................................................................................................. 46|
|6.1.|3<br>Order Match Event.................................................................................................................. 48|
|**6.2**|**Porfolio Streaming ....................................................................................................................... 48**|
|**7**<br>**App**|**endix ............................................................................................................................................... 49**|
|**7.1**|**Error Code ..................................................................................................................................... 49**|
|**7.2**|**Order Status .................................................................................................................................. 52**|
|**7.3**|**Channel .......................................................................................................................................... 52**|
|**7.4**|**Order Type..................................................................................................................................... 53**|
## Document History

|Date|Changed by|Description|Version|
|---|---|---|---|
|22/10/2020||Created|1.0|
|04/07/2022||Add streaming OrderMatchEvent|2.0|
|08/09/2022||Add API getOTP|2.1|
|09/05/2023||Update 4.7 Order Type|2.2|
|25/05/2023||Update API newOrder and API verifyCode, Modify Order, Cancel<br>Order|2.2|
|07/06/2023||Add API Orderbook, API derNewOrder, derModifyOrder,<br>derCancelOrder and Update OrderError|2.3|
|14/07/2023||Add API auditOrderBook|2.4|
|||Vaildate trường deviceID là bắt buộc và đúng format||

## 1 Description

FastConnect TradingAPI is an API service used to make an order to the trading system. It provides a set of APIs to achieve this goal.

FastConnectTrading API includes:

- FCTradingAPI: an API to interact with trading system.

- FCTAPI Order Streaming: a TCP stream to return order result to client

All will be hosted in SSI side.

## 2 Integration process
   - Register api service on iBoard

   - Get the key to intergration

## 3 Environment information:
|Service Name|Env Name|Host|Port|Requires VPN|
|---|---|---|---|---|
|FCTradingAPI|Prod|https://fc-<br>tradeapi.ssi.com.vn||No|
|FCTAPI Order|Prod|https://fc-||No|
|Streaming||tradehub.ssi.com.vn|||

## 4 Key Credential
Configuration info to integrate with FCTradingAPI

   - ConsumerID: Identify your account.

   - ConsumerSecrect: access key server

   - PrivateKey: Used create digital signatuer by RSA algorithm

## 5 TradingAPI
### 5.1 Config request
|Method|Content||
|---|---|---|
|Post|Header (Authorization, Content-<br>Type)|Body (json)|
|Get|Header (Authorization)|Params|

### 5.2 GetOTP
- Url:  https://fc-tradeapi.ssi.com.vn/api/v2/Trading/GetOTP

- Method: POST

- Uses: get OTP when User use 2FA: SMS/ Email/ SmartOTP

#### 5.2.1 GetOTP Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|consumerID|string|Yes|ConsumerID in the connection<br>information||
|consumerSecret|string|Yes|ConsumerSecret in the<br>connection information||

#### 5.2.2 GetOTP Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-404 Not Found|

##### Example
|getOTP Request|Response success|
|---|---|
|{<br>"consumerID":<br>"968d7b5940f5437583021aea2b038f35",|{<br>message: "Success",<br>status: 200<br>}|
|"consumerSecret":<br>"11ab3fc6af954c59ba646fac016f30cb"<br>}||

Case getOTP has error

|getOTP Request|Response fail|
|---|---|
|{|{|
|"consumerID":|"message": "ConsumerID is invalid",|
|"968d7b5940f5437583021aea2b038f<br>351",|"status": 400|
|"consumerSecret":<br>"11ab3fc6af954c59ba646fac016f30cb<br>"|}|
|}||

### 5.3 VerifyCode
- Url:  https://fc-tradeapi.ssi.com.vn/api/v2/Trading/verifyCode
- Method: POST

- Uses: verify OTP when User use 2FA: SMS/ Email/ SmartOTP and PIN when User use PIN

#### 5.3.1 VerifyCode Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|code|string|Yes|If User use 2FA -> fill SMS/<br>Email/ SmartOTP||
||||If User use PIN -> fill PIN||

#### 5.3.2 VerifyCode Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-404 Not Found|

##### Example
|verifyCode Request|Response success|
|---|---|
|{<br>"code": "123456"<br>}|{<br>"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.ewogICJhdWQiOiAiO”<br>}|

Case verifyCode has error

|verifyCode Request|Response fail|
|---|---|
|{|{|
|"code": "123456789"<br>}|Internal Server Error<br>}|

### 5.4 AccessToken
- Url:  https://fc-tradeapi.ssi.com.vn/api/v2/Trading/AccessToken

- Method: Post

- Uses: get token to use for other apis

#### 5.4.1 AccessToken Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|consumerID|string|Yes|ConsumerID in the connection<br>information||
|consumerSecret|string|Yes|ConsumerSecret in the<br>connection information||
|twoFactorType|string|Yes|Authentication type of account,<br>support type = 0 (PIN), type =<br>1 ( OTP)||
|code|string|Yes|Authentication code for trading<br>api||
|isSave|boolean|Yes|Save code for trading api|Include: true, fasle|

#### 5.4.2 AccessToken Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Include AccessToken||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-404 Not Found|

##### Example
|AccessToken Request|Response success|
|---|---|
|{<br>consumerID:<br>"example-consumer-id",<br>consumerSecret:<br>"example-consumer-secret",<br>twoFactorType: 0,<br>code: "123456789",<br>isSave: false<br>}|{<br>message: "Success",<br>status: 200,<br>data: {<br>"accessToken":<br>"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ”<br>}|

Case AccessToken has error

|Access Token Request|Response fail|
|---|---|
|{|{|
||message: "Key does not exist.",<br>status: 400,|
consumerID: data: null "example-consumer-id", consumerSecret: "example-consumer-secret", twoFactorType: 0, code: "123456789", isSave: true }

### 5.5 New Order
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/NewOrder

- Method: Post

- Header: X-Singature (Use PrivateKey to sign data request sent to server with RSA algothrim use SHA-256 hash function)

- Uses: Create order

#### 5.5.1 NewOrder Request
|Name of Element|Type|Required|Description|Valid Value or Format|
|---|---|---|---|---|
|instrumentID|String|Yes|Instrument ID||
|market|String|Yes||-VN: stock market<br>-VNFE: derivatives<br>market|
|buySell|String|Yes||-B: Buy<br>-S: Sell|
|orderType|String|Yes||-LO<br>-ATO<br>-ATC<br>-MP<br>-MTL<br>-MOK<br>-MAK<br>-PLO|
|channelID|String|Yes||TA|
|price|Number|Yes|If ordertype is LO, price must<br>be > 0.<br>If ordertype <> LO, price = 0||
|quantity|Number|Yes|||
|account|String|Yes|||
|requestID|String|Yes|Unique string of number in a<br>day with max length is 8|01234567|
|stopOrder|String|Yes||-True: For<br>conditional order<br>-False: for normal<br>order|
|---|---|---|---|---|
|stopPrice|Number|Yes|Default stopprice = 0<br>If stoporder = True, stopprice<br>>0||
|stopType|String|Yes|If stoporder = True, stopType<br>in value list|-D: Down<br>-U: Up<br>-V: Trailling Up<br>-E: Trailing Down<br>-O: OCO<br>-B: BullBear|
|stopStep|Number|Yes|Default stopstep = 0<br>If stoporder = True, stopstep<br>>=0||
|lossStep|Number|Yes|Default losstep = 0<br>If stoporder = True and<br>stopstyle = B, lossstep >0||
|profitStep|Number|Yes|Default losstep = 0<br>If stoporder = True and<br>stopstyle = B, profitstep >0||
|code|String|No|Trading code: PIN, OTP<br>If api AccessToken input:<br>isSave = false, code is<br>required||
|deviceId|string|Yes|Information about the device<br>that is placing the command|Satisfy 1 of the<br>following formats:<br>XX:XX:XX:XX:XX:XX,<br>XX-XX-XX-XX-XX-<br>XX,<br>XXXXXXXXXXXX.|
|userAgent|string|No|User Agent||

#### 5.5.2 NewOrder Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data||The original request input||
|message|String||Success|
|||Or Error msg|
|---|---|---|
|status|String|-200 if Success|
|||-400 if Failed|

##### Example
|NewOrder request|Response success|
|---|---|
|{<br>instrumentID: "SSI",<br>market: "VN",<br>buySell: "B",<br>orderType: "LO",<br>channelID: "IW",<br>price: 21000,<br>quantity: 300,<br>account: "0901351",<br>stopOrder: false,<br>stopPrice: 0,<br>stopType: "string",<br>stopStep: 0,<br>lossStep: 0,<br>profitStep: 0<br>requestID: "1678195",<br>code: “123456789”,<br>deviceId: "8C-EC-4B-D3-0B-96",<br>userAgent: “FCTrading”<br>}|{<br>message: "Success",<br>status: 200,<br>data: {<br>requestID: "1678195",<br>requestData:<br>{<br>instrumentID: "SSI",<br>market: "VN",<br>buySell: "B",<br>orderType: "LO",<br>channelID: "IW",<br>price: 21000,<br>quantity: 300,<br>account: "0901351",<br>stopOrder: false,<br>stopPrice: 0,<br>stopType: "string",<br>stopStep: 0,<br>lossStep: 0,<br>profitStep: 0<br>}<br>}<br>}|

##### Case newOrder has error
|NewOrder Request|Response fail|
|---|---|
|{<br>instrumentID: "SSI",<br>market: "VN",<br>buySell: "B",<br>orderType: "ATO",<br>channelID: "IW",<br>price: 21000,<br>quantity: 300,<br>account: "0901351",<br>stopOrder: false,<br>stopPrice: 0,<br>stopType: "string",<br>stopStep:0,|{<br>message: "Price is null or equal zero when order is<br>market order",<br>status: 400,<br>data: null<br>}|
lossStep: 0, profitStep: 0 requestID: "1678195", code: “123456789”, deviceId: "8C-EC-4B-D3-0B-96", userAgent: “FCTrading” }

### 5.6 Modify Order
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/ModifyOrder

- Method: Post

- Header: X-Singature (Use PrivateKey to sign data request sent to server with RSA algothrim use SHA-256 hash function)

- Uses: Modify order

#### 5.6.1 ModifyOrder Request
|Name of Element|Type|Required|Description|Valid Value or Format|
|---|---|---|---|---|
|orderID|String|Yes|ID of the order to modify||
|price|Number|Yes|New price if the price is<br>changed. Otherwise, price of<br>original order||
|quantity|Number|Yes|New quantity if the quantity is<br>changed. Otherwise, quantity<br>of the original order||
|account|String|Yes|Account of the original order||
|instrumentID|String|Yes|Symbol of the original order||
|marketID|String|Yes|MarketID of the original order|-VN: stock market<br>-VNFE: derivatives<br>market|
|buySell|String|Yes|Side of the original order|-B: Buy<br>-S: Sell|
|requestID|String|Yes|Unique string of number in a<br>day with max length of 8||
|orderType|String|Yes|Order Type of the original<br>order|-LO<br>-ATO<br>-ATC<br>-MP<br>-MTL<br>-MOK<br>-MAK<br>-PLO|
|code|String|No|Trading code: PIN, OTP<br>If api AccessToken input:<br>isSave = false, code is<br>required||
|deviceId|string|Yes|Information about the device<br>that is placing the command|Satisfy 1 of the<br>following formats:<br>XX:XX:XX:XX:XX:XX,<br>XX-XX-XX-XX-XX-<br>XX,<br>XXXXXXXXXXXX.|
|userAgent|string|No|User Agent||

#### 5.6.2 ModifyOrder Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Original request||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|

##### Example
Modify Request Response success { { requestID: "93235974", message: "Success", orderID: "12658867", status: 200, price: 1410, data: { quantity: 2, requestID: "93235974", account: "0901358", requestData: { instrumentID: "VN30F2106", orderID: "12658867", marketID: "VNFE", price: 1410, buySell: "B", quantity: 2, orderType: "LO" account: "0901358", code:”123456789”, instrumentID: "VN30F2106", deviceId: "8C-EC-4B-D3-0B-96", marketID: "VNFE", userAgent: “FCTrading” buySell: "B", } orderType: "LO" } } }

Case modifyOrder has error
|ModifyOrder Request|Response fail|
|---|---|
|{<br>requestID: "93235971",<br>orderID: "",<br>price: 1410,<br>quantity: 2,<br>account: "0901358",<br>instrumentID: "VN30F2106",<br>marketID: "VNFE",<br>buySell: "B",<br>orderType: "LO"<br>code:”123456789”,<br>deviceId: "8C-EC-4B-D3-0B-96",<br>userAgent: “FCTrading”<br>}|{<br>data: null,<br>message: "’Order ID’ must not be empty ",<br>status: 400<br>}|

### 5.7 Cancel Order
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/CancelOrder

- Method: Post

- Header: X-Singature (Use PrivateKey to sign data request sent to server with RSA algothrim use SHA-256 hash function)

- Uses: Cancel order

#### 5.7.1 CancelOrder Request
|Name of Element|Type|Required|Description|Valid Value or Format|
|---|---|---|---|---|
|orderID|String|Yes|||
|account|String|Yes|||
|marketID|String|Yes||-VN: stock market<br>-VNFE: derivatives<br>market|
|instrumentID|String|Yes|||
|buySell|String|Yes||-B: Buy<br>-S: Sell|
|requestID|String|Yes|Unique string of number in a<br>day with max length is 8|123445678|
|code|String|No|Trading code: PIN, OTP<br>If api AccessToken input:<br>isSave = false, code is<br>required||
|deviceId|string|Yes|Information about the device<br>that is placing the command|Satisfy 1 of the<br>following formats:<br>XX:XX:XX:XX:XX:XX,<br>XX-XX-XX-XX-XX-|
|||||XX,<br>XXXXXXXXXXXX.|
|---|---|---|---|---|
|userAgent|string|No|User Agent||

#### 5.7.2 CancelOrderResponse
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Unique string of number in a<br>day with max length is 8|12345678|
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|

##### Example
Cancel Request Response success { { orderID: "12658867", message: "Success", account: "0901358", status: 200, marketID: "VNFE", data: { instrumentID: "VN30F2106", requestID: "52513603", buySell: "B", requestData: { requestID: "52513603" orderID: "12658867", code:”123456789”, account: "0901358", deviceId: "8C-EC-4B-D3-0B-96", marketID: "VNFE", userAgent: “FCTrading” instrumentID: "VN30F2106", } buySell: "B", requestID: "52513603" } } }

Case cancelOrder has error

CancelOrder Request Response fail { { orderID: " ", message: "’Order ID' must not be empty.", account: "0901358", status: 400, marketID: "VNFE", data: null instrumentID: "VN30F2106", } buySell: "B", requestID: "52513603" code:”123456789”, deviceId: "8C-EC-4B-D3-0B-96", userAgent: “FCTrading” }
### 5.8 DerNewOrder
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/derNewOrder

- Method: POST

- Uses: Create order derivatives

#### 5.8.1 DerNewOrder Request
|Name of Element|Type|Required|Description|Valid Value or Format|
|---|---|---|---|---|
|instrumentID|String|Yes|Instrument ID||
|market|String|Yes||VNFE: derivatives<br>market|
|buySell|String|Yes||-B: Buy<br>-S: Sell|
|orderType|String|Yes||-LO<br>-ATO<br>-ATC<br>-MTL<br>-MOK<br>-MAK|
|channelID|String|Yes||TA|
|price|Number|Yes|If ordertype is LO, price must<br>be > 0.<br>If ordertype <> LO, price = 0||
|quantity|Number|Yes|||
|account|String|Yes|||
|requestID|String|Yes|Unique string of number in a<br>day with max length is 8|01234567|
|stopOrder|String|Yes|Only for: market = VNFE|-True: For<br>conditional order<br>-False: for normal<br>order|
|stopPrice|Number|Yes|Default stopprice = 0<br>If stoporder = True, stopprice<br>>0||
|stopType|String|Yes|If stoporder = True, stopType<br>in value list|-D: Down<br>-U: Up<br>-V: Trailling Up<br>-E: Trailing Down<br>-O: OCO|
|||||-B: BullBear|
|---|---|---|---|---|
|stopStep|Number|Yes|Default stopstep = 0<br>If stoporder = True, stopstep<br>>=0||
|lossStep|Number|Yes|Default losstep = 0<br>If stoporder = True and<br>stopstyle = B, lossstep >0||
|profitStep|Number|Yes|Default losstep = 0<br>If stoporder = True and<br>stopstyle = B, profitstep >0||
|code|String|No|Trading code: PIN, OTP<br>If api AccessToken input:<br>isSave = false, code is<br>required||
|deviceId|string|Yes|Information about the device<br>that is placing the command|Satisfy 1 of the<br>following formats:<br>XX:XX:XX:XX:XX:XX,<br>XX-XX-XX-XX-XX-<br>XX,<br>XXXXXXXXXXXX.|
|userAgent|string|No|User Agent||

#### 5.8.2 DerNewOrder Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data||The original request input||
|message|String||Success|
||||Or Error msg|
|status|String||-200 if Success<br>-400 if Failed|

##### Example
|DerNewOrder request|Response success|
|---|---|
|{<br>instrumentID: "VN30F2306",<br>market: "VNFE",<br>buySell: "B",<br>orderType: "LO",<br>channelID:"TA",|{<br>"message": "Success",<br>"status": 200,<br>"data": {<br>"requestID": "3407154",<br> "requestData": {|
|price: 1200,<br>quantity: 10,<br>account: "1184418",<br>stopOrder: false,<br>stopPrice: 0,<br>stopType: "string",<br>stopStep: 0,|"instrumentID": "VN30F2306",<br>"market": "VNFE",<br>"buySell": "B",<br>"orderType": "LO",<br>"channelID": "TA",<br>"price": 1200,<br>"quantity": 100,|
|---|---|
|lossStep: 0,<br>profitStep: 0<br>requestID: "1678198",<br>code: “123456789”,<br>deviceId: "8C-EC-4B-D3-0B-96",<br>userAgent: “FCTrading”<br>}|"account": "1184418",<br>"stopOrder": false,<br>"stopPrice": 0,<br>"stopType": "",<br>"stopStep": 0,<br>"lossStep": 0,<br>"profitStep": 0<br>}<br>}<br>}|

##### Case derNewOrder has error
|DerNewOrder Request|Response fail|
|---|---|
|{<br>instrumentID: "VN30F2306",<br>market: "VNFE",<br>buySell: "B",<br>orderType: "MTL",<br>channelID: "TA",<br>price: 1200,<br>quantity: 10,<br>account: "1184418",<br>stopOrder: false,<br>stopPrice: 0,<br>stopType: "string",<br>stopStep: 0,<br>lossStep: 0,<br>profitStep: 0<br>requestID: "1678198",<br>code: “123456789”,<br>deviceId: "8C-EC-4B-D3-0B-96",<br>userAgent: “FCTrading”<br>}|{<br>message: "Price is null or equal zero when order is<br>market order",<br>status: 400,<br>data: null<br>}|

### 5.9 DerModifyOrder
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/derModifyOrder

- Method: Post

- Header: X-Singature (Use PrivateKey to sign data request sent to server with RSA algothrim use SHA-256 hash function)

- Uses: Modify order derivatives
#### 5.9.1 DerModifyOrder Request
|Name of Element|Type|Required|Description|Valid Value or Format|
|---|---|---|---|---|
|orderID|String|Yes|ID of the order to modify||
|price|Number|Yes|New price if the price is<br>changed. Otherwise, price of<br>original order||
|quantity|Number|Yes|New quantity if the quantity is<br>changed. Otherwise, quantity<br>of the original order||
|account|String|Yes|Account of the original order||
|instrumentID|String|Yes|Symbol of the original order||
|marketID|String|Yes|MarketID of the original order|VNFE: derivatives<br>market|
|buySell|String|Yes|Side of the original order|-B: Buy<br>-S: Sell|
|requestID|String|Yes|Unique string of number in a<br>day with max length of 8||
|orderType|String|Yes|Order Type of the original<br>order|-LO<br>-ATO<br>-ATC<br>-MTL<br>-MOK<br>-MAK|
|code|String|No|Trading code: PIN, OTP<br>If api AccessToken input:<br>isSave = false, code is<br>required||
|deviceId|string|Yes|Information about the device<br>that is placing the command|Satisfy 1 of the<br>following formats:<br>XX:XX:XX:XX:XX:XX,<br>XX-XX-XX-XX-XX-<br>XX,<br>XXXXXXXXXXXX.|
|userAgent|string|No|User Agent||

#### 5.9.2 DerModifyOrder Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Original request||
|message|String||Success or Error msg|
|Status|String|-200 if Success|
|---|---|---|
|||-400 if Failed|

##### Example
|Modify Request|Response success|
|---|---|
|{<br>requestID: "93235974",<br>orderID: "12658867",<br>price: 1410,<br>quantity: 3,<br>account: "1184418",<br>instrumentID: "VN30F2306",<br>marketID: "VNFE",<br>buySell: "B",<br>orderType: "LO"<br>code:”123456789”,<br>deviceId: "8C-EC-4B-D3-0B-96",<br>userAgent: “FCTrading”<br>}|{<br>message: "Success",<br>status: 200,<br>data: {<br>requestID: "93235974",<br>requestData: {<br>orderID: "12658867",<br>price: 1410,<br>quantity: 3,<br>account: "1184418",<br>instrumentID: "VN30F2306",<br>marketID: "VNFE",<br>buySell: "B",<br>orderType: "LO"<br>}<br>}<br>}|

Case derModifyOrder has error

|DerModifyOrder Request|Response fail|
|---|---|
|{<br>requestID: "93235974",<br>orderID: "",<br>price: 1410,<br>quantity: 3,<br>account: "1184418",<br>instrumentID: "VN30F2306",<br>marketID: "VNFE",<br>buySell: "B",<br>orderType: "LO"<br>code:”123456789”,<br>deviceId: "8C-EC-4B-D3-0B-96",<br>userAgent: “FCTrading”<br>}|{<br>data: null,<br>message: "’Order ID’ must not be empty ",<br>status: 400<br>}|

### 5.10 DerCancelOrder
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/derCancelOrder

- Method: Post

- Header: X-Singature (Use PrivateKey to sign data request sent to server with RSA algothrim use SHA-256 hash function)
- Uses: Cancel order derivatives

#### 5.10.1 CancelOrder Request
|Name of Element|Type|Required|Description|Valid Value or Format|
|---|---|---|---|---|
|orderID|String|Yes|||
|account|String|Yes|||
|marketID|String|Yes||VNFE: derivatives<br>market|
|instrumentID|String|Yes|||
|buySell|String|Yes||-B: Buy<br>-S: Sell|
|requestID|String|Yes|Unique string of number in a<br>day with max length is 8|123445678|
|code|String|No|Trading code: PIN, OTP<br>If api AccessToken input:<br>isSave = false, code is<br>required||
|deviceId|string|Yes|Information about the device<br>that is placing the command|Satisfy 1 of the<br>following formats:<br>XX:XX:XX:XX:XX:XX,<br>XX-XX-XX-XX-XX-<br>XX,<br>XXXXXXXXXXXX.|
|userAgent|string|No|User Agent||

#### 5.10.2 DerCancelOrderResponse
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Unique string of number in a<br>day with max length is 8|12345678|
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|

##### Example
|Cancel Request|Response success|
|---|---|
|{<br>orderID: "12658867",|{<br>message: "Success",|
|account:"1184418",|status: 200,|
|marketID: "VNFE",|data: {|
|---|---|
|instrumentID: "VN30F2306",<br>buySell: "B",<br>requestID: "52513603"<br>code:”123456789”,<br>deviceId: "8C-EC-4B-D3-0B-96",<br>userAgent: “FCTrading”<br>}|requestID: "52513603",<br>requestData: {<br>orderID: "12658867",<br>account: "1184418",<br>marketID: "VNFE",<br>instrumentID: "VN30F2106",<br>buySell: "B",<br>requestID: "52513603"<br>}<br>}<br>}|

##### Case derCancelOrder has error
|DerCancelOrder Request|Response fail|
|---|---|
|{<br>orderID: "",<br>account: "1184418",<br>marketID: "VNFE",<br>instrumentID: "VN30F2306",<br>buySell: "B",<br>requestID: "52513603"<br>code:”123456789”,<br>deviceId: "8C-EC-4B-D3-0B-96",<br>userAgent: “FCTrading”<br>}|{<br>message: "’Order ID' must not be empty.",<br>status: 400,<br>data: null<br>}|

### 5.11 Order History
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/orderHistory

- Method: Get

- Uses: query order

#### 5.11.1 OrderHistory Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|account|String|Yes|AccountNo Stock market or<br>Derivatives market||
|startDate|Date|Yes|Start date of search|DD/MM/YYYY|
|endDate|Date|Yes|End date of search|DD/MM/YYYY|

#### 5.11.2 OrderHistory Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Order list of account in search<br>time||
|message|String|Success or Error msg|
|---|---|---|
|Status|String|-200 if Success<br>-400 if Failed|

##### Example
|OrderHistory Request|Response success|
|---|---|
|{<br>account: "0901358",<br>startDate: “18/11/2020”,<br>endDate: “18/11/2020”<br>}|{<br>message: "Success",<br>status: 200,<br>data: {<br>orderHistories: [<br>{<br>uniqueID: null,<br>orderID: "12626539",<br>buySell: "B",<br>price: 800.0,<br>quantity: 10,<br>filledQty: 0,<br>orderStatus: "RJ",<br>marketID: "VNFE",<br>inputTime: "1603157594668",<br>modifiedTime: "1603157594668",<br>instrumentID: "VN30F2012",<br>orderType: "LO",<br>cancelQty: 0,<br>avgPrice: 0.0,<br>isForcesell: null,<br>isShortsell: null<br>}<br>],<br>account: "0901358"<br>}<br>}|

Case orderHistory has error

|OrderHistory Request|Response fail|
|---|---|
|{<br>account: "0901358",<br>startDate: “”,|{<br>message: "'Start Date' must not be empty; StartDate is required.",<br>status: 400,|
|endDate: “18/11/2020”<br>}|data: null<br>}|

### 5.12 Stock Position
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/stockPosition

- Method: Get
- Uses: Get portfolio information of accounts stock

#### 5.12.1 StockPosition Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|account|String|Yes|AccountNo Stock market||

#### 5.12.2 StockPosition Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|porfolio of account include:<br>- account<br>- stockPositions<br>- totalMarketValue||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|
|account|String|||
|Detail of data||||
|totalMarketValue|number|= sum (maketprice *<br>onhand) of all instrumentid||
|stockPositions||Porfolio of account||
|marketID|String|||
|instrumentID|String|||
|onHand|number|Total quantity of securities||
|block|number|Quantity of blockaded<br>securities||
|bonus|number|||
|buyT0||Intraday bought||
|buyT1|number|Quantity securities bought<br>matched of day T-1||
|buyT2|number|Quantity securities bought<br>matched of day T-2||
|sellT0|number|Intraday sold||
|sellT1|number|Quantity securities sold<br>matched of day T-1||
|sellT2|number|Quantity securities sold<br>matched of day T-2|
|---|---|---|
|avgPrice|number|Average matched price|
|mortgage|number|Quantity of mortgage<br>securities|
|holdForTrade|number|Securities awaiting fo trade|
|marketPrice|number|Market price of securities|

##### Example
|StockDeposition Request|Response success|
|---|---|
|{<br>account: "0901351",<br>}|{<br>message: "Success",<br>status: 200,<br>data: {<br>account": "0901351,<br>totalMarketValue: 0,<br>stockPositions: [<br>{<br>marketID: "VN",<br>instrumentID: "SSI",<br>onHand: 50300,<br>block: 0,<br>bonus: 7425,<br>buyT0: 0,<br>buyT1: 0,<br>buyT2: 0,<br>sellT0: 0,<br>sellT1: 0,<br>sellT2: 0,<br>avgPrice: 18505,<br>mortgage: 0,<br>sellableQty: 50300,<br>holdForTrade: 0,<br>marketPrice: 0<br>}<br>]<br>}<br>}|

Case stockPosition has error

|StockDeposition Request|Response fail|
|---|---|
|{<br>account: "0901352"|{<br>message: "Account is not exist.",|
|}|status: 400,<br>data: null|
}

### 5.13 Derivatives Position
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/derivPosition

- Method: Get

- Uses: Get portfolio information of derivative accounts stock

#### 5.13.1 DerPosition Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|account|String|Yes|AccountNo Derivatives market||
|querySummary|Boolean|Yes|Default is true (net position)||

#### 5.13.2 DerPosition Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|porfolio of account include:<br>- openPositions<br>- closePositions<br>- account||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|
|Detail of data||||
|account|String|||
|openPosition||Open position||
|closePosition||Close position||
|Detail of position||||
|marketID|String|||
|instrumentID|String|||
|longQty|number|Long position||
|shortQty|number|Short position||
|net|number|Net position||
|bidAvgPrice|number|Average bid price||
|askAvgPrice|number|Average ask price||
|tradePrice|number|Trade price|
|---|---|---|
|marketPrice|number|Maket price|
|floatingPL|number|Temporarily calculated profit<br>and loss|
|tradingPL|number|Calculated profit and loss|

##### Example
|DerPosition Request|Response success|
|---|---|
|{<br>account: "0901358",<br>querySummary: true<br>}|{<br>message: "Success",<br>status: 200,<br>data: {<br>account: "0901358",<br>openPosition: [<br>{<br>marketID: "VNFE",<br>instrumentID: "VN30F2106",<br>longQty: 8,<br>shortQty: 0,<br>net: 8,<br>bidAvgPrice: 0,<br>askAvgPrice: 0,<br>tradePrice: 1452.7,<br>marketPrice: 1452.7,<br>floatingPL: 0,<br>tradingPL: 0<br>}   ],<br>closePosition: [ ]<br>}<br>}|

Case derPosition has error

|DerPosition Request|Response fail|
|---|---|
|{<br>account: "",<br>querySummary: true<br>}|{<br>message: "'Account' must not be empty ",<br>status: 400,<br>data: null<br>}|

### 5.14 Max Buy Quantity
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/maxBuyQty

- Method: Get

- Uses: Get max buy quantity of account
#### 5.14.1 MaxBuyQty Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|account|String|Yes|AccountNo Stock market or<br>Derivatives market||
|instrumentID|String|Yes|||
|price|Number|Yes|||

#### 5.14.2 MaxBuyQty Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|List of data includes:<br>- account<br>- maxbuyqty<br>- marginratio<br>- purchasingpower||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|

##### Example
MaxBuyQty Request Response success { { account: "0041691", message: "Success", instrumentD: “SSI”, status: 200, price:17 data: { } account: "0041691", maxBuyQty: 8241440, marginRatio: "50%", purchasingPower: 99292902171 } }

Case maxBuyQty has error

|MaxBuyQty Request|Response fail|
|---|---|
|{<br>account: "0041695",<br>querySummary: true|{<br>message: "Account is not exist.",<br>status: 400,|
|}|data: null<br>}|
### 5.15 Max Sell Quantity
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/maxSellQty

- Method: Get

- Uses: Get max sell quantity of account

#### 5.15.1 MaxSellQty Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|account|String|Yes|AccountNo Stock market or<br>Derivatives market||
|instrumentID|String|Yes|||

#### 5.15.2 MaxSellQty Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|List of data includes:<br>- account<br>- maxsellqty||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|

##### Example
|MaxSellQty Request|Response success|
|---|---|
|{<br>account: "0041691",<br>intrumentID: “SSI”<br>}|{<br>"message": "Success",<br>"status": 200,<br>"data": {<br>"account": "0041691",<br>"maxSellQty": 2000<br>}<br>}|

Case maxSellQty has error

|MaxSellQty Request|Response fail|
|---|---|
|{<br>account: "0041695",|{<br>message: "Account is not exist.",|
|intrumentID: “SSI”|status: 400,|
|}|data: null|
}

### 5.16 Account Balance
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/cashAcctBal

- Method: Get

- Uses: Get account balance information

#### 5.16.1 AccountBalance Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|account|String|Yes|AccountNo Stock market||

#### 5.16.2 AccountBalance Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Information of account balance||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|
|Detail of data||||
|account|String|||
|cashbal|number|Total cash balance||
|cashonhold|number|Total cash on hold||
|secureamount|number|Secure amount intraday||
|withdrawable|number|Withdrawable money||
|receivingcasht1|number|Receiving amount T+1||
|receivingcasht2|number|Receiving amount T+2||
|matchedbuyvolume|number|Matched buy volume||
|matchedsellvolume|number|Matched sell volume||
|unmatchedbuyvolume|number|Unmatched buy volume||
|unmatchedsellvolume|number|Unmatched sell volume||
|paidcasht1|number|Paid cash T+1||
|paidcasht2|number|Paid cash T+2||
|Cia|number|cash in advance||
|debt|number|Total debt|
|---|---|---|
|purchasingpower|number|Purchasing power|
|totalasset|number|Total asset (not debt<br>reduction)|

##### Example
|AccountBalance Request|Response success|
|---|---|
|{<br>account: "0901351",<br>}|{<br>message: "Success",<br>status: 200,<br>data: {<br>account: "0901351",<br>cashBal: 7459369481,<br>cashOnHold: 0,<br>secureAmount: 0,<br>withdrawable: 7459367581,<br>receivingCashT1: 0,<br>receivingCashT2: 0,<br>matchedBuyVolume: 0,<br>matchedSellVolume: 0,<br>debt: 1900,<br>unMatchedBuyVolume: 0,<br>unMatchedSellVolume: 864619337,<br>paidCashT1: 0,<br>paidCashT2: 0,<br>cia: 0,<br>purchasingPower: 7459367581,<br>totalAssets: 9726161481<br>}<br>}|

Case AccountBalance has error

|AccountBalance Request|Response fail|
|---|---|
|{<br>account: "0901357",<br>}|{<br>message: "Account is not exist.",<br>status: 400,<br>data: null<br>}|

### 5.17 Purchasing power Margin of Account
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/ppmmraccount

- Method: Get

- Uses: Get purchasing power margin of Account
#### 5.17.1 ppmmrAccount Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|account|String|Yes|AccountNo Stock market||

#### 5.17.2 ppmmrAccount Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Information of account ppmmr||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|
|Detail of data||||
|collateralAsset|number|Total collateral Asset||
|callLMW|number|Maintenance margin ratio||
|liability|number|Total debt||
|eeOrigin|number|||
|forceLMV|number|||
|equity|number|Net LMV||
|ee|number|||
|callMargin|number|Call Margin||
|cashBalance|number|Cash Balance||
|purchasingPower|number|Purchasing Power||
|callForcesell|number|Call Forcesell||
|lmv|number|Stock Market Value (margin)||
|marginCall|number|Call Amount||
|withdrawal|number|Withdrawal||
|collateralA|number|||
|action|string|Action call margin||
|marginRation|number|Margin Ratio||
|debt|number|Debt||
|accruedInterest|number|Accrued Interest||
|holdRight|number|Right subcription||
|preLoan|number|Pre Debt|
|---|---|---|
|fees|number|Fees|
|buyUnmatch|number|Unmatched buy volume|
|ap|number|Matched buy volume|
|apT1|number|Receiving amount T+1|
|sellUnmatch|number|Unmatched sell volume|
|cia|number|cash in advance|
|ar|number|Matched sell volume|
|arT1|number|Paid cash T+1|
|ppCredit|number||
|creditLimit|number||
|totalAssets|number||
|MarginCallMVSold|number|Call LMV Sold|
|lmvNonMarginable|number|Stock Market Value|
|eeCredit|number||
|totalEquity|number||
|eE90|number||
|eE80|number||
|eE70|number||
|eE60|number||
|eE50|number||

##### Example
|ppmmrAccount Request|Response success|
|---|---|
|{<br>account: "0901356",<br>}|{<br>message: "Success",<br>status: 200,<br>data: {<br>collateralAsset: 8404515731,<br>callLMW: 0,<br>liability: 1900,<br>eeOrigin: 7459367581,<br>forceLMV: 0,<br>equity: 8404513831,<br>ee: 7459367581,|
|callMargin: 0,|
|---|
|cashBalance: 7459369481,<br>purchasingPower: 7459367581,<br>callForcesell: 0,<br>lmv: 945146250,<br>marginCall: 0,<br>withdrawal: 7459367581,<br>collateralA: 0,<br>action: "",|
|marginRatio: 1,<br>debt: 0,|
|accruedInterest: 0,<br>holdRight: 0,<br>preLoan: 0,<br>fees: 1900,|
|buyUnmatch: 0,<br>ap: 0,<br>apT1: 0,<br>sellUnmatch: 0,<br>cia: 0,|
|ar: 0,<br>arT1: 0,<br>ppCredit: 7459367581,<br>creditLimit: 0,<br>totalAssets: 9615302981,|
|marginCallLMVSold: 0,<br>lmvNonMarginable: 1210787250,<br>eeCredit: 7459367581,<br>totalEquity: 9615301081,<br>eE90: 8288186201,<br>eE80: 9324209476,|
|eE70: 10656239401,|
|eE60: 12432279302,|
|eE50: 14918735162|
|}<br>}|

Case ppmmrAccount has error

|ppmmrAccount Request|Response fail|
|---|---|
|{|{|
|<br>account: "0901357",<br>}|<br>message: "Account is not exist”,<br>status: 400,<br>data: null<br>}|

### 5.18 Derivartives Account Balance
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/derivAcctBal

- Method: Get

- Uses: Get purchasing power margin of Account
#### 5.18.1 DerAccountBalance Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|account|String|Yes|AccountNo Derivatives market||

#### 5.18.2 DerAccountBalance Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Information of account balance||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|
|Detail of data||||
|account|String|accountid||
|accountbalance|number|total cash balance||
|fee|number|Fee||
|commission|number|Commission||
|interest|number|Interest||
|Loan|number|Loan||
|deliveryamount|number|Delivery amount||
|floatingpl|number|Temporarily calculated profit<br>and loss||
|totalpl|number|Total profit and loss||
|marginable|number|Money can deposit in SSI||
|depositable|number|Money can deposit in VND||
|rccall|number|||
|withdrawable|number|Withdrawable amount||
|noncashdrawablerccall|number|Stock value can be withdrawn||
|internalassets||Includes:<br>-<br>Cash<br>-<br>Validnoncash<br>-<br>totalvalue:<br>-<br>maxvalidnoncash<br>-<br>cashwithdrawable<br>-<br>Ee||
|exchangeassets|Includ|es:|
|---|---|---|
||-<br>-<br>-<br>-<br>-<br>-|Cash<br>Validnoncash<br>totalvalue:<br>maxvalidnoncash<br>cashwithdrawable<br>Ee|
|internalmargin|Includ|es:|
||-<br>-<br>-<br>-<br>-<br>-<br>-<br>-|initialmargin<br>deliverymargin<br>marginreq<br>accountratio<br>usedlimitwarninglevel1<br>usedlimitwarninglevel2<br>usedlimitwarninglevel3<br>margincall|
|exchangemargin|Includ<br>-<br>-<br>-<br>-<br>-<br>-|es:<br>marginreq<br>accountratio<br>usedlimitwarninglevel1<br>usedlimitwarninglevel2<br>usedlimitwarninglevel3<br>margincall|

##### Example
|DerAccountBalance Request|Response success|
|---|---|
|{<br>account: "0901358",<br>}|{<br>message: "Success",<br>status: 200,<br>data: {<br>account: "0901358",<br>accountBalance: 11166309263,<br>fee: 0,<br>commission: 0,<br>interest: 1514965,<br>loan: 0,<br>deliveryAmount: 0,<br>floatingPL: 0,<br>totalPL: 0,<br>marginable: 0,<br>depositable: 1148597520,<br>rcCall: 0,<br>withdrawable: 10912447363,<br>nonCashDrawableRCCall: 0,<br>internalAssets: {|
|cash: 1165730020,|
|---|
|validNonCash: 0,<br>totalValue: 11166309263,<br>maxValidNonCash: 0,<br>cashWithdrawable: 1148597520,|
|ee: 8197059272|
|},<br>exchangeAssets: {<br>cash: 10000579243,<br>validNonCash: 0,|
|totalValue: 10000579243,|
|maxValidNonCash: 0,|
|cashWithdrawable: 9763849843,<br>ee: 7322887382|
|},<br>internalMargin: {|
|initialMargin: 172660800,<br>deliveryMargin: 0,<br>marginReq: 172660800,<br>accountRatio: 1.5462656096416592,<br>usedLimitWarningLevel1: 75,|
|usedLimitWarningLevel2: 85,|
|usedLimitWarningLevel3: 90,|
|marginCall: 0|
|},<br>exchangeMargin: {<br>marginReq: 172660800,<br>accountRatio: 1.7265079932330476,<br>usedLimitWarningLevel1: 75,|
|usedLimitWarningLevel2: 85,|
|usedLimitWarningLevel3: 90,<br>marginCall: 0<br>}|
|}|
|<br>}|

##### Case DerAccountBalance has error
|DerAccountBalance Request|Response fail|
|---|---|
|{|{|
|<br>account: "0901357",<br>}|<br>message: "Account is not exist”,<br>status: 400,<br>data: null<br>}|

### 5.19 AuditOrderBook
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/auditOrderBook

- Method: Get

- Uses: The information given will be similar to what you see when you connect to a streaming service and it will tell you if your order was successful or not.
#### 5.19.1 auditOrderBook Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|account|String|Yes|AccountNo Stock market or<br>Derivatives market||

#### 5.19.2 auditOrderBook Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Order list of account in search<br>time||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|

##### Example
|auditOrderBook Request|Response success|
|---|---|
|{<br>account: "0001011"<br>}|{<br>"message": "Success",<br>"status": 200,<br>"data": {<br>"account": "0001011",<br>"orders": [<br>{<br>"uniqueID": "71298546",<br>"orderID": "19450163",<br>"buySell": "B",<br>"price": 15400,<br>"quantity": 100,<br>"filledQty": 0,<br>"orderStatus": "QU",<br>"marketID": "VN",<br>"inputTime": "1689653132000",<br>"modifiedTime": "1689653133000",<br>"instrumentID": "BSR",<br>"orderType": "LO",<br>"cancelQty": 0,<br>"avgPrice": 0,<br>"isForcesell": "F",<br>"isShortsell": "F",<br>"rejectReason": "0",<br>"lastErrorEvent": {<br>"uniqueID": "45706921",<br>"orderID": "19450163",<br> "buySell":"B",|
|"price": 15400,|
|---|
|"quantity": 100,<br>"filledQty": 0,<br>"orderStatus": "RJ",<br>"marketID": "VN",<br>"inputTime": "1689653132000",<br>"modifiedTime": "1689653133000",<br>"instrumentID": "BSR",<br>"orderType": "LO",<br>"cancelQty": 0,<br>"avgPrice": 0,<br>"isForcesell": "F",<br>"isShortsell": "F",|
|"rejectReason": "Order price upper limit can not|
|exceed spread limit",<br>"lastErrorEvent": null<br>}|
|},<br>{<br>"uniqueID": "69252413",<br>"orderID": "19450161",<br>"buySell": "B",<br>"price": 24050,<br>"quantity": 100,|
|"filledQty": 0,|
|"orderStatus": "CL",|
|"marketID": "VN",|
|"inputTime": "1689652691000",<br>"modifiedTime": "1689652692000",<br>"instrumentID": "SSI",<br>"orderType": "MP",<br>"cancelQty": 100,<br>"avgPrice": 0,<br>"isForcesell": "F",<br>"isShortsell": "F",<br>"rejectReason": "0",<br>"lastErrorEvent": null<br>}<br>]<br>}|
|}|

Case auditOrderBook has error

|auditOrderBook Request|Response fail|
|---|---|
|{|{|
|<br>account: "118441”<br>}|<br>"message": "Account is not exist.",<br>"status": 400,<br>"data": null<br>}|
### 5.20 OrderBook
- Url: https://fc-tradeapi.ssi.com.vn/api/v2/Trading/OrderBook

- Method: Get

- Uses: returns intraday order history

#### 5.20.1 OrderBook Request
|Name of Element|Type|Required|Description|Valid Value or<br>Format|
|---|---|---|---|---|
|account|String|Yes|AccountNo Stock market or<br>Derivatives market||

#### 5.20.2 OrderBook Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|data|String|Order list of account in search<br>time||
|message|String||Success or Error msg|
|Status|String||-200 if Success<br>-400 if Failed|

##### Example
|orderBook Request|Response success|
|---|---|
|{<br>account: "1184418"<br>}|{<br>"message": "Success",<br>"status": 200,<br>"data": {<br>"account": "1184418",<br>"orders": [<br>{<br>"uniqueID": "73885549",<br>"orderID": "T202306146w273885549",<br>"buySell": "B",<br>"price": 1000,<br>"quantity": 100,<br>"filledQty": 0,<br>"orderStatus": "RJ",<br>"marketID": "VNFE",<br>"inputTime": "1686730747945",<br>"modifiedTime": "1686730747945",<br>"instrumentID": "VN30F2306",<br>"orderType": "LO",<br>"cancelQty": 0,<br>"avgPrice": 0,<br>"isForcesell": "F",<br>"isShortsell": "F",|
|"rejectReason": "Invalid market status"<br>}<br>]|
|---|
|}<br>}|

Case orderBook has error

|orderBook Request|Response fail|
|---|---|
|{<br>account: "118441”|{<br>"message": "Account is not exist.",|
|}|"status": 400,<br>"data": null<br>}|

## 6 TAPI Streaming
### 6.1 Order Streaming
Streaming is available at https://fc-tradehub.ssi.com.vn

#### Order Streaming setup
To use the order stream, the client needs to init the stream and then bind to subcribe the update

- Initstream:

client.initStream({ url: config.stream_url, consumer_id: config. ConsumerID, consumer_secret: config. ConsumerSecret, notify_id:   0

});

Note: when server disconnected:  notify_id = 0 return data from the start day, notify_id  =  -1 return data fromreconnected,  notify_id  = n return data from n

- Bind to subcribe:

   - Bind a callback function to: event onNewOrder:

      - client.bind(client.events.onOrderUpdate, function (e,data) {

      - console.log(e + ": ");

      - console.log(JSON.stringify(data));

      - });

   - Bind a callback function to event onOrderError:: client.bind(client.events.onOrderError, function (e, data) {

      - console.log(e + ": ");

      - console.log(JSON.stringify(data));

      - });
#### 6.1.1 Order Event Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|type|string|Type of event||
|uniqueID|String|||
|prefix|String|||
|ipAddress|String|To detect the request<br>from client to TAPI||
|notifyID|String|The serial number of<br>msg||
|orderID|String|||
|instrumentID|String|Stock symbol||
|buySell|String||-B: Buy<br>-S: Sell|
|orderType|String||-LO<br>-ATO<br>-ATC<br>-MP<br>-MTL<br>-MOK<br>-MAK<br>-PLO|
|price|Number|||
|quantity|Number|||
|marketID|String||-VN: stock market<br>-VNFE: derivatives market|
|origRequestID|String|||
|account|String|||
|cancelQty|Number|Total quantity was<br>cancelled||
|osqty|Number|Total quantity was not<br>matched||
|filledQty|Number|Total quantity was<br>matched||
|avgPrice|Number|||
|channel|String|||
|inputTime|String||Unixtime|
|modifiedTime|String||Unixtime|
|---|---|---|---|
|isForceSell|Boolean|||
|isShortSell|Boolean|||
|orderStatus|String||-WA: WaitingApproval<br>-RS: ReadyToSendExchange<br>-SD: SentToExchange<br>-QU: QueueInExchange<br>-FF: FullyFilled<br>-PF: PartiallyFilled<br>-FFPC: FullyFilledPartiallyCancelled<br>-WM: WaitingModify<br>-WC: WaitingCancel<br>-CL: Cancelled<br>-RJ: Rejected<br>-EX: Expired<br>-SOR: StopOrderReady<br>-SOS: StopOrderSent<br>-IAV: PresessionOrder<br>-SOI: PresessionstopOrder|
|origOrderID|String|The order id of the<br>parent conditional order||
|rejectReason|String|||
|stopOrder|String|Defined order type is<br>Stop order|Include: true, false|
|stopType|String|If stopOrder = True,<br>stopType in value list|-D: Down<br>-U: Up<br>-V: Trailling Up<br>-E: Trailing Down<br>-O: OCO<br>-B: BullBear|
|stopStep|Number|||
|stopPrice|Number|||
|profitPrice|Number|||

##### Example
##### New order
|New Order Input|Streaming Output|
|---|---|
|{|{|
|<br>requestID: "30304045",|<br>"type":"orderEvent",|
|instrumentID: "VN30F2106",|"data": {|
|market:"VNFE",|"orderID":"12663204",|
buySell: "B", "notifyID":10, orderType: "LO", “instrumentID":"VN30F2106", channelID: "WT", "uniqueID":"30304045", price: 1410, "buySell":"B", quantity: 10, “orderType":"LO", account: "0901358", "ipAddress":"192.168.202.36", stopOrder: false, "price":1410,"prefix":"2mw", stopPrice: 800, "quantity":10, stopType: "D", "marketID":"VNFE", stopStep: 0.5, "origOrderId":"", lossStep: 0, "account":"0901358", profitStep: 0, "cancelQty":0,"osQty":10, code:”123456789” "filledQty":0, } "avgPrice":0, "channel":"TA", "inputTime":"1606277281849", "modifiedTime":"1606277281850", "isForceSell":"F", "isShortSell":"F", "orderStatus":"RS", "rejectReason":"", "origRequestID":"30304045", "stopOrder":false, "stopPrice":0, "stopType":"", "stopStep":0, "profitPrice":0 } }

##### Case newOrder has error:
{"type":"orderError","data":{"message":"Price 1,600.00 exceeds ceiling price

808.50!","notifyID":15455,"data":null,"errorCode":"ERR500","uniqueID":"02365132","connectionID":"","ipAdd ress":"192.168.202.87","prefix":"23o"}}

##### Modify Order
|Modify Input|Streaming Output|
|---|---|
|{<br>requestID: "31618366",<br>orderID: "12663204",<br>price: 1410,<br>quantity: 2,<br>account: "0901358",<br>instrumentID: "VN30F2106",<br>marketID: "VNFE",<br>buySell: "B",<br>orderType: "LO"<br>code: “123456789”<br>}|{<br>"type":"orderEvent",<br>"data": {<br>"orderID":"12663204",<br>"notifyID":11,<br>"instrumentID":"VN30F2106",<br>"uniqueID":"31618366",<br>"buySell":"B",<br>"orderType":"LO",<br>"ipAddress":"192.168.202.36",<br>"price":1410,<br>"prefix":"2mw",<br>"quantity":2,|
|}|"marketID":"VNFE",<br>"origOrderId":"",<br>"account":"0901358",<br>"cancelQty":0,<br>"osQty":2,<br>"filledQty":0,<br>"avgPrice":0,<br>"channel":"TA",<br>"inputTime":"1606277281849",<br>"modifiedTime":"1606277330852",<br>"isForceSell":"F",<br>"isShortSell":"F",<br>"orderStatus":"RS",<br>"rejectReason":"",<br>"origRequestID":"30304045",<br>"stopOrder": false,<br>"stopPrice":0,<br>"stopType":"",<br>"stopStep":0,<br>"profitPrice":0<br>}|
|---|---|

##### Case modifyOrder  has error:
{"type":"orderError","data":{"message":"Price 1,000.00 exceeds ceiling price

808.50!","notifyID":15460,"data":null,"errorCode":"ERR500","uniqueID":"65896571","connectionID":"","ipAdd ress":"192.168.202.87","prefix":"23o"}}

##### Cancel Order
|Cancel Input|Streaming Output|
|---|---|
|{<br>requestID: "59028516",<br>orderID: "12663204",<br>account: "0901358",<br>marketID: "VNFE",<br>instrumentID: "VN30F2106",<br>buySell: "B",<br>requestID: "59028516"<br>code: “123456789”<br>}|{<br>"type":"orderEvent",<br>"data": {<br>"orderID":"12663204",<br>"notifyID":12,<br>"instrumentID":"VN30F2106",<br>"uniqueID":"59028516",<br>"buySell":"B",<br>"orderType":"LO",<br>"ipAddress":"192.168.202.36",<br>"price":1410<br>,"prefix":"2mw",<br>"quantity":2,<br>"marketID":"VNFE",<br>"origOrderId":"",<br>"account":"0901358",<br>"cancelQty":2,<br>"osQty":0,<br>"filledQty":0,<br>"avgPrice":0,|
<!-- Start of picture text -->
"channel":"TA",<br>"inputTime":"1606277281849",<br>"modifiedTime":"1606277330861",<br>"isForceSell":"F",<br>"isShortSell":"F",<br>"orderStatus":"CL",<br>"rejectReason":"",<br>"origRequestID":"30304045",<br>"stopOrder": false,<br>"stopPrice":0,<br>"stopType":"",<br>"stopStep":0,<br>"profitPrice":0<br>}<br>}<br>

Case cancel has error:

{"type":"orderError","data":{"message":"Invalid Order Transition Error!","notifyID":15468,"data":null,"errorCode":"ERR500","uniqueID":"25162310",ipAddress":"192.168.202. 87","prefix":"23o"}}

#### 6.1.2 Order Error
|**Name of Element**|**Type**|**Description**|**Valid Value or Format**|
|---|---|---|---|
|type|string|type of event|orderError|
|data||Information of order||
|Detail of data||||
|uniqueid|String|Original request ID||
|ipaddress|String|To detect the request from<br>client to TAPI||
|notifyID|String|The serial number of msg||
|errorCode|String|Mã code lỗi trả ra||
|message|String|Thông tin lệnh lỗi||
|orderID|String|Số hiệu lệnh||
|instrumentID|String|Mã CK||
|buySell|String|Chiều mua/ bán|B: Buy<br>S: Sell|
|prefix|String|||
|orderType|String|Loại lệnh|-<br>LO<br>-<br>ATO<br>-<br>ATC<br>-<br>MP|
||||-<br>MTL<br>-<br>MOK<br>-<br>MAK<br>-<br>PLO|
|---|---|---|---|
|price|Number|Giá||
|quantity|Number|Khối lượng||
|marketID|String|Sàn|VN<br>VNFE|
|origOrderID|String|||
|account|String|Số tài khoản||
|channel|String|Kênh||
|inputTime|String|Thời gian nhận||
|modifiedTime|String|Thời gian sửa lệnh||
|isForceSell|Boolean|Luôn = F||
|isShortSell|Boolean|Luôn = F||
|origRequestID|String|||
|stopOrder|String|True hoặc False||
|stopPrice|number|||
|stopType|String|Nếu là true thì điền 1 trong list|-D: Down<br>-U: Up<br>-V: Trailling Up<br>-E: Trailing Down<br>-B: BullBear<br>-O: OCO|
|stopStep|number|||
|profitPrice|number|||
|modifiable|String|||
|note|String|Ghi chú||

Example:

- { "type": "orderError", "data": { "message": "This channel has been block; disallow to place order ", "notifyID": 0, "errorCode": "ORD015", "uniqueID": "6163422", "orderID": "T20230504w3806163422", "instrumentID": "SSI", "ipAddress": "10.255.241.47", "buySell": "B", "prefix": "w38", "orderType": "LO", "price": 19600, "quantity": 200, "marketID": "VN", "origOrderId": "T20230504w3806163422", "account": "0322206", "channel": "TA", "inputTime": "1683165600160", "modifiedTime": "1683165600161",
"isForceSell": "F", "isShortSell": "F", "origRequestID": "6163422", "stopOrder": false, "stopPrice": 0, "stopType": "", "stopStep": 0, "profitPrice": 0, "modifiable": false, "note": "" } }

#### 6.1.3 Order Match Event
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|type|string|type of event||
|data||Information of matched order||
|Detail of data||||
|orderID|String|order ID||
|instrumentID|String|Stock symbol||
|ipAddress|String|To detect the request from<br>client to TAPI||
|uniqueID||Original request ID||
|notifyid|String|The serial number of msg||
|buySell|String||-B: Buy<br>-S: Sell|
|matchPrice|Number|||
|matchQty|Number|||
|prefix|String|||
|account|String|||
|matchTime|String|||

##### Example
orderMatchEvent:

{"type":"orderMatchEvent","data":{"orderID":"16201867","notifyID":101180,"instrumentID":"BVS","uniqueID": "24194396","buySell":"B","matchPrice":1000,"ipAddress":"10.48.41.16","matchQty":100,"prefix":"t4c","accou nt":"1184411","matchTime":"1656665019000"}}

### 6.2 Porfolio Streaming
To use the porfolio stream, the client needs to init the stream and then bind to subcribe the update

client.bind(client.events.onClientPortfolioEvent,function(e,data){ Process data... console.log(e + ": "); console.log(JSON.stringify(data)); })

Porfolio Event Response
|Name of Element|Type|Description|Valid Value or Format|
|---|---|---|---|
|type|String|Type of event||
|uniqueid|String|||
|connectionid|String|||
|ipaddress|String|To detect the request<br>from client to TAPI||
|notifyid|String|Id of message||
|account||accountid||
|marketid|String||VNFE|
|instrumentid|String|Stock symbol||
|longqty|number|Long position||
|shortqty|number|Short position||
|Net|number|Net position||
|bidavgprice|number|Average bid price||
|askavgprice|number|Average ask  price||
|tradeprice|number|Trade price||
|marketprice|number|Maket price||
|floatingpl|number|Temporarily calculated<br>profit and loss||
|tradingpl|number|Calculated profit and<br>loss||

##### Example
{"type":"clientPortfolioEvent","data":{"account":"0901358","notifyID":27,"data":null,"clientPortfoliosOpen":[{"m artketID":"VNFE","instrumentID":"VN30F2106","longQty":9,"shortQty":0,"net":9,"bidAvgPrice":1402.4000244 140625,"askAvgPrice":0,"tradePrice":0,"marketPrice":873,"floatingPL":-

476460000,"tradingPL":0},{"martketID":"VNFE","instrumentID":"VN30F2107","longQty":2,"shortQty":0,"net": 2,"bidAvgPrice":830,"askAvgPrice":0,"tradePrice":0,"marketPrice":830,"floatingPL":0,"tradingPL":0}],"unique ID":null,"clientPortfoliosClose":null,"connectionID":"","ipAddress":null,"prefix":null}}

## 7 Appendix
### 7.1 Error Code
|№.|ErrorCode|Message|Message (ENG)|Case|
|---|---|---|---|---|
|1.|ERR001|Invalid login ID or password|Wrong login information|Login|
|2.|ERR001|Invalid login ID or password|Wrong login information|Login|
|3.|ORD001|Security ticker does not<br>exist.|The stock code does not<br>exist|Set, Cancel, Edit<br>commands|
|4.|ORD002|Price is under floor level|The price is outside the<br>floor ceiling. The price is<br>less than the floor price|Set, Edit commands|
|5.|ORD003|Price exceeds ceiling level.|The price is outside the<br>floor ceiling. Price exceeds<br>the ceiling price|Set, Edit commands|
|6.|ORD004|Invalid Price Unit (Spread)|Wrong price step|Set, Edit commands|
|7.|ORD005|Invalid trading lot/block|Wrong batch of<br>transactions|Set, Edit commands|
|8.|ORD006|Invalid parameters|Lack of command<br>information|Set, Cancel, Edit<br>commands|
|9.|ORD007|Quantity exceeds the<br>allowance|Volume exceeding the<br>allowed stock balance<br>traded by the account|Set, Edit commands|
|10.|ORD008|Total quantity exceeds limit|Volumes exceeding the<br>volume allowed to trade|Set, Edit commands|
|11.|ORD008|Not enough purchasing<br>power|Insufficient purchasing<br>power|Set, Edit commands|
|12.|ORD009|Already exist B/S order of<br>same stock|This code cannot be placed<br>due to the existence of a<br>Sell/Buy order with an<br>unediable code.|Place a command|
|13.|ORD016|\<orderType\> is not allowed<br>in this session|Command \<orderType\><br>not set in current session|Set, Cancel, Edit<br>commands|
|14.|ORD017|This stock is suspended or<br>terminated|Stock code suspended<br>from trading|Place a command|
|15.|ORD011|Cannot be amended in this<br>session|Order not to be cancelled|Cancel, Edit<br>command|
|16.|ORD012|This order cannot be<br>modified|Orders cannot be cancelled<br>or edited|Cancel, Edit<br>command|
|17.|ORD013|Order Is Null Error!|Command number does<br>not exist|Cancel, Edit<br>command|
|18.|ORD014|Price and Quantity have no<br>changes|Prices and volumes<br>unchanged|Edit command|
|19.|ORD018|Odd lot is not allowed|Do not place odd lot<br>commands|Place a command|
|20.|ERR002|Duplicate Login Session<br>error|Wrong session||
|21.|ORD015|This channel has been<br>blocked; disallow to place<br>order|Ordering is not supported<br>on the current channel|Place a command|
|22.|ORD010|Invalid Order Type|The type of command that<br>does not allow execution|Cancel,  Edit<br>commands|
|23.|400|BadRequest||Transmission of<br>missing information<br>required when<br>placing, canceling,<br>correcting<br>commands (catching<br>errors at api,<br>depending on the<br>case will pay specific<br>errors)|
|24.|401|Unauthorized||An error occurred in<br>relation to access|
|25.|500|InternalServerError||Server error|
|26.|ORD027|Client cannot execute this<br>order|The account is not allowed<br>to make this order||
|27.|ORD026|Client status not allowed to<br>trade|The account is not allowed<br>to trade||
|28.|ORD023|System receive duplicated<br>price|Duplicate bid price||
|29.|ORD025|The trading account not<br>opened yet|Uns opened trading<br>account||
|30.|ORD031|Reduce qty more than<br>outstanding qty|The set volume must be<br>less than the remaining<br>volume||
|31.|ORD034|Exceed foreign room|Excess foreign room||
|32.|ORD036|Not enough current room|Insufficient room ordering||
|33.|ORD037|Exceed stock room|Excess room||
|34.|ORD038|Stock out of margin room|Excess room||
|35.|ORD039|DR > BFDL and insufficient<br>Room|||
|36.|ORD040|Not enough bal|Insufficient balance|
|37.|204|No such client information||
|38.|429|API calls quota exceeded!<br>maximum admitted x per y.||

### 7.2 Order Status
|№.|Status<br>Code|Status Name|Status Name (VIE)|
|---|---|---|---|
|1.|Wa|Waiting Approval|Chờ duyệt|
|2.|Rs|Ready to Send Exch|Chờ gửi lên sàn|
|3.|Sd|Sent to Exch|Đang gửi lên sàn|
|4.|Qu|Queue in Exch|Chờ khớp tại sàn|
|5.|Ff|Fully Filled|Khớp toàn phần|
|6.|Pf|Partially Filled|Khớp một phần|
|7.|FFPC|Fully Filled Partially Cancelled|Khớp 1 phần hủy phần còn lại|
|8.|Wm|Waiting Modify|Chờ sửa|
|9.|WC|Waiting Cancel|Chờ hủy|
|10.|Cl|Cancelled|Đã hủy|
|11.|Rj|Rejected|Từ chối|
|12.|Ex|Expired|Hết hiệu lực|
|13.|Sor|Stop Order Ready|Chờ kích hoạt|
|14.|Sos|Stop Order Sent|Đã kích hoạt|
|15.|IAV|Pre-Session Order|Lệnh trước phiên|
|16.|Soi|Pre-Session Stop Order|Lệnh ĐK trước phiên|

### 7.3 Channel
|№.|Channel Code|Name|Name (VIE)|
|---|---|---|---|
|1.|WT|Web Trading|Web Trading|
|2.|Ma|Mobile|Mobile|
|3.|Br|Broker|Broker|
|4.|IW|iBoard Web|iBoard Web|
|5.|IM|iBoard Mobile|iBoard Mobile|
|6.|I'm not<br>going|TradeAPI|TradeAPI|
|7.|VT|ProTrading|ProTrading|

### 7.4 Order Type
|№.|OrderType|Name|Name (VIE)|
|---|---|---|---|
|1.|LO|Limit Order|LO|
|2.|ATO|At The Opening|ATO|
|3.|ATC|At The Closing|ATC|
|4.|MP|Market Order (HOSE)|MP|
|5.|MTL|Market Order|MTL|
|6.|MOK|Match Or Kill|MOK|
|7.|MAK|Match And Kill|MAK|
|8.|PLO|Plo|PLO|
|9.|GTD|Good Till Date|GTD|
