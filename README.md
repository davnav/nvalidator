# nvalidator

napi addons for validator functions for Node.js applications using Rust. inspired from https://github.com/validatorjs
The napi s functions are built using a framework called https://napi.rs/  


**Folder Structure:**
The Rust napi functions will be in the src folder.
test.mjs is the file used to test the exported functions.

**Use Rust addon functions in node**

Your node program would something like _test.mjs_ . You can import the rust made functions into the file using import as shown below.

```javascript
    
    import { isValidCard,isJson,isCurrency,isPhonenumber } from './index.js'
        
    const card_number = "5236313877109142";
    console.log( isValidCard(card_number));
```


**Building :**

1. clone the project - https://github.com/davnav/nvalidator.git
2. install napi as per the doc - https://napi.rs/docs/introduction/getting-started
3. yarn install
4. yarn build or napi build


**Contributing**
1. create pull request and add new validator functions in the _src_ folder 
2. If you add a new _rust_ file ( eg _card.rs_) in the _src_ folder and add the module in _lib.rs_, the napi build generate respective addon functions to node
3. _nvalidator.node_ dynamic linking file will have the generated functions.
4. you can test the functions using importing the functions in the test.mjs file and run _node test.mjs_ 
