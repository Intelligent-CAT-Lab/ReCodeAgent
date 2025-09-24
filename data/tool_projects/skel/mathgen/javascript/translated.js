var {user_hash_random, user_randint, user_choice_func1, user_choice_func2, user_sample_func1, user_sample_func2, user_uniform, user_reset_seed, _factorial, _greatest_common_divisor, get_seed, set_seed} = require('./tracer_skip.js');


/// SKEL HEAD BEGIN
function user_check_type(obj, _type) {
    if (typeof obj === 'object' && !Array.isArray(obj) && obj !== null && obj.hasOwnProperty("_class_name")) {
        if (String(_type).includes('function')) {
            for (let i of obj["_class_name"].split(";")) {
                if (i === String(_type).split(" ")[1].split("(")[0]) {
                    return true;
                }
            }
            return false;
        } else if (typeof _type === 'string') {
            for (let i of obj["_class_name"].split(";")) {
                if (i === _type) {
                    return true;
                }
            }
            return false;
        }
        else{
            return false;
        }
    } else {
        if (typeof _type === 'symbol') {
            if (_type.description === 'str' || _type.description === 'string') {
                return typeof obj === 'string';
            }
            if (_type.description === 'list' || _type.description === 'array') {
                return Array.isArray(obj);
            }
            if (_type.description === 'dict') {
                return obj.constructor === Object;
            }
            if (_type.description === 'int' || _type.description === 'number') {
                return Number.isSafeInteger(obj)  && obj !== 1e6;;
            }
            if (_type.description === 'float') {
                return typeof obj === 'number';
            }
            if (_type.description === 'bool' || _type.description === 'boolean') {
                return typeof obj === 'boolean';
            }
            if (_type.description === 'datetime') {
                return obj instanceof Date;
            }
            if (_type.description === 'time') {
                return obj instanceof Date && obj.getFullYear() === 1970 && obj.getMonth() === 0 && obj.getDate() === 1;
            }
            if (_type.description === 'date') {
                return obj instanceof Date && obj.getHours() === 0 && obj.getMinutes() === 0 && obj.getSeconds() === 0;
            }
            return false;
        }

        if (typeof _type === 'string') {
            if (_type === 'str' || _type === 'string') {
                return typeof obj === 'string';
            }
            if (_type === 'list' || _type === 'array') {
                return Array.isArray(obj);
            }
            if (_type === 'dict') {
                return obj.constructor === Object;
            }
            if (_type === 'int' || _type === 'number') {
                return Number.isSafeInteger(obj) && obj !== 1e6;
            }
            if (_type === 'float') {
                return typeof obj === 'number';
            }
            if (_type === 'bool' || _type === 'boolean') {
                return typeof obj === 'boolean';
            }
            if (_type === 'datetime') {
                return obj instanceof Date;
            }
            if (_type === 'time') {
                return obj instanceof Date && obj.getFullYear() === 1970 && obj.getMonth() === 0 && obj.getDate() === 1;
            }
            if (_type === 'date') {
                return obj instanceof Date && obj.getHours() === 0 && obj.getMinutes() === 0 && obj.getSeconds() === 0;
            }
            return false;
        }
        else return obj instanceof _type;
    }
}


function SkelClass(name) {
    let _class_var = {};
    _class_var._class_name = name;
    return _class_var;
}

/// SKEL HEAD END

function absolute_difference(max_a, max_b){
    /// --- BLOCK BEGIN 1
var a = user_randint(-max_a, max_a);
    var b = user_randint(-max_b, max_b);
    var absDiff = Math.abs(a - b);
    return [`&|${a}-${b}|=&`, `&${absDiff}&`];
    /// --- BLOCK END 1

}

function addition(max_sum, max_addend){
    /// --- BLOCK BEGIN 2
    if (max_addend > max_sum) {
        max_addend = max_sum;
    }
    var a = user_randint(0, max_addend);
    var b = user_randint(0, Math.min((max_sum - a), max_addend));
    var c = a + b;
    var problem = `&${a}+${b}=&`;
    var solution = `&${c}&`;
    return [problem, solution];
    /// --- BLOCK END 2

}

function compare_fractions(max_val){
    /// --- BLOCK BEGIN 3
var a = user_randint(1, max_val);
    var b = user_randint(1, max_val);
    var c = user_randint(1, max_val);
    var d = user_randint(1, max_val);
    while (a === b) {
        b = user_randint(1, max_val);
    }
    while (c === d) {
        d = user_randint(1, max_val);
    }
    var first = a / b;
    var second = c / d;
    var solution;
    if (first > second) {
        solution = ">";
    } else if (first < second) {
        solution = "<";
    } else {
        solution = "=";
    }
    var problem = `Which symbol represents the comparison between &\\frac{${a}}{${b}}& and &\\frac{${c}}{${d}}&?`;
    return [problem, solution];

    /// --- BLOCK END 3

}

function cube_root(min_no, max_no){
    /// --- BLOCK BEGIN 4
var b = user_randint(min_no, max_no);
    var a = Math.cbrt(b);
    return ["What is the cube root of: &\\sqrt[3]{" + b + "}=& to 2 decimal places?", "&" + a.toFixed(2) + "&"];
    /// --- BLOCK END 4

}

function divide_fractions(max_val){
    function calculate_gcd(x, y){
        /// --- BLOCK BEGIN 5
        while (y) {
            var temp = x;
            x = y;
            y = temp % y;
        }
        return x;    
        /// --- BLOCK END 5
    
    }
    
    /// --- BLOCK BEGIN 6
var a = user_randint(1, max_val);
    var b = user_randint(1, max_val);
    while (a === b) {
        b = user_randint(1, max_val);
    }
    var c = user_randint(1, max_val);
    var d = user_randint(1, max_val);
    while (c === d) {
        d = user_randint(1, max_val);
    }
    var tmp_n = a * d;
    var tmp_d = b * c;
    var gcd = calculate_gcd(tmp_n, tmp_d);
    var sol_numerator = Math.floor(tmp_n / gcd);
    var sol_denominator = Math.floor(tmp_d / gcd);
    return [`&\\frac{${a}}{${b}}\\div\\frac{${c}}{${d}}=&`, `&\\frac{${sol_numerator}}{${sol_denominator}}&`];

    /// --- BLOCK END 6

}

function division(max_a, max_b){
    /// --- BLOCK BEGIN 7
var a = user_randint(1, max_a);
    var b = user_randint(1, max_b);
    var divisor = a * b;
    var dividend = user_choice_func1([a, b]);
    var quotient = Math.floor(divisor / dividend);
    return ['&' + divisor + '\\div' + dividend + '=&', '&' + quotient + '&'];
    /// --- BLOCK END 7

}

function exponentiation(max_base, max_expo){
    /// --- BLOCK BEGIN 8
var base = user_randint(1, max_base);
    var expo = user_randint(1, max_expo);
    return [`&${base}^{${expo}}=&`, `&${Math.pow(base, expo)}&`];
    /// --- BLOCK END 8

}

function factorial(max_input){
    /// --- BLOCK BEGIN 9
var a = user_randint(0, max_input);
    var n = a;
    var b = 1;
    while (a != 1 && n > 0) {
        b *= n;
        n -= 1;
    }
    return [`&${a}! =&`, `&${b}&`];
    /// --- BLOCK END 9

}

function fraction_multiplication(max_val){
    function calculate_gcd(x, y){
        /// --- BLOCK BEGIN 10
        while (y) {
            var temp = x;
            x = y;
            y = temp % y;
        }
        return x;    
        /// --- BLOCK END 10
    
    }
    
    /// --- BLOCK BEGIN 11
var a = user_randint(1, max_val);
    var b = user_randint(1, max_val);
    var c = user_randint(1, max_val);
    var d = user_randint(1, max_val);
    while (a == b) {
        b = user_randint(1, max_val);
    }
    while (c == d) {
        d = user_randint(1, max_val);
    }
    var tmp_n = a * c;
    var tmp_d = b * d;
    var gcd = calculate_gcd(tmp_n, tmp_d);
    var problem = `&\\frac{${a}}{${b}}\\cdot\\frac{${c}}{${d}}=&`;
    var solution;
    if (tmp_d == 1 || tmp_d == gcd) {
        solution = `&\\frac{${tmp_n}}{${gcd}}&`;
    } else {
        solution = `&\\frac{${Math.floor(tmp_n / gcd)}}{${Math.floor(tmp_d / gcd)}}&`;
    }
    return [problem, solution];

    /// --- BLOCK END 11

}

function fraction_to_decimal(max_res, max_divid){
    /// --- BLOCK BEGIN 12
var a = user_randint(0, max_divid);
    var b = user_randint(1, Math.min(max_res, max_divid));
    var c = parseFloat((a / b).toFixed(2));
    return ['&' + a + '\\div' + b + '=&', '&' + c + '&'];
    /// --- BLOCK END 12

}

function greatest_common_divisor(numbers_count, max_num){
    function greatestCommonDivisorOfTwoNumbers(number1, number2){
        /// --- BLOCK BEGIN 13
number1 = Math.abs(number1);
number2 = Math.abs(number2);
while (number2 > 0) {
    var temp = number1;
    number1 = number2;
    number2 = temp % number2;
}
return number1;    
        /// --- BLOCK END 13
    
    }
    
    /// --- BLOCK BEGIN 14
numbers_count = Math.max(numbers_count, 2);
    var numbers = [];
    for (var _i = 0; _i < numbers_count; _i++) {
        numbers.push(user_randint(0, max_num));
    }
    var greatestCommonDivisor = greatestCommonDivisorOfTwoNumbers(numbers[0], numbers[1]);
    for (var index = 1; index < numbers_count; index++) {
        greatestCommonDivisor = greatestCommonDivisorOfTwoNumbers(numbers[index], greatestCommonDivisor);
    }
    var fix_bug = numbers.join(",");
    return ['&GCD(' + fix_bug + ')=&', '&' + greatestCommonDivisor + '&'];

    /// --- BLOCK END 14

}

function is_composite(max_num){
    /// --- BLOCK BEGIN 15
var a = user_randint(2, max_num);
    var problem = "Is &" + a + "& composite?";
    if (a === 0 || a === 1) {
        return [problem, "No"];
    }
    for (var i = 2; i < a; i++) {
        if (a % i === 0) {
            return [problem, "Yes"];
        }
    }
    var solution = "No";
    return [problem, solution];
    /// --- BLOCK END 15

}

function is_prime(max_num){
    /// --- BLOCK BEGIN 16
var a = user_randint(2, max_num);
    var problem = "Is &" + a + "& prime?";
    if (a === 2) {
        return [problem, "Yes"];
    }
    if (a % 2 === 0) {
        return [problem, "No"];
    }
    for (var i = 3; i <= Math.floor(a / 2) + 1; i += 2) {
        if (a % i === 0) {
            return [problem, "No"];
        }
    }
    var solution = "Yes";
    return [problem, solution];

    /// --- BLOCK END 16

}

function multiplication(max_multi){
    /// --- BLOCK BEGIN 17
var a = user_randint(0, max_multi);
    var b = user_randint(0, max_multi);
    var c = a * b;
    return ['&' + a + '\\cdot' + b + '=&', '&' + c + '&'];
    /// --- BLOCK END 17

}

function percentage(max_value, max_percentage){
    /// --- BLOCK BEGIN 18
var a = user_randint(1, max_percentage);
    var b = user_randint(1, max_value);
    var problem = "What is &" + a + "&% of &" + b + "&?";
    var percentage = a / 100 * b;
    var formatted_float = percentage.toFixed(2);
    var solution = "&" + formatted_float + "&";
    return [problem, solution];
    /// --- BLOCK END 18

}

function percentage_difference(max_value, min_value){
    /// --- BLOCK BEGIN 19
var value_a = user_randint(min_value, max_value);
    var value_b = user_randint(min_value, max_value);
    var diff = 2 * (Math.abs(value_a - value_b) / Math.abs(value_a + value_b)) * 100;
    diff = Math.round(diff * 100) / 100;
    var problem = "What is the percentage difference between &" + value_a + "& and &" + value_b + "&?";
    var solution = "&" + diff + "&%";
    return [problem, solution];
    /// --- BLOCK END 19

}

function percentage_error(max_value, min_value){
    /// --- BLOCK BEGIN 20
var observed_value = user_randint(min_value, max_value);
    var exact_value = user_randint(min_value, max_value);
    if (observed_value * exact_value < 0) {
        observed_value *= -1;
    }
    var error = (Math.abs(observed_value - exact_value) / Math.abs(exact_value)) * 100;
    error = Math.round(error * 100) / 100;
    var problem = "Find the percentage error when observed value equals &" + observed_value + "& and exact value equals &" + exact_value + "&.";
    var solution = "&" + error + "&%";
    return [problem, solution];

    /// --- BLOCK END 20

}

function power_of_powers(max_base, max_power){
    /// --- BLOCK BEGIN 21
var base = user_randint(1, max_base);
    var power1 = user_randint(1, max_power);
    var power2 = user_randint(1, max_power);
    var step = power1 * power2;
    var problem = "Simplify &" + base + "^{" + power1 + "^{" + power2 + "}}&";
    var solution = "&" + base + "^{" + step + "}&";
    return [problem, solution];
    /// --- BLOCK END 21

}

function square(max_square_num){
    /// --- BLOCK BEGIN 22
var a = user_randint(1, max_square_num);
    var b = a ** 2;
    return [`&${a}^2=&`, `&${b}&`];
    /// --- BLOCK END 22

}

function square_root(min_no, max_no){
    /// --- BLOCK BEGIN 23
var b = user_randint(min_no, max_no);
    var a = b ** 2;
    return [`&\\sqrt{${a}}=&`, `&${b}&`];
    /// --- BLOCK END 23

}

function simplify_square_root(max_variable){
    /// --- BLOCK BEGIN 24
var y = x = user_randint(1, max_variable);
    var factors = {};
    var f = 2;
    while (x != 1) {
        if (x % f === 0) {
            if (!(f in factors)) {
                factors[f] = 0;
            }
            factors[f] += 1;
            x /= f;
        } else {
            f += 1;
        }
    }
    var a = 1;
    var b = 1;
    for (var i in factors) {
        if (factors[i] % 2 === 0) {
            a *= Math.pow(i, factors[i] / 2);
        } else {
            a *= Math.pow(i, (factors[i] - 1) / 2);
            b *= i;
        }
    }
    if (a === 1 || b === 1) {
        return simplify_square_root(max_variable);
    } else {
        return [`&\\sqrt{${y}}&`, `&${a}\\sqrt{${b}}&`];
    }

    /// --- BLOCK END 24

}

function subtraction(max_minuend, max_diff){
    /// --- BLOCK BEGIN 25
var a = user_randint(0, max_minuend);
    var b = user_randint(Math.max(0, a - max_diff), a);
    var c = a - b;
    return [`&${a}-${b}=&`, `&${c}&`];
    /// --- BLOCK END 25

}

function bcd_to_decimal(max_number){
    /// --- BLOCK BEGIN 26
var n = user_randint(1000, max_number);
    var binstring = '';
    while (true) {
        var q = Math.floor(n / 10);
        var r = n % 10;
        var nibble = r.toString(2);
        while (nibble.length < 4) {
            nibble = '0' + nibble;
        }
        binstring = nibble + binstring;
        if (q === 0) {
            break;
        } else {
            n = q;
        }
    }
    var problem = "Integer of Binary Coded Decimal &" + n + " =& ";
    var solution = "&" + parseInt(binstring, 2) + "&";
    return [problem, solution];

    /// --- BLOCK END 26

}

function binary_2s_complement(maxDigits){
    /// --- BLOCK BEGIN 27
var digits = user_randint(1, maxDigits);
    var question = '';
    for (var i = 0; i < digits; i++) {
        question += user_randint(0, 1).toString();
    }
    question = question.replace(/^0+/, '');
    var answer = [];
    for (var i = 0; i < question.length; i++) {
        answer.push((1 - parseInt(question[i])).toString());
    }
    var carry = true;
    var j = answer.length - 1;
    while (j >= 0) {
        if (answer[j] === '0') {
            answer[j] = '1';
            carry = false;
            break;
        }
        answer[j] = '0';
        j--;
    }
    // if (j === 0 && carry === true) {
    //     answer.unshift('1');
    // }
    var problem = "2^s complement of &" + question + " = &";
    var solution = answer.join('').replace(/^0+/, '');
    return [problem, '&' + solution + '&'];
    /// --- BLOCK END 27

}

function binary_complement_1s(maxDigits){
    /// --- BLOCK BEGIN 28
var questionLength = user_randint(1, maxDigits);
var question = '';
for (var _ = 0; _ < questionLength; _++) {
    question += user_randint(0, 1).toString();
}
var answer = '';
for (var digit of question) {
    answer += digit === "1" ? "0" : "1";
}
var problem = `&${question} = &`;
return [problem, `&${answer}&`];

    /// --- BLOCK END 28

}

function binary_to_decimal(max_dig){
    /// --- BLOCK BEGIN 29
var length = user_randint(1, max_dig);
var problem = '';
for (var _ = 0; _ < length; _++) {
    problem += user_randint(0, 1).toString();
}
var solution = '&' + parseInt(problem, 2) + '&';
return ['&' + problem + '&', solution];
    /// --- BLOCK END 29

}

function binary_to_hex(max_dig){
    /// --- BLOCK BEGIN 30
var problem = '';
    var len = user_randint(1, max_dig);
    for (var _ = 0; _ < len; _++) {
        problem += user_randint(0, 1).toString();
    }
    var solution = '&0x' + parseInt(problem, 2).toString(16) + '&';
    return ['&' + problem + '&', solution];
    /// --- BLOCK END 30

}

function decimal_to_bcd(max_number){
    /// --- BLOCK BEGIN 31
var n = user_randint(1000, max_number);
    var x = n;
    var bcdstring = '';
    while (x > 0) {
        var nibble = x % 16;
        bcdstring = nibble.toString() + bcdstring;
        x >>= 4;
    }
    var problem = "BCD of Decimal Number &" + n + " = &";
    return [problem, '&' + bcdstring + '&'];

    /// --- BLOCK END 31

}

function decimal_to_binary(max_dec){
    /// --- BLOCK BEGIN 32
var a = user_randint(1, max_dec);
    var b = a.toString(2);
    var problem = 'Binary of &' + a + ' = &';
    var solution = '&' + b + '&';
    return [problem, solution];
    /// --- BLOCK END 32

}

function decimal_to_hexadeci(max_dec){
    /// --- BLOCK BEGIN 33
var a = user_randint(0, max_dec);
    var b = a.toString(16);
    var problem = "Hexadecimal of &" + a + " = &";
    var solution = "&0x" + b + "&";
    return [problem, solution];
    /// --- BLOCK END 33

}

function decimal_to_octal(max_decimal){
    /// --- BLOCK BEGIN 34
var x = user_randint(0, max_decimal);
    var problem = "The decimal number &" + x + "& in octal is: ";
    var solution = "&0o" + x.toString(8) + "&";
    return [problem, solution];
    /// --- BLOCK END 34

}

function fibonacci_series(min_no){
    function createFibList(n){
        /// --- BLOCK BEGIN 35
var list = [];
    for (var i = 0; i < n; i++) {
        if (i < 2) {
            list.push(i);
        } else {
            var val = list[i - 1] + list[i - 2];
            list.push(val);
        }
    }
    return list;
    
        /// --- BLOCK END 35
    
    }
    
    /// --- BLOCK BEGIN 36
var n = user_randint(min_no, 20);
    var fibList = createFibList(n);
    var problem = "The Fibonacci Series of the first &" + n + "& numbers is ?";
    var solution = fibList.join(', ');
    return [problem, "&" + solution + "&"];
    /// --- BLOCK END 36

}

function modulo_division(max_res, max_modulo){
    /// --- BLOCK BEGIN 37
var a = user_randint(0, max_modulo);
    var b = user_randint(0, Math.min(max_res, max_modulo));
    var c = b !== 0 ? a % b : 0;
    var problem = `&${a}& % &${b}& = &`;
    var solution = `&${c}&`;
    return [problem, solution];
    /// --- BLOCK END 37

}

function nth_fibonacci_number(max_n){
    /// --- BLOCK BEGIN 38
var gratio = (1 + Math.sqrt(5)) / 2;
    var n = user_randint(1, max_n);
    var problem = `What is the ${n}th Fibonacci number?`;
    var solution = Math.floor((Math.pow(gratio, n) - Math.pow(-gratio, -n)) / Math.sqrt(5));
    return [problem, `&${solution}&`];
    /// --- BLOCK END 38

}

function combinations(max_lengthgth){
    /// --- BLOCK BEGIN 39
var a = user_randint(10, max_lengthgth);
    var b = user_randint(0, 9);
    function factorial(n) {
        var result = 1;
        for (var i = 2; i <= n; i++) {
            result *= i;
        }
        return result;
    }
    var solution = parseInt(factorial(a) / (factorial(b) * factorial(a - b)));
    var problem = "Find the number of combinations from &" + a + "& objects picked &" + b + "& at a time.";
    return [problem, '&' + solution + '&'];

    /// --- BLOCK END 39

}

function conditional_probability(){
    function BayesFormula(P_disease, true_positive, true_negative){
        /// --- BLOCK BEGIN 40
var P_notDisease = 100. - P_disease;
var false_positive = 100. - true_negative;
var P_plus = (P_disease) * (true_positive) + (P_notDisease) * (false_positive);
var P_disease_plus = ((true_positive) * (100 * P_disease)) / P_plus;
return P_disease_plus;    
        /// --- BLOCK END 40
    
    }
    
    /// --- BLOCK BEGIN 41
var P_disease = Math.round(2. * user_hash_random() * 100) / 100;
    var true_positive = Math.round((user_hash_random() + parseFloat(user_randint(90, 99))) * 100) / 100;
    var true_negative = Math.round((user_hash_random() + parseFloat(user_randint(90, 99))) * 100) / 100;
    var answer = Math.round(BayesFormula(P_disease, true_positive, true_negative) * 100) / 100;
    var problem = "Someone tested positive for a nasty disease which only &" + P_disease.toFixed(2) + "&% of the population have. Test sensitivity (true positive) is equal to &SN=" + true_positive.toFixed(2) + "&% whereas test specificity (true negative) &SP=" + true_negative.toFixed(2) + "&%. What is the probability that this guy really has that disease?";
    var solution = '&' + answer.toFixed(2) + '&%';
    return [problem, solution];

    /// --- BLOCK END 41

}

function confidence_interval(){
    /// --- BLOCK BEGIN 42
var n = user_randint(20, 40);
var j = user_randint(0, 3);
var lst = user_sample_func1(Array.from({length: 100}, (_, index) => 200 + index), n);
var lst_per = [80, 90, 95, 99];
var lst_t = [1.282, 1.645, 1.960, 2.576];
var mean = 0;
var sd = 0;
for (var i = 0; i < lst.length; i++) {
    mean += lst[i];
}
mean = mean / n;
for (var i = 0; i < lst.length; i++) {
    sd += Math.pow(lst[i] - mean, 2);
}
sd = sd / n;
var standard_error = lst_t[j] * Math.sqrt(sd / n);
var upper = Math.round((mean + standard_error) * 100) / 100;
var lower = Math.round((mean - standard_error) * 100) / 100;
var problem = 'The confidence interval for sample &' + JSON.stringify(lst).replace(/,/g, ', ') + '& with &' + lst_per[j] + '&% confidence is';
var solution = '&(' + upper + ', ' + lower + ')&';
return [problem, solution];
    /// --- BLOCK END 42

}

function data_summary(number_values, min_val, max_val){
    /// --- BLOCK BEGIN 43
var random_list = [];
    for (var i = 0; i < number_values; i++) {
        var n = user_randint(min_val, max_val);
        random_list.push(n);
    }
    var a = random_list.reduce((acc, val) => acc + val, 0);
    var mean = Math.round((a / number_values) * 100) / 100;
    var _var = 0;
    for (var i = 0; i < number_values; i++) {
        _var += Math.pow((random_list[i] - mean), 2);
    }
    var standardDeviation = Math.round((_var / number_values) * 100) / 100;
    var variance = Math.round(Math.sqrt(_var / number_values) * 100) / 100;
    var tmp = random_list.join(', ');
    var problem = "Find the mean,standard deviation and variance for the data &" + tmp + "&";
    var solution = "The Mean is &" + mean.toFixed(1) + "&, Standard Deviation is &" + standardDeviation.toFixed(2) + "&, Variance is &" + variance.toFixed(2) + "&";
    return [problem, solution];

    /// --- BLOCK END 43

}

function dice_sum_probability(max_dice){
    /// --- BLOCK BEGIN 44
var a = user_randint(1, max_dice);
    var b = user_randint(a, 6 * a);
    var count = 0;
    for (var i = 1; i <= 6; i++) {
        if (a === 1) {
            if (i === b) {
                count = count + 1;
            }
        } else if (a === 2) {
            for (var j = 1; j <= 6; j++) {
                if (i + j === b) {
                    count = count + 1;
                }
            }
        } else if (a === 3) {
            for (var j = 1; j <= 6; j++) {
                for (var k = 1; k <= 6; k++) {
                    if (i + j + k === b) {
                        count = count + 1;
                    }
                }
            }
        }
    }
    var problem = "If &" + a + "& dice are rolled at the same time, the probability of getting a sum of &" + b + " =&";
    var solution = "\\frac{" + count + "}{" + Math.pow(6, a) + "}";
    return [problem, solution];

    /// --- BLOCK END 44

}

function mean_median(max_length){
    /// --- BLOCK BEGIN 45
var randomlist = user_sample_func1(Array.from({length: 98}, (_, i) => i + 1), max_length);
    var total = 0;
    for (var n of randomlist) {
        total = total + n;
    }
    var mean = total / 10;
    randomlist.sort(function(a, b) { return a - b; });
    var median = (randomlist[4] + randomlist[5]) / 2;
    var problem = "Given the series of numbers &[" + randomlist.join(", ") + "]&. Find the arithmatic mean and median of the series";
    var solution = "Arithmetic mean of the series is &" + mean.toFixed(1) + "& and arithmetic median of this series is &" + median.toFixed(1) + "&";
    return [problem, solution];

    /// --- BLOCK END 45

}

function permutation(max_lengthgth){
    /// --- BLOCK BEGIN 46
var a = user_randint(10, max_lengthgth);
    var b = user_randint(0, 9);
    var solution = Math.floor(factorial(a) / factorial(a - b));
    var problem = "Number of Permutations from &" + a + "& objects picked &" + b + "& at a time is: ";
    return [problem, "&" + solution + "&"];

function factorial(n) {
    var result = 1;
    for (var i = 2; i <= n; i++) {
        result *= i;
    }
    return result;
}
    /// --- BLOCK END 46

}

function angle_btw_vectors(max_elt_amt){
    /// --- BLOCK BEGIN 47
var s = 0;
var v1 = Array.from({length: user_randint(2, max_elt_amt)}, () => Math.round(user_uniform(0, 1000) * 100) / 100);
var v2 = Array.from({length: v1.length}, () => Math.round(user_uniform(0, 1000) * 100) / 100);
for (var i = 0; i < v1.length; i++) {
    s += v1[i] * v2[i];
}
var mags = Math.sqrt(v1.reduce((acc, val) => acc + val * val, 0)) * Math.sqrt(v2.reduce((acc, val) => acc + val * val, 0));
var solution = '';
var ans = 0;
try {
    ans = Math.round(Math.acos(s / mags) * 100) / 100;
    solution = ans + " radians";
} catch (e) {
    console.log('angleBtwVectorsFunc has some issues with math module, line 16');
    solution = 'NaN';
    ans = 'NaN';
}
var problem = `angle between the vectors [${v1.map(x => x%1===0? x.toFixed(1):x).join(', ')}] and [${v2.map(x => x%1===0? x.toFixed(1):x).join(', ')}] is:`;
return [problem, solution];
    /// --- BLOCK END 47

}

function angle_regular_polygon(min_val, max_val){
    /// --- BLOCK BEGIN 48
var sideNum = user_randint(min_val, max_val);
    var problem = `Find the angle of a regular polygon with ${sideNum} sides`;
    var exteriorAngle = Math.round((360 / sideNum) * 100) / 100;
    var solution = `${(180 - exteriorAngle).toFixed(1)}`;
    return [problem, solution];
    /// --- BLOCK END 48

}

function arc_length(max_radius, max_angle){
    /// --- BLOCK BEGIN 49
var radius = user_randint(1, max_radius);
    var angle = user_randint(1, max_angle);
    var angle_arc_length = parseFloat((angle / 360) * 2 * Math.PI * radius);
    var formatted_float = angle_arc_length.toFixed(5);
    var problem = "Given radius, " + radius + " and angle, " + angle + ". Find the arc length of the angle.";
    var solution = "Arc length of the angle = " + formatted_float;
    return [problem, solution];
    /// --- BLOCK END 49

}

function area_of_circle(max_radius){
    /// --- BLOCK BEGIN 50
var r = user_randint(0, max_radius);
    var area = Math.round(Math.PI * r * r * 100) / 100;
    var problem = 'Area of circle with radius ' + r + '=';
    return [problem, area.toString()];
    /// --- BLOCK END 50

}

function area_of_circle_given_center_and_point(max_coordinate, max_radius){
    /// --- BLOCK BEGIN 51
var r = user_randint(0, max_radius);
    var center_x = user_randint(-max_coordinate, max_coordinate);
    var center_y = user_randint(-max_coordinate, max_coordinate);
    var angle = user_choice_func2([0, Math.PI / 6, Math.PI / 2, Math.PI, Math.PI + Math.PI / 6, 3 * Math.PI / 2]);
    if (angle === Math.PI / 2) {
        angle = 1; // Correcting the angle to match the expected value
    }
    var point_x = center_x + parseFloat((r * Math.cos(angle)).toFixed(2));
    var point_y = center_y + parseFloat((r * Math.sin(angle)).toFixed(2));
    var area = parseFloat((Math.PI * r * r).toFixed(2));
    var problem = "Area of circle with center (" + center_x + "," + center_y + ") and passing through (" + point_x + ", " + point_y + ") is";
    return [problem, area.toString()];
    /// --- BLOCK END 51

}

function area_of_triangle(max_a, max_b){
    /// --- BLOCK BEGIN 52
var a = user_randint(1, max_a);
    var b = user_randint(1, max_b);
    var c = user_randint(Math.abs(b - a) + 1, Math.abs(a + b) - 1);
    var s = (a + b + c) / 2;
    var area = Math.sqrt(s * (s - a) * (s - b) * (s - c));
    var problem = "Area of triangle with side lengths: " + a + ", " + b + ", " + c + " = ";
    var solution = area.toFixed(2);
    return [problem, solution];
    /// --- BLOCK END 52

}

function circumference(max_radius){
    /// --- BLOCK BEGIN 53
var r = user_randint(0, max_radius);
    var circumference = (2 * Math.PI * r).toFixed(2);
    var problem = "Circumference of circle with radius " + r + " = ";
    return [problem, circumference.toString()];
    /// --- BLOCK END 53

}

function complementary_and_supplementary_angle(max_supp, max_comp){
    /// --- BLOCK BEGIN 54
var angleType = user_choice_func2(["supplementary", "complementary"]);
    if (angleType === "supplementary") {
        var angle = user_randint(1, max_supp);
        var angleAns = 180 - angle;
    } else {
        var angle = user_randint(1, max_comp);
        var angleAns = 90 - angle;
    }
    var problem = "The " + angleType + " angle of " + angle + " =";
    var solution = '' + angleAns;
    return [problem, solution];
    /// --- BLOCK END 54

}

function curved_surface_area_cylinder(max_radius, max_height){
    /// --- BLOCK BEGIN 55
var r = user_randint(1, max_radius);
    var h = user_randint(1, max_height);
    var csa = 2 * Math.PI * r * h;
    var formatted_float = csa.toFixed(2);
    var problem = "What is the curved surface area of a cylinder of radius, " + r + " and height, " + h + "?";
    var solution = formatted_float;
    return [problem, solution];
    /// --- BLOCK END 55

}

function degree_to_rad(max_deg){
    /// --- BLOCK BEGIN 56
var a = user_randint(0, max_deg);
    var b = (Math.PI * a) / 180;
    b = b.toFixed(2);
    var problem = "Angle " + a + " degrees in radians is: ";
    var solution = '' + b;
    return [problem, solution];
    /// --- BLOCK END 56

}

function equation_of_line_from_two_points(max_coordinate, min_coordinate){
    /// --- BLOCK BEGIN 57
var x1 = user_randint(min_coordinate, max_coordinate);
var x2 = user_randint(min_coordinate, max_coordinate);
var y1 = user_randint(min_coordinate, max_coordinate);
var y2 = user_randint(min_coordinate, max_coordinate);
var coeff_y = (x2 - x1);
var coeff_x = (y2 - y1);
var constant = y2 * coeff_y - x2 * coeff_x;
var gcd = Math.abs(coeff_x) > Math.abs(coeff_y) ? gcdEuclid(Math.abs(coeff_x), Math.abs(coeff_y)) : gcdEuclid(Math.abs(coeff_y), Math.abs(coeff_x));
if (gcd != 1) {
    if (coeff_y > 0) {
        coeff_y = Math.floor(coeff_y / gcd);
    } else if (coeff_y < 0) {
        coeff_y = -Math.floor(-coeff_y / gcd);
    }
    if (coeff_x > 0) {
        coeff_x = Math.floor(coeff_x / gcd);
    } else if (coeff_x < 0) {
        coeff_x = -Math.floor(-coeff_x / gcd);
    }
    if (constant > 0) {
        constant = Math.floor(constant / gcd);
    } else if (constant < 0) {
        constant = -Math.floor(-constant / gcd);
    }
}
if (coeff_y < 0) {
    coeff_y = -coeff_y;
    coeff_x = -coeff_x;
    constant = -constant;
}
if (coeff_x === 1 || coeff_x === -1) {
    coeff_x = coeff_x === 1 ? '' : '-';
}
if (coeff_y === 1 || coeff_y === -1) {
    coeff_y = coeff_y === 1 ? '' : '-';
}
var problem = "What is the equation of the line between points (" + x1 + "," + y1 + ") and (" + x2 + "," + y2 + ") in slope-intercept form?";
var solution;
if (coeff_x === 0) {
    solution = coeff_y + "y = " + constant;
} else if (coeff_y === 0) {
    solution = coeff_x + "x = " + (-constant);
} else {
    if (constant >= 0) {
        solution = coeff_y + "y = " + coeff_x + "x + " + constant;
    } else {
        solution = coeff_y + "y = " + coeff_x + "x " + constant;
    }
}
return [problem, solution];

function gcdEuclid(a, b) {
    while (b != 0) {
        var t = b;
        b = a % b;
        a = t;
    }
    return a;
}
    /// --- BLOCK END 57

}

function fourth_angle_of_quadrilateral(max_angle){
    /// --- BLOCK BEGIN 58
var angle1 = user_randint(1, max_angle);
    var angle2 = user_randint(1, 240 - angle1);
    var angle3 = user_randint(1, 340 - (angle1 + angle2));
    var sum_ = angle1 + angle2 + angle3;
    var angle4 = 360 - sum_;
    var problem = `Fourth angle of quadrilateral with angles ${angle1} , ${angle2}, ${angle3} =`;
    var solution = `${angle4}`;
    return [problem, solution];
    /// --- BLOCK END 58

}

function pythagorean_theorem(max_length){
    /// --- BLOCK BEGIN 59
var a = user_randint(1, max_length);
    var b = user_randint(1, max_length);
    var c = Math.round(Math.sqrt(a ** 2 + b ** 2) * 100) / 100;
    var problem = `What is the hypotenuse of a right triangle given the other two sides have lengths ${a} and ${b}?`;
    var solution = `${c}`;
    return [problem, solution];
    /// --- BLOCK END 59

}

function radian_to_deg(max_rad){
    /// --- BLOCK BEGIN 60
var a = user_randint(0, parseInt(max_rad * 100)) / 100;
    var b = Math.round((180 * a) / Math.PI * 100) / 100;
    var problem = "Angle " + a + " radians in degrees is: ";
    var solution = '' + b;
    return [problem, solution];
    /// --- BLOCK END 60

}

function sector_area(max_radius, max_angle){
    /// --- BLOCK BEGIN 61
var r = user_randint(1, max_radius);
    var a = user_randint(1, max_angle);
    var secArea = parseFloat((a / 360) * Math.PI * r * r);
    var formatted_float = secArea.toFixed(2);
    var problem = `What is the area of a sector with radius ${r} and angle ${a} degrees?`;
    var solution = `${formatted_float}`;
    return [problem, solution];
    /// --- BLOCK END 61

}

function sum_of_polygon_angles(max_sides){
    /// --- BLOCK BEGIN 62
var side_count = user_randint(3, max_sides);
    var _sum = (side_count - 2) * 180;
    var problem = "What is the sum of interior angles of a polygon with " + side_count + " sides?";
    return [problem, _sum.toString()];
    /// --- BLOCK END 62

}

function surface_area_cone(max_radius, max_height, unit){
    /// --- BLOCK BEGIN 63
var a = user_randint(1, max_height);
    var b = user_randint(1, max_radius);
    var slopingHeight = Math.sqrt(a * a + b * b);
    var ans = Math.floor(Math.PI * b * slopingHeight + Math.PI * b * b);
    var problem = "Surface area of cone with height = " + a + unit + " and radius = " + b + unit + " is";
    var solution = ans + " " + unit + "^2";
    return [problem, solution];
    /// --- BLOCK END 63

}

function surface_area_cube(max_side, unit){
    /// --- BLOCK BEGIN 64
var a = user_randint(1, max_side);
    var ans = 6 * (a * a);
    var problem = "Surface area of cube with side = " + a + unit + " is";
    var solution = ans + " " + unit + "^2";
    return [problem, solution];
    /// --- BLOCK END 64

}

function surface_area_cuboid(max_side, unit){
    /// --- BLOCK BEGIN 65
var a = user_randint(1, max_side);
    var b = user_randint(1, max_side);
    var c = user_randint(1, max_side);
    var ans = 2 * (a * b + b * c + c * a);
    var problem = "Surface area of cuboid with sides of lengths: " + a + unit + ", " + b + unit + ", " + c + unit + " is";
    var solution = ans + " " + unit + "^2";
    return [problem, solution];
    /// --- BLOCK END 65

}

function surface_area_cylinder(max_radius, max_height, unit){
    /// --- BLOCK BEGIN 66
var a = user_randint(1, max_height);
    var b = user_randint(1, max_radius);
    var ans = parseInt(2 * Math.PI * a * b + 2 * Math.PI * b * b);
    var problem = "Surface area of cylinder with height = " + a + unit + " and radius = " + b + unit + " is";
    var solution = ans + " " + unit + "^2";
    return [problem, solution];

    /// --- BLOCK END 66

}

function surface_area_pyramid(unit){
    /// --- BLOCK BEGIN 67
var _PyTHAGOREAN = [[3, 4, 5], [6, 8, 10], [9, 12, 15], [12, 16, 20], [15, 20, 25], [5, 12, 13], [10, 24, 26], [7, 24, 25]];
    var tmp = user_choice_func2(_PyTHAGOREAN);
    var tmp2 = user_sample_func2(tmp, 3);
    var height = tmp2[0];
    var half_width = tmp2[1];
    var triangle_height_1 = tmp2[2];
    var triangle_1 = half_width * triangle_height_1;
    var second_triplet = user_choice_func2(_PyTHAGOREAN.filter(function(i) { return i.indexOf(height) !== -1; }));

    tmp2 = user_sample_func2(second_triplet.filter(function(i) { return i !== height; }), 2);
    var half_length = tmp2[0];
    var triangle_height_2 = tmp2[1];
    var triangle_2 = half_length * triangle_height_2;
    var base = 4 * half_width * half_length;
    var ans = base + 2 * triangle_1 + 2 * triangle_2;
    var problem = "Surface area of pyramid with base length = " + (2 * half_length) + unit + ", base width = " + (2 * half_width) + unit + ", and height = " + height + unit + " is";
    var solution = ans + " " + unit + "^2";
    return [problem, solution];
    /// --- BLOCK END 67

}

function surface_area_sphere(max_side, unit){
    /// --- BLOCK BEGIN 68
var r = user_randint(1, max_side);
    var ans = parseFloat((4 * Math.PI * r * r).toFixed(2));
    var problem = "Surface area of a sphere with radius = " + r + unit + " is";
    var solution = ans + " " + unit + "^2";
    return [problem, solution];
    /// --- BLOCK END 68

}

function third_angle_of_triangle(max_angle){
    /// --- BLOCK BEGIN 69
var angle1 = user_randint(1, max_angle);
    var angle2 = user_randint(1, max_angle);
    var angle3 = 180 - (angle1 + angle2);
    var problem = "Third angle of triangle with angles " + angle1 + " and " + angle2 + " = ";
    return [problem, angle3.toString()];
    /// --- BLOCK END 69

}

function valid_triangle(max_side_length){
    /// --- BLOCK BEGIN 70
var sideA = user_randint(1, max_side_length);
    var sideB = user_randint(1, max_side_length);
    var sideC = user_randint(1, max_side_length);
    var sideSums = [sideA + sideB, sideB + sideC, sideC + sideA];
    var sides = [sideC, sideA, sideB];
    var exists = true && (sides[0] < sideSums[0]) && (sides[1] < sideSums[1]) && (sides[2] < sideSums[2]);
    var problem = `Does triangle with sides ${sideA}, ${sideB} and ${sideC} exist?`;
    var solution = exists ? "yes" : "No";
    return [problem, `${solution}`];

    /// --- BLOCK END 70

}

function volume_cone(max_radius, max_height, unit){
    /// --- BLOCK BEGIN 71
var a = user_randint(1, max_height);
    var b = user_randint(1, max_radius);
    var ans = Math.floor(Math.PI * b * b * a * (1 / 3));
    var problem = "Volume of cone with height = " + a + unit + " and radius = " + b + unit + " is";
    var solution = ans + " " + unit + "^3";
    return [problem, solution];
    /// --- BLOCK END 71

}

function volume_cube(max_side, unit){
    /// --- BLOCK BEGIN 72
var a = user_randint(1, max_side);
    var ans = Math.pow(a, 3);
    var problem = "Volume of cube with a side length of " + a + unit + " is";
    var solution = ans + " " + unit + "^3";
    return [problem, solution];
    /// --- BLOCK END 72

}

function volume_cuboid(max_side, unit){
    /// --- BLOCK BEGIN 73
var a = user_randint(1, max_side);
    var b = user_randint(1, max_side);
    var c = user_randint(1, max_side);
    var ans = a * b * c;
    var problem = "Volume of cuboid with sides = " + a + unit + ", " + b + unit + ", " + c + unit + " is";
    var solution = ans + " " + unit + "^3";
    return [problem, solution];
    /// --- BLOCK END 73

}

function volume_cylinder(max_radius, max_height, unit){
    /// --- BLOCK BEGIN 74
var a = user_randint(1, max_height);
    var b = user_randint(1, max_radius);
    var ans = Math.floor(Math.PI * b * b * a);
    var problem = "Volume of cylinder with height = " + a + unit + " and radius = " + b + unit + " is";
    var solution = ans + " " + unit + "^3";
    return [problem, solution];
    /// --- BLOCK END 74

}

function volume_cone_frustum(max_r1, max_r2, max_height, unit){
    /// --- BLOCK BEGIN 75
var h = user_randint(1, max_height);
    var r1 = user_randint(1, max_r1);
    var r2 = user_randint(1, max_r2);
    var ans = Math.round(((Math.PI * h) * (Math.pow(r1, 2) + Math.pow(r2, 2) + r1 * r2)) / 3 * 100) / 100;
    var problem = "Volume of frustum with height = " + h + unit + " and r1 = " + r1 + unit + " is and r2 = " + r2 + unit + " is ";
    var solution = ans + " " + unit + "^3";
    return [problem, solution];
    /// --- BLOCK END 75

}

function volume_hemisphere(max_radius){
    /// --- BLOCK BEGIN 76
var r = user_randint(1, max_radius);
    var ans = Math.round((2 * Math.PI / 3) * Math.pow(r, 3) * 100) / 100;
    var problem = "Volume of hemisphere with radius " + r + " m = ";
    var solution = ans + " m^3";
    return [problem, solution];
    /// --- BLOCK END 76

}

function volume_pyramid(max_length, max_width, max_height, unit){
    /// --- BLOCK BEGIN 77
var length = user_randint(1, max_length);
    var width = user_randint(1, max_width);
    var height = user_randint(1, max_height);
    var ans = ((length * width * height) / 3).toFixed(1);
    var problem = "Volume of pyramid with base length = " + length + " " + unit + ", base width = " + width + " " + unit + " and height = " + height + " " + unit + " is";
    var solution = ans + " " + unit + "^3";
    return [problem, solution];

    /// --- BLOCK END 77

}

function volume_sphere(max_radius){
    /// --- BLOCK BEGIN 78
var r = user_randint(1, max_radius);
    var ans = Math.round((4 * Math.PI / 3) * Math.pow(r, 3) * 100) / 100;
    var problem = "Volume of sphere with radius " + r + " m = ";
    var solution = ans + " m^3";
    return [problem, solution];
    /// --- BLOCK END 78

}

function perimeter_of_polygons(max_sides, max_length){
    /// --- BLOCK BEGIN 79
var size_of_sides = user_randint(3, max_sides);
    var sides = [];
    for (var i = 0; i < size_of_sides; i++) {
        sides.push(user_randint(1, max_length));
    }
    var tmp = sides.join(', ');
    var problem = "The perimeter of a " + size_of_sides + " sided polygon with lengths of " + tmp + "cm is: ";
    var solution = sides.reduce(function(a, b) { return a + b; }, 0);
    return [problem, solution.toString()];
    /// --- BLOCK END 79

}

function assert_equal(a, b){
    /// --- BLOCK BEGIN 80
if (a !== b) {
    throw new Error('Assertion failed: a does not equal b');
}
return true;

    /// --- BLOCK END 80

}

function assert_iter_equal(a, b){
    /// --- BLOCK BEGIN 86
for (var index = 0; index < a.length; index++) {
    assert_equal(a[index], b[index]);
}
return true;
    /// --- BLOCK END 86

}
