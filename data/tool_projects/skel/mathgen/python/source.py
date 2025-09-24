
### SKEL HEAD BEGIN
def user_get_type(obj):
    if hasattr(obj, '_class_name'):
        return "<function " + obj._class_name.split(";")[0] + " >"
    else:
        return type(obj)

def user_check_type(obj, _type):
    if str(_type).startswith("<class") and str(_type).split("'")[1] in ["dict", "object"]:
        return isinstance(obj, _type)
    elif hasattr(obj, '_class_name'):
        if "function" in str(_type):
            for i in obj._class_name.split(";"):
                if i == str(_type).split(" ")[1]:
                    return True
            return False
    else:
        if str(_type).startswith("<function"):
            typename = str(_type).split(" ")[1]
            if typename == 'func_dict':
                return isinstance(obj, dict)
        return isinstance(obj, _type)


def SkelClass(class_name, super_class=None):
    if super_class is None:
        class myclass:
            _class_name = class_name
    else:
        class myclass(super_class):
            _class_name = class_name
    return myclass()

### SKEL HEAD END

import math

from tracer_skip import *

def absolute_difference(max_a, max_b):
    ### --- BLOCK BEGIN 1
    a = user_randint((- 1) * max_a, max_a)
    b = user_randint((- 1) * max_b, max_b)
    absDiff = abs(a - b)
    return [f'&|{a}-{b}|=&', f"&{absDiff}&"]
    ### --- BLOCK END 1



def addition(max_sum, max_addend):
    ### --- BLOCK BEGIN 2
    if max_addend > max_sum:
        max_addend = max_sum
    a = user_randint(0, max_addend)
    b = user_randint(0, min((max_sum - a), max_addend))
    c = a + b
    problem = f'&{a}+{b}=&'
    solution = f'&{c}&'
    return [problem, solution]
    ### --- BLOCK END 2



def compare_fractions(max_val):
    ### --- BLOCK BEGIN 3
    a = user_randint(1, max_val)
    b = user_randint(1, max_val)
    c = user_randint(1, max_val)
    d = user_randint(1, max_val)
    while (a == b):
        b = user_randint(1, max_val)
    while (c == d):
        d = user_randint(1, max_val)
    first = a / b
    second = c / d
    if (first > second):
        solution = ">"
    elif (first < second):
        solution = "<"
    else:
        solution = "="
    problem = rf"Which symbol represents the comparison between &\frac{{{a}}}{{{b}}}& and &\frac{{{c}}}{{{d}}}&?"
    return [problem, solution]
    ### --- BLOCK END 3



def cube_root(min_no, max_no):
    ### --- BLOCK BEGIN 4
    b = user_randint(min_no, max_no)
    a = b**(1 / 3)
    return (rf"What is the cube root of: &\sqrt[3]{{{b}}}=& to 2 decimal places?", f"&{round(a, 2)}&")
    ### --- BLOCK END 4



def divide_fractions(max_val):
    def calculate_gcd(x, y):
        ### --- BLOCK BEGIN 5
        while (y):
            x, y = y, x % y
        return x
        ### --- BLOCK END 5
    
    
    
    ### --- BLOCK BEGIN 6
    a = user_randint(1, max_val)
    b = user_randint(1, max_val)
    while (a == b):
        b = user_randint(1, max_val)
    c = user_randint(1, max_val)
    d = user_randint(1, max_val)
    while (c == d):
        d = user_randint(1, max_val)
    tmp_n = a * d
    tmp_d = b * c
    gcd = calculate_gcd(tmp_n, tmp_d)
    sol_numerator = tmp_n // gcd
    sol_denominator = tmp_d // gcd
    return [rf'&\frac{{{a}}}{{{b}}}\div\frac{{{c}}}{{{d}}}=&', rf'&\frac{{{sol_numerator}}}{{{sol_denominator}}}&']
    ### --- BLOCK END 6



def division(max_a, max_b):
    ### --- BLOCK BEGIN 7
    a = user_randint(1, max_a)
    b = user_randint(1, max_b)
    divisor = a * b
    dividend = user_choice_func1([a, b])
    quotient = int(divisor / dividend)
    return [rf'&{divisor}\div{dividend}=&', f'&{quotient}&']
    ### --- BLOCK END 7



def exponentiation(max_base, max_expo):
    ### --- BLOCK BEGIN 8
    base = user_randint(1, max_base)
    expo = user_randint(1, max_expo)
    return [f'&{base}^{{{expo}}}=&', f'&{base**expo}&']
    ### --- BLOCK END 8



def factorial(max_input):
    ### --- BLOCK BEGIN 9
    a = user_randint(0, max_input)
    n = a
    b = 1
    while a != 1 and n > 0:
        b *= n
        n -= 1
    return [f'&{a}! =&', f'&{b}&']
    ### --- BLOCK END 9



def fraction_multiplication(max_val):
    def calculate_gcd(x, y):
        ### --- BLOCK BEGIN 10
        while (y):
            x, y = y, x % y
        return x
        ### --- BLOCK END 10
    
    
    
    ### --- BLOCK BEGIN 11
    a = user_randint(1, max_val)
    b = user_randint(1, max_val)
    c = user_randint(1, max_val)
    d = user_randint(1, max_val)
    while (a == b):
        b = user_randint(1, max_val)
    while (c == d):
        d = user_randint(1, max_val)
    tmp_n = a * c
    tmp_d = b * d
    gcd = calculate_gcd(tmp_n, tmp_d)
    problem = rf"&\frac{{{a}}}{{{b}}}\cdot\frac{{{c}}}{{{d}}}=&"
    if (tmp_d == 1 or tmp_d == gcd):
        solution = rf"&\frac{{{tmp_n}}}{{{gcd}}}&"
    else:
        solution = rf"&\frac{{{tmp_n//gcd}}}{{{tmp_d//gcd}}}&"
    return [problem, solution]
    ### --- BLOCK END 11



def fraction_to_decimal(max_res, max_divid):
    ### --- BLOCK BEGIN 12
    a = user_randint(0, max_divid)
    b = user_randint(1, min(max_res, max_divid))
    c = round(a / b, 2)
    return [rf'&{a}\div{b}=&', f'&{c}&']
    ### --- BLOCK END 12



def greatest_common_divisor(numbers_count, max_num):
    def greatestCommonDivisorOfTwoNumbers(number1, number2):
        ### --- BLOCK BEGIN 13
        number1 = abs(number1)
        number2 = abs(number2)
        while number2 > 0:
            number1, number2 = number2, number1 % number2
        return number1
        ### --- BLOCK END 13
    
    
    
    ### --- BLOCK BEGIN 14
    numbers_count = max(numbers_count, 2)
    numbers = []
    for _i in range(numbers_count):
        numbers.append(user_randint(0, max_num))
    greatestCommonDivisor = greatestCommonDivisorOfTwoNumbers(numbers[0], numbers[1])
    for index in range(1, numbers_count):
        greatestCommonDivisor = greatestCommonDivisorOfTwoNumbers(numbers[index], greatestCommonDivisor)
    fix_bug = ",".join(map(str, numbers))
    return [f'&GCD({fix_bug})=&', f"&{greatestCommonDivisor}&"]
    ### --- BLOCK END 14



def is_composite(max_num):
    ### --- BLOCK BEGIN 15
    a = user_randint(2, max_num)
    problem = f"Is &{a}& composite?"
    if a == 0 or a == 1:
        return [problem, "No"]
    for i in range(2, a):
        if a % i == 0:
            return [problem, "Yes"]
    solution = "No"
    return [problem, solution]
    ### --- BLOCK END 15



def is_prime(max_num):
    ### --- BLOCK BEGIN 16
    a = user_randint(2, max_num)
    problem = f"Is &{a}& prime?"
    if a == 2:
        return [problem, "Yes"]
    if a % 2 == 0:
        return [problem, "No"]
    for i in range(3, a // 2 + 1, 2):
        if a % i == 0:
            return [problem, "No"]
    solution = "Yes"
    return [problem, solution]
    ### --- BLOCK END 16



def multiplication(max_multi):
    ### --- BLOCK BEGIN 17
    a = user_randint(0, max_multi)
    b = user_randint(0, max_multi)
    c = a * b
    return [rf'&{a}\cdot{b}=&', f'&{c}&']
    ### --- BLOCK END 17



def percentage(max_value, max_percentage):
    ### --- BLOCK BEGIN 18
    a = user_randint(1, max_percentage)
    b = user_randint(1, max_value)
    problem = f"What is &{a}&% of &{b}&?"
    percentage = a / 100 * b
    formatted_float = "{:.2f}".format(percentage)
    solution = f"&{formatted_float}&"
    return [problem, solution]
    ### --- BLOCK END 18



def percentage_difference(max_value, min_value):
    ### --- BLOCK BEGIN 19
    value_a = user_randint(min_value, max_value)
    value_b = user_randint(min_value, max_value)
    diff = 2 * (abs(value_a - value_b) / abs(value_a + value_b)) * 100
    diff = round(diff, 2)
    problem = f"What is the percentage difference between &{value_a}& and &{value_b}&?"
    solution = f'&{diff}&%'
    return [problem, solution]
    ### --- BLOCK END 19



def percentage_error(max_value, min_value):
    ### --- BLOCK BEGIN 20
    observed_value = user_randint(min_value, max_value)
    exact_value = user_randint(min_value, max_value)
    if observed_value * exact_value < 0:
        observed_value *= -1
    error = (abs(observed_value - exact_value) / abs(exact_value)) * 100
    error = round(error, 2)
    problem = f"Find the percentage error when observed value equals &{observed_value}& and exact value equals &{exact_value}&."
    solution = f'&{error}&%'
    return [problem, solution]
    ### --- BLOCK END 20



def power_of_powers(max_base, max_power):
    ### --- BLOCK BEGIN 21
    base = user_randint(1, max_base)
    power1 = user_randint(1, max_power)
    power2 = user_randint(1, max_power)
    step = power1 * power2
    problem = f"Simplify &{base}^{{{power1}^{{{power2}}}}}&"
    solution = f"&{base}^{{{step}}}&"
    return [problem, solution]
    ### --- BLOCK END 21



def square(max_square_num):
    ### --- BLOCK BEGIN 22
    a = user_randint(1, max_square_num)
    b = a ** 2
    return [f'&{a}^2=&', f'&{b}&']
    ### --- BLOCK END 22



def square_root(min_no, max_no):
    ### --- BLOCK BEGIN 23
    b = user_randint(min_no, max_no)
    a = b ** 2
    return [rf'&\sqrt{{{a}}}=&', f'&{b}&']
    ### --- BLOCK END 23



def simplify_square_root(max_variable):
    ### --- BLOCK BEGIN 24
    y = x = user_randint(1, max_variable)
    factors = {}
    f = 2
    while x != 1:
        if x % f == 0:
            if f not in factors:
                factors[f] = 0
            factors[f] += 1
            x /= f
        else:
            f += 1
    a = b = 1
    for i in factors.keys():
        if factors[i] % 2 == 0:
            a *= i ** (factors[i] // 2)
        else:
            a *= i ** ((factors[i] - 1) // 2)
            b *= i
    if a == 1 or b == 1:
        return simplify_square_root(max_variable)
    else:
        return [rf'&\sqrt{{{y}}}&', rf'&{a}\sqrt{{{b}}}&']
    ### --- BLOCK END 24



def subtraction(max_minuend, max_diff):
    ### --- BLOCK BEGIN 25
    a = user_randint(0, max_minuend)
    b = user_randint(max(0, (a - max_diff)), a)
    c = a - b
    return [f'&{a}-{b}=&', f'&{c}&']
    ### --- BLOCK END 25



def bcd_to_decimal(max_number):
    ### --- BLOCK BEGIN 26
    n = user_randint(1000, max_number)
    binstring = ''
    while True:
        q, r = divmod(n, 10)
        nibble = bin(r).replace('0b', "")
        while len(nibble) < 4:
            nibble = '0' + nibble
        binstring = nibble + binstring
        if q == 0:
            break
        else:
            n = q
    problem = f"Integer of Binary Coded Decimal &{n} =& "
    solution = f'&{int(binstring, 2)}&'
    return [problem, solution]
    ### --- BLOCK END 26



def binary_2s_complement(maxDigits):
    ### --- BLOCK BEGIN 27
    digits = user_randint(1, maxDigits)
    question = ''.join([str(user_randint(0, 1)) for i in range(digits)]).lstrip('0')
    answer = []
    for i in question:
        answer.append(str(int(not bool(int(i)))))
    carry = True
    j = len(answer) - 1
    while j >= 0:
        if answer[j] == '0':
            answer[j] = '1'
            carry = False
            break
        answer[j] = '0'
        j -= 1
    # if j == 0 and carry is True:
    #     answer.insert(0, '1')
    problem = f"2^s complement of &{question} = &"
    solution = ''.join(answer).lstrip('0')
    return [problem, f'&{solution}&']
    ### --- BLOCK END 27



def binary_complement_1s(maxDigits):
    ### --- BLOCK BEGIN 28
    question = ''.join([str(user_randint(0, 1)) for _ in range(user_randint(1, maxDigits))])
    answer = ''.join(["0" if digit == "1" else "1" for digit in question])
    problem = f'&{question} = &'
    return [problem, f'&{answer}&']
    ### --- BLOCK END 28



def binary_to_decimal(max_dig):
    ### --- BLOCK BEGIN 29
    problem = ''.join([str(user_randint(0, 1)) for _ in range(user_randint(1, max_dig))])
    solution = f'&{int(problem, 2)}&'
    return [f'&{problem}&', solution]
    ### --- BLOCK END 29



def binary_to_hex(max_dig):
    ### --- BLOCK BEGIN 30
    problem = ''.join([str(user_randint(0, 1)) for _ in range(user_randint(1, max_dig))])
    solution = f'&{hex(int(problem, 2))}&'
    return [f'&{problem}&', solution]
    ### --- BLOCK END 30



def decimal_to_bcd(max_number):
    ### --- BLOCK BEGIN 31
    n = user_randint(1000, max_number)
    x = n
    # binstring = ''
    bcdstring = ''
    while x > 0:
        nibble = x % 16
        bcdstring = str(nibble) + bcdstring
        x >>= 4
    problem = f"BCD of Decimal Number &{n} = &"
    return [problem, f'&{bcdstring}&']
    ### --- BLOCK END 31



def decimal_to_binary(max_dec):
    ### --- BLOCK BEGIN 32
    a = user_randint(1, max_dec)
    b = bin(a).replace("0b", "")
    problem = f'Binary of &{a} = &'
    solution = f'&{b}&'
    return [problem, solution]
    ### --- BLOCK END 32



def decimal_to_hexadeci(max_dec):
    ### --- BLOCK BEGIN 33
    a = user_randint(0, max_dec)
    b = hex(a)
    problem = f"Hexadecimal of &{a} = &"
    solution = f"&{b}&"
    return [problem, solution]
    ### --- BLOCK END 33



def decimal_to_octal(max_decimal):
    ### --- BLOCK BEGIN 34
    x = user_randint(0, max_decimal)
    problem = f"The decimal number &{x}& in octal is: "
    solution = f'&{oct(x)}&'
    return [problem, solution]
    ### --- BLOCK END 34



def fibonacci_series(min_no):
    def createFibList(n):
        ### --- BLOCK BEGIN 35
        list = []
        for i in range(n):
            if i < 2:
                list.append(i)
            else:
                val = list[i - 1] + list[i - 2]
                list.append(val)
        return list
        ### --- BLOCK END 35
    
    
    
    ### --- BLOCK BEGIN 36
    n = user_randint(min_no, 20)
    fibList = createFibList(n)
    problem = f"The Fibonacci Series of the first &{n}& numbers is ?"
    solution = ', '.join(map(str, fibList))
    return [problem, f'&{solution}&']
    ### --- BLOCK END 36



def modulo_division(max_res, max_modulo):
    ### --- BLOCK BEGIN 37
    a = user_randint(0, max_modulo)
    b = user_randint(0, min(max_res, max_modulo))
    c = a % b if b != 0 else 0
    problem = f'&{a}& % &{b}& = &'
    solution = f'&{c}&'
    return [problem, solution]
    ### --- BLOCK END 37



def nth_fibonacci_number(max_n):
    ### --- BLOCK BEGIN 38
    gratio = (1 + math.sqrt(5)) / 2
    n = user_randint(1, max_n)
    problem = f"What is the {n}th Fibonacci number?"
    solution = int((math.pow(gratio, n) - math.pow(-gratio, -n)) / (math.sqrt(5)))
    return [problem, f'&{solution}&']
    ### --- BLOCK END 38



def combinations(max_lengthgth):
    ### --- BLOCK BEGIN 39
    a = user_randint(10, max_lengthgth)
    b = user_randint(0, 9)
    solution = int(math.factorial(a) / (math.factorial(b) * math.factorial(a - b)))
    problem = f"Find the number of combinations from &{a}& objects picked &{b}& at a time."
    return [problem, f'&{solution}&']
    ### --- BLOCK END 39



def conditional_probability():
    def BayesFormula(P_disease, true_positive, true_negative):
        ### --- BLOCK BEGIN 40
        P_notDisease = 100. - P_disease
        false_positive = 100. - true_negative
        P_plus = (P_disease) * (true_positive) + (P_notDisease) * (false_positive)
        P_disease_plus = ((true_positive) * (100 * P_disease)) / P_plus
        return P_disease_plus
        ### --- BLOCK END 40
    
    
    
    ### --- BLOCK BEGIN 41
    P_disease = round(2. * user_hash_random(), 2)
    true_positive = round(user_hash_random() + float(user_randint(90, 99)), 2)
    true_negative = round(user_hash_random() + float(user_randint(90, 99)), 2)
    answer = round(BayesFormula(P_disease, true_positive, true_negative), 2)
    problem = "Someone tested positive for a nasty disease which only &{0:.2f}&% of the population have. Test sensitivity (true positive) is equal to &SN={1:.2f}&% whereas test specificity (true negative) &SP={2:.2f}&%. What is the probability that this guy really has that disease?".format(P_disease, true_positive, true_negative)
    solution = f'&{answer}&%'
    return [problem, solution]
    ### --- BLOCK END 41



def confidence_interval():
    ### --- BLOCK BEGIN 42
    n = user_randint(20, 40)
    j = user_randint(0, 3)
    lst = user_sample_func1(range(200, 300), n)
    lst_per = [80, 90, 95, 99]
    lst_t = [1.282, 1.645, 1.960, 2.576]
    mean = 0
    sd = 0
    for i in lst:
        count = i + mean
        mean = count
    mean = mean / n
    for i in lst:
        x = (i - mean)**2 + sd
        sd = x
    sd = sd / n
    standard_error = lst_t[j] * math.sqrt(sd / n)
    upper = round(mean + standard_error, 2)
    lower = round(mean - standard_error, 2)
    problem = 'The confidence interval for sample &{}& with &{}&% confidence is'.format([x for x in lst], lst_per[j])
    solution = f'&({upper}, {lower})&'
    return [problem, solution]
    ### --- BLOCK END 42



def data_summary(number_values, min_val, max_val):
    ### --- BLOCK BEGIN 43
    random_list = []
    for i in range(number_values):
        n = user_randint(min_val, max_val)
        random_list.append(n)
    a = sum(random_list)
    mean = round(a / number_values, 2)
    _var = 0
    for i in range(number_values):
        _var += (random_list[i] - mean)**2
    standardDeviation = round(_var / number_values, 2)
    variance = round((_var / number_values) ** 0.5, 2)
    tmp = ', '.join(map(str, random_list))
    problem = f"Find the mean,standard deviation and variance for the data &{tmp}&"
    solution = f"The Mean is &{mean}&, Standard Deviation is &{standardDeviation}&, Variance is &{variance}&"
    return [problem, solution]
    ### --- BLOCK END 43



def dice_sum_probability(max_dice):
    ### --- BLOCK BEGIN 44
    a = user_randint(1, max_dice)
    b = user_randint(a, 6 * a)
    count = 0
    for i in [1, 2, 3, 4, 5, 6]:
        if a == 1:
            if i == b:
                count = count + 1
        elif a == 2:
            for j in [1, 2, 3, 4, 5, 6]:
                if i + j == b:
                    count = count + 1
        elif a == 3:
            for j in [1, 2, 3, 4, 5, 6]:
                for k in [1, 2, 3, 4, 5, 6]:
                    if i + j + k == b:
                        count = count + 1
    problem = f"If &{a}& dice are rolled at the same time, the probability of getting a sum of &{b} =&"
    solution = rf"\frac{{{count}}}{{{6**a}}}"
    return [problem, solution]
    ### --- BLOCK END 44



def mean_median(max_length):
    ### --- BLOCK BEGIN 45
    randomlist = user_sample_func1(range(1, 99), max_length)
    total = 0
    for n in randomlist:
        total = total + n
    mean = total / 10
    randomlist.sort()
    median = (randomlist[4] + randomlist[5]) / 2
    problem = f"Given the series of numbers &{randomlist}&. Find the arithmatic mean and median of the series"
    solution = f"Arithmetic mean of the series is &{mean}& and arithmetic median of this series is &{median}&"
    return [problem, solution]
    ### --- BLOCK END 45



def permutation(max_lengthgth):
    ### --- BLOCK BEGIN 46
    a = user_randint(10, max_lengthgth)
    b = user_randint(0, 9)
    solution = int(math.factorial(a) / (math.factorial(a - b)))
    problem = f"Number of Permutations from &{a}& objects picked &{b}& at a time is: "
    return [problem, f"&{solution}&"]
    ### --- BLOCK END 46



def angle_btw_vectors(max_elt_amt):
    ### --- BLOCK BEGIN 47
    s = 0
    v1 = [round(user_uniform(0, 1000), 2) for i in range(user_randint(2, max_elt_amt))]
    v2 = [round(user_uniform(0, 1000), 2) for i in v1]
    for i in range(len(v1)):
        s += v1[i] * v2[i]
    mags = math.sqrt(sum([i**2 for i in v1])) * math.sqrt(sum([i**2 for i in v2]))
    solution = ''
    ans = 0
    try:
        ans = round(math.acos(s / mags), 2)
        solution = f"{ans} radians"
    except ValueError:
        print('angleBtwVectorsFunc has some issues with math module, line 16')
        solution = 'NaN'
        ans = 'NaN'
    problem = f"angle between the vectors {v1} and {v2} is:"
    return [problem, solution]
    ### --- BLOCK END 47



def angle_regular_polygon(min_val, max_val):
    ### --- BLOCK BEGIN 48
    sideNum = user_randint(min_val, max_val)
    problem = f"Find the angle of a regular polygon with {sideNum} sides"
    exteriorAngle = round((360 / sideNum), 2)
    solution = f'{180 - exteriorAngle}'
    return [problem, solution]
    ### --- BLOCK END 48



def arc_length(max_radius, max_angle):
    ### --- BLOCK BEGIN 49
    radius = user_randint(1, max_radius)
    angle = user_randint(1, max_angle)
    angle_arc_length = float((angle / 360) * 2 * math.pi * radius)
    formatted_float = "{:.5f}".format(angle_arc_length)
    problem = f"Given radius, {radius} and angle, {angle}. Find the arc length of the angle."
    solution = f"Arc length of the angle = {formatted_float}"
    return [problem, solution]
    ### --- BLOCK END 49



def area_of_circle(max_radius):
    ### --- BLOCK BEGIN 50
    r = user_randint(0, max_radius)
    area = round(math.pi * r * r, 2)
    problem = f'Area of circle with radius {r}='
    return [problem, f'{area}']
    ### --- BLOCK END 50



def area_of_circle_given_center_and_point(max_coordinate, max_radius):
    ### --- BLOCK BEGIN 51
    r = user_randint(0, max_radius)
    center_x = user_randint(-max_coordinate, max_coordinate)
    center_y = user_randint(-max_coordinate, max_coordinate)
    angle = user_choice_func2([0, math.pi // 6, math.pi // 2, math.pi, math.pi + math.pi // 6, 3 * math.pi // 2])
    point_x = center_x + round(r * math.cos(angle), 2)
    point_y = center_y + round(r * math.sin(angle), 2)
    area = round(math.pi * r * r, 2)
    problem = f"Area of circle with center ({center_x},{center_y}) and passing through ({point_x}, {point_y}) is"
    return [problem, f'{area}']
    ### --- BLOCK END 51



def area_of_triangle(max_a, max_b):
    ### --- BLOCK BEGIN 52
    a = user_randint(1, max_a)
    b = user_randint(1, max_b)
    c = user_randint(abs(b - a) + 1, abs(a + b) - 1)
    s = (a + b + c) / 2
    area = (s * (s - a) * (s - b) * (s - c))**0.5
    problem = f"Area of triangle with side lengths: {a}, {b}, {c} = "
    solution = f'{round(area, 2)}'
    return [problem, solution]
    ### --- BLOCK END 52



def circumference(max_radius):
    ### --- BLOCK BEGIN 53
    r = user_randint(0, max_radius)
    circumference = round(2 * math.pi * r, 2)
    problem = f"Circumference of circle with radius {r} = "
    return [problem, f'{circumference}']
    ### --- BLOCK END 53



def complementary_and_supplementary_angle(max_supp, max_comp):
    ### --- BLOCK BEGIN 54
    angleType = user_choice_func2(["supplementary", "complementary"])
    if angleType == "supplementary":
        angle = user_randint(1, max_supp)
        angleAns = 180 - angle
    else:
        angle = user_randint(1, max_comp)
        angleAns = 90 - angle
    problem = f"The {angleType} angle of {angle} ="
    solution = f'{angleAns}'
    return [problem, solution]
    ### --- BLOCK END 54



def curved_surface_area_cylinder(max_radius, max_height):
    ### --- BLOCK BEGIN 55
    r = user_randint(1, max_radius)
    h = user_randint(1, max_height)
    csa = float(2 * math.pi * r * h)
    formatted_float = round(csa, 2)
    # "{:.5f}".format(csa)
    problem = f"What is the curved surface area of a cylinder of radius, {r} and height, {h}?"
    solution = f"{formatted_float}"
    return [problem, solution]
    ### --- BLOCK END 55



def degree_to_rad(max_deg):
    ### --- BLOCK BEGIN 56
    a = user_randint(0, max_deg)
    b = (math.pi * a) / 180
    b = round(b, 2)
    problem = f"Angle {a} degrees in radians is: "
    solution = f'{b}'
    return [problem, solution]
    ### --- BLOCK END 56



def equation_of_line_from_two_points(max_coordinate, min_coordinate):
    ### --- BLOCK BEGIN 57
    x1 = user_randint(min_coordinate, max_coordinate)
    x2 = user_randint(min_coordinate, max_coordinate)
    y1 = user_randint(min_coordinate, max_coordinate)
    y2 = user_randint(min_coordinate, max_coordinate)
    coeff_y = (x2 - x1)
    coeff_x = (y2 - y1)
    constant = y2 * coeff_y - x2 * coeff_x
    gcd = math.gcd(abs(coeff_x), abs(coeff_y))
    if gcd != 1:
        if coeff_y > 0:
            coeff_y //= gcd
        if coeff_x > 0:
            coeff_x //= gcd
        if constant > 0:
            constant //= gcd
        if coeff_y < 0:
            coeff_y = -(-coeff_y // gcd)
        if coeff_x < 0:
            coeff_x = -(-coeff_x // gcd)
        if constant < 0:
            constant = -(-constant // gcd)
    if coeff_y < 0:
        coeff_y = -(coeff_y)
        coeff_x = -(coeff_x)
        constant = -(constant)
    if coeff_x in [1, -1]:
        if coeff_x == 1:
            coeff_x = ''
        else:
            coeff_x = '-'
    if coeff_y in [1, -1]:
        if coeff_y == 1:
            coeff_y = ''
        else:
            coeff_y = '-'
    problem = f"What is the equation of the line between points ({x1},{y1}) and ({x2},{y2}) in slope-intercept form?"
    if coeff_x == 0:
        solution = str(coeff_y) + "y = " + str(constant)
    elif coeff_y == 0:
        solution = str(coeff_x) + "x = " + str(-constant)
    else:
        if constant > 0:
            solution = str(coeff_y) + "y = " + str(coeff_x) + "x + " + str(constant)
        else:
            solution = str(coeff_y) + "y = " + str(coeff_x) + "x " + str(constant)
    return [problem, f'{solution}']
    ### --- BLOCK END 57



def fourth_angle_of_quadrilateral(max_angle):
    ### --- BLOCK BEGIN 58
    angle1 = user_randint(1, max_angle)
    angle2 = user_randint(1, 240 - angle1)
    angle3 = user_randint(1, 340 - (angle1 + angle2))
    sum_ = angle1 + angle2 + angle3
    angle4 = 360 - sum_
    problem = f"Fourth angle of quadrilateral with angles {angle1} , {angle2}, {angle3} ="
    solution = f'{angle4}'
    return [problem, solution]
    ### --- BLOCK END 58



def pythagorean_theorem(max_length):
    ### --- BLOCK BEGIN 59
    a = user_randint(1, max_length)
    b = user_randint(1, max_length)
    c = round((a ** 2 + b ** 2) ** 0.5, 2)
    problem = f"What is the hypotenuse of a right triangle given the other two sides have lengths {a} and {b}?"
    solution = f"{c}"
    return [problem, solution]
    ### --- BLOCK END 59



def radian_to_deg(max_rad):
    ### --- BLOCK BEGIN 60
    a = user_randint(0, int(max_rad * 100)) / 100
    b = round((180 * a) / math.pi, 2)
    problem = f"Angle {a} radians in degrees is: "
    solution = f'{b}'
    return [problem, solution]
    ### --- BLOCK END 60



def sector_area(max_radius, max_angle):
    ### --- BLOCK BEGIN 61
    r = user_randint(1, max_radius)
    a = user_randint(1, max_angle)
    secArea = float((a / 360) * math.pi * r * r)
    formatted_float = round(secArea, 2)
    problem = f"What is the area of a sector with radius {r} and angle {a} degrees?"
    solution = f"{formatted_float}"
    return [problem, solution]
    ### --- BLOCK END 61



def sum_of_polygon_angles(max_sides):
    ### --- BLOCK BEGIN 62
    side_count = user_randint(3, max_sides)
    _sum = (side_count - 2) * 180
    problem = f"What is the sum of interior angles of a polygon with {side_count} sides?"
    return [problem, f'{_sum}']
    ### --- BLOCK END 62



def surface_area_cone(max_radius, max_height, unit):
    ### --- BLOCK BEGIN 63
    a = user_randint(1, max_height)
    b = user_randint(1, max_radius)
    slopingHeight = math.sqrt(a**2 + b**2)
    ans = int(math.pi * b * slopingHeight + math.pi * b * b)
    problem = f"Surface area of cone with height = {a}{unit} and radius = {b}{unit} is"
    solution = f"{ans} {unit}^2"
    return [problem, solution]
    ### --- BLOCK END 63



def surface_area_cube(max_side, unit):
    ### --- BLOCK BEGIN 64
    a = user_randint(1, max_side)
    ans = 6 * (a ** 2)
    problem = f"Surface area of cube with side = {a}{unit} is"
    solution = f"{ans} {unit}^2"
    return [problem, solution]
    ### --- BLOCK END 64



def surface_area_cuboid(max_side, unit):
    ### --- BLOCK BEGIN 65
    a = user_randint(1, max_side)
    b = user_randint(1, max_side)
    c = user_randint(1, max_side)
    ans = 2 * (a * b + b * c + c * a)
    problem = f"Surface area of cuboid with sides of lengths: {a}{unit}, {b}{unit}, {c}{unit} is"
    solution = f"{ans} {unit}^2"
    return [problem, solution]
    ### --- BLOCK END 65



def surface_area_cylinder(max_radius, max_height, unit):
    ### --- BLOCK BEGIN 66
    a = user_randint(1, max_height)
    b = user_randint(1, max_radius)
    ans = int(2 * math.pi * a * b + 2 * math.pi * b * b)
    problem = f"Surface area of cylinder with height = {a}{unit} and radius = {b}{unit} is"
    solution = f"{ans} {unit}^2"
    return [problem, solution]
    ### --- BLOCK END 66



def surface_area_pyramid(unit):
    ### --- BLOCK BEGIN 67
    _PyTHAGOREAN = [(3, 4, 5),(6, 8, 10),(9, 12, 15),(12, 16, 20),(15, 20, 25),(5, 12, 13),(10, 24, 26),(7, 24, 25)]
    tmp = user_choice_func2(_PyTHAGOREAN)
    tmp2 = user_sample_func2(tmp, 3)
    height = tmp2[0]
    half_width = tmp2[1]
    triangle_height_1 = tmp2[2]
    triangle_1 = half_width * triangle_height_1
    second_triplet = user_choice_func2([i for i in _PyTHAGOREAN if height in i])
    tmp2 = user_sample_func2(tuple(i for i in second_triplet if i != height), 2)
    half_length = tmp2[0]
    triangle_height_2 = tmp2[1]
    triangle_2 = half_length * triangle_height_2
    base = 4 * half_width * half_length
    ans = base + 2 * triangle_1 + 2 * triangle_2
    problem = f"Surface area of pyramid with base length = {2*half_length}{unit}, base width = {2*half_width}{unit}, and height = {height}{unit} is"
    solution = f"{ans} {unit}^2"
    return [problem, solution]
    ### --- BLOCK END 67



def surface_area_sphere(max_side, unit):
    ### --- BLOCK BEGIN 68
    r = user_randint(1, max_side)
    ans = round(4 * math.pi * r * r, 2)
    problem = f"Surface area of a sphere with radius = {r}{unit} is"
    solution = f"{ans} {unit}^2"
    return [problem, solution]
    ### --- BLOCK END 68



def third_angle_of_triangle(max_angle):
    ### --- BLOCK BEGIN 69
    angle1 = user_randint(1, max_angle)
    angle2 = user_randint(1, max_angle)
    angle3 = 180 - (angle1 + angle2)
    problem = f"Third angle of triangle with angles {angle1} and {angle2} = "
    return [problem, f'{angle3}']
    ### --- BLOCK END 69



def valid_triangle(max_side_length):
    ### --- BLOCK BEGIN 70
    sideA = user_randint(1, max_side_length)
    sideB = user_randint(1, max_side_length)
    sideC = user_randint(1, max_side_length)
    sideSums = [sideA + sideB, sideB + sideC, sideC + sideA]
    sides = [sideC, sideA, sideB]
    exists = True & (sides[0] < sideSums[0]) & (sides[1] < sideSums[1]) & (
    sides[2] < sideSums[2])
    problem = f"Does triangle with sides {sideA}, {sideB} and {sideC} exist?"
    solution = "yes" if exists else "No"
    return [problem, f'{solution}']
    ### --- BLOCK END 70



def volume_cone(max_radius, max_height, unit):
    ### --- BLOCK BEGIN 71
    a = user_randint(1, max_height)
    b = user_randint(1, max_radius)
    ans = int(math.pi * b * b * a * (1 / 3))
    problem = f"Volume of cone with height = {a}{unit} and radius = {b}{unit} is"
    solution = f"{ans} {unit}^3"
    return [problem, solution]
    ### --- BLOCK END 71



def volume_cube(max_side, unit):
    ### --- BLOCK BEGIN 72
    a = user_randint(1, max_side)
    ans = a**3
    problem = f"Volume of cube with a side length of {a}{unit} is"
    solution = f"{ans} {unit}^3"
    return [problem, solution]
    ### --- BLOCK END 72



def volume_cuboid(max_side, unit):
    ### --- BLOCK BEGIN 73
    a = user_randint(1, max_side)
    b = user_randint(1, max_side)
    c = user_randint(1, max_side)
    ans = a * b * c
    problem = f"Volume of cuboid with sides = {a}{unit}, {b}{unit}, {c}{unit} is"
    solution = f"{ans} {unit}^3"
    return [problem, solution]
    ### --- BLOCK END 73



def volume_cylinder(max_radius, max_height, unit):
    ### --- BLOCK BEGIN 74
    a = user_randint(1, max_height)
    b = user_randint(1, max_radius)
    ans = int(math.pi * b * b * a)
    problem = f"Volume of cylinder with height = {a}{unit} and radius = {b}{unit} is"
    solution = f"{ans} {unit}^3"
    return [problem, solution]
    ### --- BLOCK END 74



def volume_cone_frustum(max_r1, max_r2, max_height, unit):
    ### --- BLOCK BEGIN 75
    h = user_randint(1, max_height)
    r1 = user_randint(1, max_r1)
    r2 = user_randint(1, max_r2)
    ans = round(((math.pi * h) * (r1 ** 2 + r2 ** 2 + r1 * r2)) / 3, 2)
    problem = f"Volume of frustum with height = {h}{unit} and r1 = {r1}{unit} is and r2 = {r2}{unit} is "
    solution = f"{ans} {unit}^3"
    return [problem, solution]
    ### --- BLOCK END 75



def volume_hemisphere(max_radius):
    ### --- BLOCK BEGIN 76
    r = user_randint(1, max_radius)
    ans = round((2 * math.pi / 3) * r**3, 2)
    problem = f"Volume of hemisphere with radius {r} m = "
    solution = f"{ans} m^3"
    return [problem, solution]
    ### --- BLOCK END 76



def volume_pyramid(max_length, max_width, max_height, unit):
    ### --- BLOCK BEGIN 77
    length = user_randint(1, max_length)
    width = user_randint(1, max_width)
    height = user_randint(1, max_height)
    ans = round((length * width * height) / 3, 2)
    problem = f"Volume of pyramid with base length = {length} {unit}, base width = {width} {unit} and height = {height} {unit} is"
    solution = f"{ans} {unit}^3"
    return [problem, solution]
    ### --- BLOCK END 77



def volume_sphere(max_radius):
    ### --- BLOCK BEGIN 78
    r = user_randint(1, max_radius)
    ans = round((4 * math.pi / 3) * r**3, 2)
    problem = f"Volume of sphere with radius {r} m = "
    solution = f"{ans} m^3"
    return [problem, solution]
    ### --- BLOCK END 78



def perimeter_of_polygons(max_sides, max_length):
    ### --- BLOCK BEGIN 79
    size_of_sides = user_randint(3, max_sides)
    sides = []
    for i in range(size_of_sides):
        sides.append(user_randint(1, max_length))
    tmp = ', '.join(map(str, sides))
    problem = f"The perimeter of a {size_of_sides} sided polygon with lengths of {tmp}cm is: "
    solution = sum(sides)
    return [problem, f'{solution}']
    ### --- BLOCK END 79



def assert_equal(a, b):
    ### --- BLOCK BEGIN 80
    assert a == b
    return True
    ### --- BLOCK END 80



def test_1():
    ### --- BLOCK BEGIN 81
    tmp = absolute_difference(100, 100)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&|-16-66|=&')
    assert_equal(b, '&82&')
    tmp = addition(99, 50)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&15+14=&')
    assert_equal(b, '&29&')
    tmp = compare_fractions(10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Which symbol represents the comparison between &\\frac{10}{1}& and &\\frac{5}{2}&?')
    assert_equal(b, '>')
    tmp = cube_root(1, 1000)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'What is the cube root of: &\\sqrt[3]{291}=& to 2 decimal places?')
    assert_equal(b, '&6.63&')
    tmp = divide_fractions(10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&\\frac{4}{5}\\div\\frac{3}{6}=&')
    assert_equal(b, '&\\frac{8}{5}&')
    tmp = division(25, 25)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&414\\div23=&')
    assert_equal(b, '&18&')
    tmp = exponentiation(20, 10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&7^{6}=&')
    assert_equal(b, '&117649&')
    tmp = factorial(6)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&6! =&')
    assert_equal(b, '&720&')
    tmp = fraction_multiplication(10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&\\frac{5}{8}\\cdot\\frac{4}{8}=&')
    assert_equal(b, '&\\frac{5}{16}&')
    tmp = fraction_to_decimal(99, 99)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&37\\div40=&')
    assert_equal(b, '&0.93&')
    tmp = greatest_common_divisor(2, 10**3)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&GCD(351,207)=&')
    assert_equal(b, '&9&')
    tmp = is_composite(250)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Is &97& composite?')
    assert_equal(b, 'No')
    tmp = is_prime(100)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Is &92& prime?')
    assert_equal(b, 'No')
    tmp = multiplication(12)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&11\\cdot10=&')
    assert_equal(b, '&110&')
    tmp = percentage(99, 99)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'What is &53&% of &62&?')
    assert_equal(b, '&32.86&')
    tmp = percentage_difference(200, 0)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'What is the percentage difference between &93& and &96&?')
    assert_equal(b, '&3.17&%')
    tmp = percentage_error(100, -100)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Find the percentage error when observed value equals &-37& and exact value equals &-91&.')
    assert_equal(b, '&59.34&%')
    tmp = power_of_powers(50, 10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Simplify &42^{3^{5}}&')
    assert_equal(b, '&42^{15}&')
    tmp = square(20)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&6^2=&')
    assert_equal(b, '&36&')
    tmp = square_root(1, 12)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&\\sqrt{36}=&')
    assert_equal(b, '&6&')
    tmp = simplify_square_root(100)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&\\sqrt{20}&')
    assert_equal(b, '&2\\sqrt{5}&')
    tmp = subtraction(99, 99)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&59-3=&')
    assert_equal(b, '&56&')
    ### --- BLOCK END 81



def test_2():
    ### --- BLOCK BEGIN 82
    tmp = bcd_to_decimal(10000)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Integer of Binary Coded Decimal &4 =& ')
    assert_equal(b, '&18304&')
    tmp = binary_2s_complement(10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, "2^s complement of &1100000 = &")
    assert_equal(b, '&100000&')
    tmp = binary_complement_1s(10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&01110 = &')
    assert_equal(b, '&10001&')
    tmp = binary_to_decimal(10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&1100&')
    assert_equal(b, '&12&')
    tmp = binary_to_hex(10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&1100&')
    assert_equal(b, '&0xc&')
    tmp = decimal_to_bcd(10000)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'BCD of Decimal Number &4160 = &')
    assert_equal(b, '&1040&')
    tmp = decimal_to_binary(99)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Binary of &21 = &')
    assert_equal(b, '&10101&')
    tmp = decimal_to_hexadeci(1000)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Hexadecimal of &384 = &')
    assert_equal(b, '&0x180&')
    tmp = decimal_to_octal(4096)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'The decimal number &3762& in octal is: ')
    assert_equal(b, '&0o7262&')
    tmp = fibonacci_series(1)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'The Fibonacci Series of the first &18& numbers is ?')
    assert_equal(b, '&0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597&')
    tmp = modulo_division(99, 99)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, '&77& % &52& = &')
    assert_equal(b, '&25&')
    tmp = nth_fibonacci_number(100)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'What is the 63th Fibonacci number?')
    assert_equal(b, '&6557470319842&')
    ### --- BLOCK END 82



def test_3():
    ### --- BLOCK BEGIN 83
    tmp = combinations(20)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Find the number of combinations from &14& objects picked &8& at a time.')
    assert_equal(b, '&3003&')
    tmp = conditional_probability()
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Someone tested positive for a nasty disease which only &0.61&% of the population have. Test sensitivity (true positive) is equal to &SN=99.29&% whereas test specificity (true negative) &SP=94.91&%. What is the probability that this guy really has that disease?')
    assert_equal(b, '&10.69&%')
    tmp = confidence_interval()
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'The confidence interval for sample &[229, 231, 242, 225, 252, 290, 270, 227, 231, 258, 296, 243, 247, 232, 276, 272, 237, 240, 235, 220, 238, 292, 289]& with &80&% confidence is')
    assert_equal(b, '&(257.29, 244.62)&')
    tmp = data_summary(15, 5, 50)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Find the mean,standard deviation and variance for the data &40, 29, 33, 26, 26, 36, 7, 43, 16, 25, 17, 25, 28, 11, 13&')
    assert_equal(b, 'The Mean is &25.0&, Standard Deviation is &104.67&, Variance is &10.23&')
    tmp = dice_sum_probability(3)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'If &2& dice are rolled at the same time, the probability of getting a sum of &2 =&')
    assert_equal(b, '\\frac{1}{36}')
    tmp = mean_median(10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Given the series of numbers &[2, 2, 11, 16, 19, 25, 26, 38, 46, 78]&. Find the arithmatic mean and median of the series')
    assert_equal(b, 'Arithmetic mean of the series is &26.3& and arithmetic median of this series is &22.0&')
    tmp = permutation(20)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Number of Permutations from &12& objects picked &8& at a time is: ')
    assert_equal(b, '&19958400&')
    ### --- BLOCK END 83



def test_4():
    ### --- BLOCK BEGIN 84
    tmp = angle_btw_vectors(20)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'angle between the vectors [829.89, 304.8, 293.49, 934.28, 906.11, 472.69, 173.37, 99.0, 290.11] and [311.65, 419.22, 249.45, 520.14, 899.08, 693.34, 270.07, 307.76, 578.14] is:')
    assert_equal(b, '0.49 radians')
    tmp = angle_regular_polygon(3, 20)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Find the angle of a regular polygon with 20 sides')
    assert_equal(b, '162.0')
    tmp = arc_length(49, 359)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Given radius, 22 and angle, 169. Find the arc length of the angle.')
    assert_equal(b, 'Arc length of the angle = 64.89134')
    tmp = area_of_circle(100)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Area of circle with radius 32=')
    assert_equal(b, '3216.99')
    tmp = area_of_circle_given_center_and_point(10, 10)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Area of circle with center (5,-3) and passing through (9.32, 3.7300000000000004) is')
    assert_equal(b, '201.06')
    tmp = area_of_triangle(20, 20)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Area of triangle with side lengths: 8, 5, 7 = ')
    assert_equal(b, '17.32')
    tmp = circumference(100)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Circumference of circle with radius 92 = ')
    assert_equal(b, '578.05')
    tmp = complementary_and_supplementary_angle(180, 90)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'The complementary angle of 70 =')
    assert_equal(b, '20')
    tmp = curved_surface_area_cylinder(49, 99)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'What is the curved surface area of a cylinder of radius, 26 and height, 62?')
    assert_equal(b, '10128.49')
    tmp = degree_to_rad(360)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Angle 167 degrees in radians is: ')
    assert_equal(b, '2.91')
    tmp = equation_of_line_from_two_points(20, -20)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'What is the equation of the line between points (-1,-19) and (7,14) in slope-intercept form?')
    assert_equal(b, '8y = 33x -119')
    tmp = fourth_angle_of_quadrilateral(180)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Fourth angle of quadrilateral with angles 44 , 89, 56 =')
    assert_equal(b, '171')
    tmp = pythagorean_theorem(20)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'What is the hypotenuse of a right triangle given the other two sides have lengths 9 and 11?')
    assert_equal(b, '14.21')
    tmp = radian_to_deg(6.28)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Angle 0.93 radians in degrees is: ')
    assert_equal(b, '53.29')
    tmp = sector_area(49, 359)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'What is the area of a sector with radius 10 and angle 214 degrees?')
    assert_equal(b, '186.75')
    tmp = sum_of_polygon_angles(12)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'What is the sum of interior angles of a polygon with 3 sides?')
    assert_equal(b, '180')
    tmp = surface_area_cone(20, 50, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Surface area of cone with height = 6m and radius = 1m is')
    assert_equal(b, '22 m^2')
    tmp = surface_area_cube(20, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Surface area of cube with side = 6m is')
    assert_equal(b, '216 m^2')
    tmp = surface_area_cuboid(20, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Surface area of cuboid with sides of lengths: 4m, 4m, 1m is')
    assert_equal(b, '48 m^2')
    tmp = surface_area_cylinder(20, 50, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Surface area of cylinder with height = 24m and radius = 16m is')
    assert_equal(b, '4021 m^2')
    tmp = surface_area_pyramid('m')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Surface area of pyramid with base length = 40m, base width = 32m, and height = 12m is')
    assert_equal(b, '2560 m^2')
    tmp = surface_area_sphere(20, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Surface area of a sphere with radius = 2m is')
    assert_equal(b, '50.27 m^2')
    tmp = third_angle_of_triangle(89)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Third angle of triangle with angles 21 and 26 = ')
    assert_equal(b, '133')
    tmp = valid_triangle(50)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Does triangle with sides 32, 39 and 50 exist?')
    assert_equal(b, 'yes')
    tmp = volume_cone(20, 50, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Volume of cone with height = 25m and radius = 11m is')
    assert_equal(b, '3167 m^3')
    tmp = volume_cube(20, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Volume of cube with a side length of 12m is')
    assert_equal(b, '1728 m^3')
    tmp = volume_cuboid(20, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Volume of cuboid with sides = 19m, 20m, 20m is')
    assert_equal(b, '7600 m^3')
    tmp = volume_cylinder(20, 50, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Volume of cylinder with height = 33m and radius = 5m is')
    assert_equal(b, '2591 m^3')
    tmp = volume_cone_frustum(20, 20, 50, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Volume of frustum with height = 30m and r1 = 6m is and r2 = 7m is ')
    assert_equal(b, '3989.82 m^3')
    tmp = volume_hemisphere(100)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Volume of hemisphere with radius 65 m = ')
    assert_equal(b, '575173.25 m^3')
    tmp = volume_pyramid(20, 20, 50, 'm')
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Volume of pyramid with base length = 15 m, base width = 6 m and height = 36 m is')
    assert_equal(b, '1080.0 m^3')
    tmp = volume_sphere(100)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'Volume of sphere with radius 27 m = ')
    assert_equal(b, '82447.96 m^3')
    tmp = perimeter_of_polygons(12, 120)
    a = tmp[0]
    b = tmp[1]
    assert_equal(a, 'The perimeter of a 10 sided polygon with lengths of 66, 97, 50, 14, 62, 52, 107, 82, 58, 101cm is: ')
    assert_equal(b, '689')
    ### --- BLOCK END 84



def test():
    ### --- BLOCK BEGIN 85
    test_1()
    user_reset_seed()
    test_2()
    user_reset_seed()
    test_3()
    user_reset_seed()
    test_4()
    user_reset_seed()
    additional_tests()
    ### --- BLOCK END 85



def assert_iter_equal(a, b):
    ### --- BLOCK BEGIN 86
    for i, j in zip(a, b):
        assert_equal(i, j)
    return True
    ### --- BLOCK END 86



def additional_tests():
    ### --- BLOCK BEGIN 87
    tmp = addition(10, 20)
    assert_iter_equal(tmp, ['&4+5=&', '&9&'])
    for i in range(4):
        tmp = compare_fractions(2)
    assert_iter_equal(tmp, ['Which symbol represents the comparison between &\\frac{1}{2}& and &\\frac{1}{2}&?', '='])
    for i in range(3):
        tmp = divide_fractions(2)
    assert_iter_equal(tmp, ['&\\frac{2}{1}\\div\\frac{1}{2}=&', '&\\frac{4}{1}&'])
    for i in range(5):
        tmp = fraction_multiplication(2)
    assert_iter_equal(tmp, ['&\\frac{2}{1}\\cdot\\frac{2}{1}=&', '&\\frac{4}{1}&'])
    tmp = is_composite(4)
    assert_iter_equal(tmp, ['Is &4& composite?', 'Yes'])
    tmp = is_prime(2)
    assert_iter_equal(tmp, ['Is &2& prime?', 'Yes'])
    tmp = is_prime(3)
    assert_iter_equal(tmp, ['Is &3& prime?', 'Yes'])
    for i in range(4):
        tmp = is_prime(36)
    assert_iter_equal(tmp, ['Is &11& prime?', 'Yes'])
    tmp = dice_sum_probability(1)
    assert_iter_equal(tmp, ['If &1& dice are rolled at the same time, the probability of getting a sum of &1 =&', '\\frac{1}{6}'])
    for i in range(4):
        tmp = dice_sum_probability(3)
    assert_iter_equal(tmp, ['If &3& dice are rolled at the same time, the probability of getting a sum of &9 =&', '\\frac{25}{216}'])
    tmp = complementary_and_supplementary_angle(2, 3)
    tmp = complementary_and_supplementary_angle(2, 4)
    tmp = complementary_and_supplementary_angle(2, 5)
    tmp = complementary_and_supplementary_angle(2, 6)
    assert_iter_equal(tmp, ['The supplementary angle of 2 =', '178'])
    tmp = equation_of_line_from_two_points(3, 2)
    tmp = equation_of_line_from_two_points(4, 2)
    tmp = equation_of_line_from_two_points(6, 6)
    tmp = equation_of_line_from_two_points(8, 2)
    tmp = equation_of_line_from_two_points(10, 2)
    tmp = equation_of_line_from_two_points(16, 4)
    tmp = equation_of_line_from_two_points(36, 4)
    assert_iter_equal(tmp, ['What is the equation of the line between points (5,34) and (7,4) in slope-intercept form?', 'y = -15x + 109'])
    for i in range(15):
        tmp = equation_of_line_from_two_points(1, 0)
    assert_iter_equal(tmp, ['What is the equation of the line between points (0,1) and (1,1) in slope-intercept form?', 'y = 1'])
    tmp = is_composite(0)
    assert_iter_equal(tmp, ['Is &1& composite?', 'No'])
    ### --- BLOCK END 87



### Global Begin

### --- BLOCK BEGIN 0
test()

### --- BLOCK END 0
