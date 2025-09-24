const crypto = require('crypto');
const { exp } = require('numjs');
var seed = 1;
function user_hash_random(){
  const hash = crypto.createHash('sha256');
  hash.update(seed.toString());
  seed += 1;
  const hex = hash.digest('hex');
  return parseInt(hex, 16) / (2 ** 256 - 1);
}

function user_randint(min, max) {
  return Math.floor(user_hash_random() * (max - min + 1)) + min;
}

function user_choice_func1(x) {
  if (user_hash_random() < 0.5) {
    return x[0];
  } else {
    return x[1];
  }
}

function user_choice_func2(x) {
  return x[Math.floor(user_hash_random() * x.length)];
}

function user_sample_func1(x, n) {
  // This sample function user_works assuming x is in the form of range(a, b)
  var a = x[0];
  var b = x[x.length - 1] + 1;

  var lst = [];
  for (var i = 0; i < n; i++) {
    lst.push(Math.floor(user_hash_random() * (b - a + 1)) + a);
  }
  return lst;
}

function user_sample_func2(x, n) {
    x = Array.from(x);
    var lst = [];
    for (var i = 0; i < n; i++) {
        var index = Math.floor(user_hash_random() * x.length);
        lst.push(x[index]);
        x.splice(index, 1);
    }
    return lst;
}

function user_uniform(a, b) {
    return user_hash_random() * (b - a) + a
}

function user_reset_seed() {
    seed = 1;
}

function get_seed() {
    return seed;
}

function set_seed(new_seed) {
    seed = new_seed;
}

function factorial(a) {
  let result = 1;
  for (let i = 2; i <= a; i++) {
      result *= i;
  }
  return result;
}

function greatest_common_divisor(number1, number2){
  var _return_value;
  number1 = Math.abs(number1);
  number2 = Math.abs(number2);
  while (number2 > 0) {
    var temp = number1;
    number1 = number2;
    number2 = temp % number2;
  }
  _return_value = number1;
  return _return_value;
}

exports.user_hash_random = user_hash_random;
exports.user_randint = user_randint;
exports.user_choice_func1 = user_choice_func1;
exports.user_choice_func2 = user_choice_func2;
exports.user_sample_func1 = user_sample_func1;
exports.user_sample_func2 = user_sample_func2;
exports.user_uniform = user_uniform;
exports.user_reset_seed = user_reset_seed;
exports._factorial = factorial;
exports._greatest_common_divisor = greatest_common_divisor;
exports.get_seed = get_seed;
exports.set_seed = set_seed;
