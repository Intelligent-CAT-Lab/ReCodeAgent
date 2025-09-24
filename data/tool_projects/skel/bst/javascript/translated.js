
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

function Node(param_0, param_1){
    function __init__(label, parent){
        /// --- BLOCK BEGIN 1
class_var.label = label;
class_var.parent = parent;
class_var.left = null;
class_var.right = null;    
        /// --- BLOCK END 1
    
    }
    
    var class_var = SkelClass('Node');
    class_var.__init__ = __init__;
    __init__(param_0, param_1);
    return class_var;
}


function BinarySearchTree(){
    function __init__(){
        /// --- BLOCK BEGIN 2
class_var.root = null;
return null;    
        /// --- BLOCK END 2
    
    }
    
    function empty(){
        /// --- BLOCK BEGIN 3
class_var.root = null;
return null;    
        /// --- BLOCK END 3
    
    }
    
    function is_empty(){
        /// --- BLOCK BEGIN 4
return class_var.root === null;    
        /// --- BLOCK END 4
    
    }
    
    function put(label){
        /// --- BLOCK BEGIN 5
class_var.root = class_var._put(class_var.root, label, null);    
        /// --- BLOCK END 5
    
    }
    
    function _put(node, label, parent){
        /// --- BLOCK BEGIN 6
        if (node === null) {
            node = new Node(label, parent);
        } else {
            if (label < node.label) {
                node.left = class_var._put(node.left, label, node);
            } else if (label > node.label) {
                node.right = class_var._put(node.right, label, node);
            } else {
                var msg = "Node with label " + label + " already exists";
                throw new Exception(msg);
            }
        }
        return node;    
        /// --- BLOCK END 6
    
    }
    
    function search(label){
        /// --- BLOCK BEGIN 7
return class_var._search(class_var.root, label);    
        /// --- BLOCK END 7
    
    }
    
    function _search(node, label){
        /// --- BLOCK BEGIN 8
        if (node === null) {
            var msg = "Node with label " + label + " does not exist";
            throw new Exception(msg);
        } else {
            if (label < node.label) {
                node = class_var._search(node.left, label);
            } else if (label > node.label) {
                node = class_var._search(node.right, label);
            }
        }
        return node;    
        /// --- BLOCK END 8
    
    }
    
    function remove(label){
        /// --- BLOCK BEGIN 9
var node = class_var.search(label);
if (node.right && node.left) {
    var lowest_node = class_var._get_lowest_node(node.right);
    lowest_node.left = node.left;
    lowest_node.right = node.right;
    node.left.parent = lowest_node;
    if (node.right) {
        node.right.parent = lowest_node;
    }
    class_var._reassign_nodes(node, lowest_node);
} else if (!node.right && node.left) {
    class_var._reassign_nodes(node, node.left);
} else if (node.right && !node.left) {
    class_var._reassign_nodes(node, node.right);
} else {
    class_var._reassign_nodes(node, null);
}    
        /// --- BLOCK END 9
    
    }
    
    function _reassign_nodes(node, new_children){
        /// --- BLOCK BEGIN 10
if (new_children !== null) {
    new_children.parent = node.parent;
}
if (node.parent !== null) {
    if (node.parent.right === node) {
        node.parent.right = new_children;
    } else {
        node.parent.left = new_children;
    }
} else {
    class_var.root = new_children;
}
return null;
    
        /// --- BLOCK END 10
    
    }
    
    function _get_lowest_node(node){
        /// --- BLOCK BEGIN 11
        var lowest_node;
        if (node.left) {
            lowest_node = class_var._get_lowest_node(node.left);
        } else {
            lowest_node = node;
            class_var._reassign_nodes(node, node.right);
        }
        return lowest_node;    
        /// --- BLOCK END 11
    
    }
    
    function exists(label){
        /// --- BLOCK BEGIN 12
try {
    class_var.search(label);
    return true;
} catch (exception) {
    return false;
}    
        /// --- BLOCK END 12
    
    }
    
    function get_max_label(){
        /// --- BLOCK BEGIN 13
if (class_var.root === null) {
    throw new Exception("Binary search tree is empty");
}
var node = class_var.root;
while (node.right !== null) {
    node = node.right;
}
return node.label;    
        /// --- BLOCK END 13
    
    }
    
    function get_min_label(){
        /// --- BLOCK BEGIN 14
if (class_var.root === null) {
    throw new Error("Binary search tree is empty");
}
var node = class_var.root;
while (node.left !== null) {
    node = node.left;
}
return node.label;
    
        /// --- BLOCK END 14
    
    }
    
    function inorder_traversal(){
        /// --- BLOCK BEGIN 15
return class_var._inorder_traversal(class_var.root);    
        /// --- BLOCK END 15
    
    }
    
    function* _inorder_traversal(node){
        /// --- BLOCK BEGIN 16
if (node !== null) {
    yield* class_var._inorder_traversal(node.left);
    yield node;
    yield* class_var._inorder_traversal(node.right);
}    
        /// --- BLOCK END 16
    
    }
    
    function preorder_traversal(){
        /// --- BLOCK BEGIN 17
return class_var._preorder_traversal(class_var.root);    
        /// --- BLOCK END 17
    
    }
    
    function* _preorder_traversal(node){
        /// --- BLOCK BEGIN 18
if (node !== null) {
    yield node;
    yield* class_var._preorder_traversal(node.left);
    yield* class_var._preorder_traversal(node.right);
}    
        /// --- BLOCK END 18
    
    }
    
    var class_var = SkelClass('BinarySearchTree');
    class_var.__init__ = __init__;
    class_var.empty = empty;
    class_var.is_empty = is_empty;
    class_var.put = put;
    class_var._put = _put;
    class_var.search = search;
    class_var._search = _search;
    class_var.remove = remove;
    class_var._reassign_nodes = _reassign_nodes;
    class_var._get_lowest_node = _get_lowest_node;
    class_var.exists = exists;
    class_var.get_max_label = get_max_label;
    class_var.get_min_label = get_min_label;
    class_var.inorder_traversal = inorder_traversal;
    class_var._inorder_traversal = _inorder_traversal;
    class_var.preorder_traversal = preorder_traversal;
    class_var._preorder_traversal = _preorder_traversal;
    __init__();
    return class_var;
}


function _get_binary_search_tree(){
    /// --- BLOCK BEGIN 19
var t = new BinarySearchTree();
    t.put(8);
    t.put(3);
    t.put(6);
    t.put(1);
    t.put(10);
    t.put(14);
    t.put(13);
    t.put(4);
    t.put(7);
    t.put(5);
    return t;

    /// --- BLOCK END 19

}
