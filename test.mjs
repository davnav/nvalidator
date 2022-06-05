
import { isValidCard,isJson,isCurrency,isPhonenumber,isUpc,isUSzipCode,isPostalCode } from './index.js'
const card_number = "5236313877109142";

const obj = 
{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
};
const st1 = 
'{ ' +' \"code\"' + ' 200' + '}';



console.log( isValidCard(card_number));
console.log( isJson(JSON.stringify(obj)));
console.log( isJson(st1));
console.log( isCurrency('AAA'));
console.log( isCurrency('INR'));

console.log(isPhonenumber("9446528807","IN"));

console.log(isUpc("123456789012"));
console.log(isUpc("123456789011"));
console.log(isUSzipCode("10465"));
console.log(isUSzipCode("V5B 0A1"));
console.log(isPostalCode("V5B 0A1","CA"));
console.log(isPostalCode("12234","AT"));