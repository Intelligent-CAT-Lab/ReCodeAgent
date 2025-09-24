
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

function RedBlackTree(param_0, param_1, param_2, param_3, param_4){
    function __init__(label, color, parent, left, right){
        /// --- BLOCK BEGIN 1
class_var.label = label;
class_var.parent = parent;
class_var.left = left;
class_var.right = right;
class_var.color = color;    
        /// --- BLOCK END 1
    
    }
    
    function rotate_left(){
        /// --- BLOCK BEGIN 2
var parent = class_var.parent;
    var right = class_var.right;
    if (right === null) {
        return class_var;
    }
    class_var.right = right.left;
    if (class_var.right) {
        class_var.right.parent = class_var;
    }
    class_var.parent = right;
    right.left = class_var;
    if (parent !== null) {
        if ((parent.left && parent.left.__eq__ && parent.left.__eq__(class_var)) || (!parent.left.__eq__ && parent.left === class_var)) {
            parent.left = right;
        } else {
            parent.right = right;
        }
    }
    right.parent = parent;
    return right;
    
        /// --- BLOCK END 2
    
    }
    
    function rotate_right(){
        /// --- BLOCK BEGIN 3
        if (class_var.left === null) {
            return class_var;
        }
        var parent = class_var.parent;
        var left = class_var.left;
        class_var.left = left.right;
        if (class_var.left) {
            class_var.left.parent = class_var;
        }
        class_var.parent = left;
        left.right = class_var;
        if (parent !== null) {
            if (parent.right === class_var) {
                parent.right = left;
            } else {
                parent.left = left;
            }
        }
        left.parent = parent;
        return left;    
        /// --- BLOCK END 3
    
    }
    
    function insert(label){
        /// --- BLOCK BEGIN 4
if (class_var.label === null) {
    // Only possible with an empty tree
    class_var.label = label;
    return class_var;
}
if (class_var.label === label) {
    return class_var;
} else if (class_var.label > label) {
    if (class_var.left) {
        class_var.left.insert(label);
    } else {
        class_var.left = new RedBlackTree(label, 1, class_var, null, null);
        class_var.left._insert_repair();
    }
} else {
    if (class_var.right) {
        class_var.right.insert(label);
    } else {
        class_var.right = new RedBlackTree(label, 1, class_var, null, null);
        class_var.right._insert_repair();
    }
}
return class_var.parent || class_var;    
        /// --- BLOCK END 4
    
    }
    
    function _insert_repair(){
        /// --- BLOCK BEGIN 5
        if (class_var.parent === null) {
        // This node is the root, so it just needs to be black
            class_var.color = 0;
        } else if (get_color(class_var.parent) === 0) {
        // If the parent is black, then it just needs to be red
            class_var.color = 1;
        } else {
            var uncle = class_var.parent.sibling();
            if (get_color(uncle) === 0) {
                if (class_var.is_left() && class_var.parent.is_right()) {
                    class_var.parent.rotate_right();
                    if (class_var.right) {
                        class_var.right._insert_repair();
                    }
                } else if (class_var.is_right() && class_var.parent.is_left()) {
                    class_var.parent.rotate_left();
                    if (class_var.left) {
                        class_var.left._insert_repair();
                    }
                } else if (class_var.is_left()) {
                    if (class_var.grandparent()) {
                        class_var.grandparent().rotate_right();
                        class_var.parent.color = 0;
                    }
                    if (class_var.parent.right) {
                        class_var.parent.right.color = 1;
                    }
                } else {
                    if (class_var.grandparent()) {
                        class_var.grandparent().rotate_left();
                        class_var.parent.color = 0;
                    }
                    if (class_var.parent.left) {
                        class_var.parent.left.color = 1;
                    }
                }
            } else {
                class_var.parent.color = 0;
                if (uncle && class_var.grandparent()) {
                    uncle.color = 0;
                    class_var.grandparent().color = 1;
                    class_var.grandparent()._insert_repair();
                }
            }
        }    
        /// --- BLOCK END 5
    
    }
    
    function remove(label){
        /// --- BLOCK BEGIN 6
        if (class_var.label === label) {
            if (class_var.left && class_var.right) {
                var value = class_var.left.get_max();
                if (value !== null) {
                    class_var.label = value;
                    class_var.left.remove(value);
                }
            } else {
                var child = class_var.left || class_var.right;
                if (class_var.color === 1) {
                    if (class_var.parent) {
                        if (class_var.is_left()) {
                            class_var.parent.left = null;
                        } else {
                            class_var.parent.right = null;
                        }
                    }
                } else {
                    if (child === null) {
                        if (class_var.parent === null) {
                            return new RedBlackTree(null);
                        } else {
                            class_var._remove_repair();
                            if (class_var.is_left()) {
                                class_var.parent.left = null;
                            } else {
                                class_var.parent.right = null;
                            }
                            class_var.parent = null;
                        }
                    } else {
                        class_var.label = child.label;
                        class_var.left = child.left;
                        class_var.right = child.right;
                        if (class_var.left) {
                            class_var.left.parent = class_var;
                        }
                        if (class_var.right) {
                            class_var.right.parent = class_var;
                        }
                    }
                }
            }
        } else if (class_var.label !== null && class_var.label > label) {
            if (class_var.left) {
                class_var.left.remove(label);
            }
        } else {
            if (class_var.right) {
                class_var.right.remove(label);
            }
        }
        return class_var.parent || class_var;    
        /// --- BLOCK END 6
    
    }
    
    function _remove_repair(){
        /// --- BLOCK BEGIN 7
        if (
        class_var.parent === null
        || class_var.sibling() === null
        || class_var.parent.sibling() === null
        || class_var.grandparent() === null
        ) {
            return;
        }
        if (get_color(class_var.sibling()) === 1) {
            class_var.sibling().color = 0;
            class_var.parent.color = 1;
            if (class_var.is_left()) {
                class_var.parent.rotate_left();
            } else {
                class_var.parent.rotate_right();
            }
        }
        if (
        get_color(class_var.parent) === 0
        && get_color(class_var.sibling()) === 0
        && get_color(class_var.sibling().left) === 0
        && get_color(class_var.sibling().right) === 0
        ) {
            class_var.sibling().color = 1;
            class_var.parent._remove_repair();
            return;
        }
        if (
        get_color(class_var.parent) === 1
        && get_color(class_var.sibling()) === 0
        && get_color(class_var.sibling().left) === 0
        && get_color(class_var.sibling().right) === 0
        ) {
            class_var.sibling().color = 1;
            class_var.parent.color = 0;
            return;
        }
        if (
        class_var.is_left()
        && get_color(class_var.sibling()) === 0
        && get_color(class_var.sibling().right) === 0
        && get_color(class_var.sibling().left) === 1
        ) {
            class_var.sibling().rotate_right();
            class_var.sibling().color = 0;
            if (class_var.sibling().right) {
                class_var.sibling().right.color = 1;
            }
        }
        if (
        class_var.is_right()
        && get_color(class_var.sibling()) === 0
        && get_color(class_var.sibling().right) === 1
        && get_color(class_var.sibling().left) === 0
        ) {
            class_var.sibling().rotate_left();
            class_var.sibling().color = 0;
            if (class_var.sibling().left) {
                class_var.sibling().left.color = 1;
            }
        }
        if (
        class_var.is_left()
        && get_color(class_var.sibling()) === 0
        && get_color(class_var.sibling().right) === 1
        ) {
            class_var.parent.rotate_left();
            class_var.grandparent().color = class_var.parent.color;
            class_var.parent.color = 0;
            class_var.parent.sibling().color = 0;
        }
        if (
        class_var.is_right()
        && get_color(class_var.sibling()) === 0
        && get_color(class_var.sibling().left) === 1
        ) {
            class_var.parent.rotate_right();
            class_var.grandparent().color = class_var.parent.color;
            class_var.parent.color = 0;
            class_var.parent.sibling().color = 0;
        }    
        /// --- BLOCK END 7
    
    }
    
    function check_color_properties(){
        /// --- BLOCK BEGIN 8
        if (class_var.color) {
        // The root was red
            console.log("Property 2");
            return false;
        }
        // Property 3 does not need to be checked, because None is assumed
        // to be black and is all the leaves.
        // Property 4
        if (!class_var.check_coloring()) {
            console.log("Property 4");
            return false;
        }
        // Property 5
        if (class_var.black_height() === null) {
            console.log("Property 5");
            return false;
        }
        // All properties were met
        return true;    
        /// --- BLOCK END 8
    
    }
    
    function check_coloring(){
        /// --- BLOCK BEGIN 9
        if (class_var.color === 1 && [get_color(class_var.left), get_color(class_var.right)].includes(1)) {
            return false;
        }
        if (class_var.left && !class_var.left.check_coloring()) {
            return false;
        }
        if (class_var.right && !class_var.right.check_coloring()) {
            return false;
        }
        return true;    
        /// --- BLOCK END 9
    
    }
    
    function black_height(){
        /// --- BLOCK BEGIN 10
if (class_var === null || class_var.left === null || class_var.right === null) {
    // If we're already at a leaf, there is no path
    return 1;
}
var left = class_var.left.black_height();
var right = class_var.right.black_height();
if (left === null || right === null) {
    // There are issues with coloring below children nodes
    return null;
}
if (left !== right) {
    // The two children have unequal depths
    return null;
}
// Return the black depth of children, plus one if this node is
// black
return left + (1 - class_var.color);
    
        /// --- BLOCK END 10
    
    }
    
    function __contains__(label){
        /// --- BLOCK BEGIN 11
return class_var.search(label) !== null;    
        /// --- BLOCK END 11
    
    }
    
    function search(label){
        /// --- BLOCK BEGIN 12
if (class_var.label === label) {
    return class_var;
} else if (class_var.label !== null && label > class_var.label) {
    if (class_var.right === null) {
        return null;
    } else {
        return class_var.right.search(label);
    }
} else {
    if (class_var.left === null) {
        return null;
    } else {
        return class_var.left.search(label);
    }
}    
        /// --- BLOCK END 12
    
    }
    
    function floor(label){
        /// --- BLOCK BEGIN 13
        if (class_var.label === label) {
            return class_var.label;
        } else if (class_var.label !== null && class_var.label > label) {
            if (class_var.left) {
                return class_var.left.floor(label);
            } else {
                return null;
            }
        } else {
            if (class_var.right) {
                var attempt = class_var.right.floor(label);
                if (attempt !== null) {
                    return attempt;
                }
            }
            return class_var.label;
        }    
        /// --- BLOCK END 13
    
    }
    
    function ceil(label){
        /// --- BLOCK BEGIN 14
        if (class_var.label === label) {
            return class_var.label;
        } else if (class_var.label !== null && class_var.label < label) {
            if (class_var.right) {
                return class_var.right.ceil(label);
            } else {
                return null;
            }
        } else {
            if (class_var.left) {
                var attempt = class_var.left.ceil(label);
                if (attempt !== null) {
                    return attempt;
                }
            }
            return class_var.label;
        }    
        /// --- BLOCK END 14
    
    }
    
    function get_max(){
        /// --- BLOCK BEGIN 15
if (class_var.right) {
    // Go as far right as possible
    return class_var.right.get_max();
} else {
    return class_var.label;
}    
        /// --- BLOCK END 15
    
    }
    
    function get_min(){
        /// --- BLOCK BEGIN 16
if (class_var.left) {
    // Go as far left as possible
    return class_var.left.get_min();
} else {
    return class_var.label;
}    
        /// --- BLOCK END 16
    
    }
    
    function grandparent(){
        /// --- BLOCK BEGIN 17
if (class_var.parent === null) {
    return null;
} else {
    return class_var.parent.parent;
}    
        /// --- BLOCK END 17
    
    }
    
    function sibling(){
        /// --- BLOCK BEGIN 18
if (class_var.parent === null) {
    return null;
} else if (class_var.parent.left === class_var) {
    return class_var.parent.right;
} else {
    return class_var.parent.left;
}    
        /// --- BLOCK END 18
    
    }
    
    function is_left(){
        /// --- BLOCK BEGIN 19
if (class_var.parent === null) {
    return false;
}
return class_var.parent.left === class_var.parent.left && class_var.parent.left === class_var;
    
        /// --- BLOCK END 19
    
    }
    
    function is_right(){
        /// --- BLOCK BEGIN 20
if (class_var.parent === null) {
    return false;
}
return class_var.parent.right === class_var;
    
        /// --- BLOCK END 20
    
    }
    
    function __bool__(){
        /// --- BLOCK BEGIN 21
return true;    
        /// --- BLOCK END 21
    
    }
    
    function __len__(){
        /// --- BLOCK BEGIN 22
var ln = 1;
        if (class_var.left) {
            ln += class_var.left.length;
        }
        if (class_var.right) {
            ln += class_var.right.length;
        }
        return ln;
    
        /// --- BLOCK END 22
    
    }
    
    function* preorder_traverse(){
        /// --- BLOCK BEGIN 23
    yield class_var.label;
    if (class_var.left) {
        yield* class_var.left.preorder_traverse();
    }
    if (class_var.right) {
        yield* class_var.right.preorder_traverse();
    }    
        /// --- BLOCK END 23
    
    }
    
    function* inorder_traverse(){
        /// --- BLOCK BEGIN 24
if (class_var.left) {
    yield* class_var.left.inorder_traverse();
}
yield class_var.label;
if (class_var.right) {
    yield* class_var.right.inorder_traverse();
}    
        /// --- BLOCK END 24
    
    }
    
    function* postorder_traverse(){
        /// --- BLOCK BEGIN 25
if (class_var.left) {
    yield* class_var.left.postorder_traverse();
}
if (class_var.right) {
    yield* class_var.right.postorder_traverse();
}
yield class_var.label;    
        /// --- BLOCK END 25
    
    }
    
    function __eq__(other){
        /// --- BLOCK BEGIN 26
        if (!user_check_type(other, RedBlackTree)) {
            return NotImplemented;
        }
        if (class_var.label === other.label) {
            return ((class_var.left && class_var.left.__eq__ && class_var.left.__eq__(other.left)) || (!class_var.left || !class_var.left.__eq__ && class_var.left === other.left)) && ((class_var.right && class_var.right.__eq__ && class_var.right.__eq__(other.right)) || (!class_var.right || !class_var.right.__eq__ && class_var.right === other.right));
        } else {
            return false;
        }    
        /// --- BLOCK END 26
    
    }
    
    var class_var = SkelClass('RedBlackTree');
    class_var.__init__ = __init__;
    class_var.rotate_left = rotate_left;
    class_var.rotate_right = rotate_right;
    class_var.insert = insert;
    class_var._insert_repair = _insert_repair;
    class_var.remove = remove;
    class_var._remove_repair = _remove_repair;
    class_var.check_color_properties = check_color_properties;
    class_var.check_coloring = check_coloring;
    class_var.black_height = black_height;
    class_var.__contains__ = __contains__;
    class_var.search = search;
    class_var.floor = floor;
    class_var.ceil = ceil;
    class_var.get_max = get_max;
    class_var.get_min = get_min;
    class_var.grandparent = grandparent;
    class_var.sibling = sibling;
    class_var.is_left = is_left;
    class_var.is_right = is_right;
    class_var.__bool__ = __bool__;
    class_var.__len__ = __len__;
    class_var.preorder_traverse = preorder_traverse;
    class_var.inorder_traverse = inorder_traverse;
    class_var.postorder_traverse = postorder_traverse;
    class_var.__eq__ = __eq__;
    __init__(param_0, param_1, param_2, param_3, param_4);
    return class_var;
}


function get_color(node){
    /// --- BLOCK BEGIN 27
if (node === null) {
    return 0;
} else {
    return node.color;
}
    /// --- BLOCK END 27

}
