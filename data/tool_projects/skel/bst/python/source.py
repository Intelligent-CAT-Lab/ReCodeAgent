from __future__ import annotations

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

"""
This is a python3 implementation of binary search tree using recursion

To run tests:
python -m unittest binary_search_tree_recursive.py

To run an example:
python binary_search_tree_recursive.py
"""

from collections.abc import Iterator

def Node(param_0, param_1):
    def __init__(label, parent):
        ### --- BLOCK BEGIN 1
        class_var.label = label
        class_var.parent = parent
        class_var.left = None
        class_var.right = None
        ### --- BLOCK END 1
    
    
    
    class_var = SkelClass('Node')
    class_var.__init__ = __init__
    __init__(param_0, param_1)
    return class_var


def BinarySearchTree():
    def __init__():
        ### --- BLOCK BEGIN 2
        class_var.root = None
        ### --- BLOCK END 2
    
    
    
    def empty():
        """
            Empties the tree
    
            >>> t = BinarySearchTree()
            >>> assert t.root is None
            >>> t.put(8)
            >>> assert t.root is not None
            """
        ### --- BLOCK BEGIN 3
        class_var.root = None
        ### --- BLOCK END 3
    
    
    
    def is_empty():
        """
            Checks if the tree is empty
    
            >>> t = BinarySearchTree()
            >>> t.is_empty()
            True
            >>> t.put(8)
            >>> t.is_empty()
            False
            """
        ### --- BLOCK BEGIN 4
        return class_var.root is None
        ### --- BLOCK END 4
    
    
    
    def put(label):
        """
            Put a new node in the tree
    
            >>> t = BinarySearchTree()
            >>> t.put(8)
            >>> assert t.root.parent is None
            >>> assert t.root.label == 8
    
            >>> t.put(10)
            >>> assert t.root.right.parent == t.root
            >>> assert t.root.right.label == 10
    
            >>> t.put(3)
            >>> assert t.root.left.parent == t.root
            >>> assert t.root.left.label == 3
            """
        ### --- BLOCK BEGIN 5
        class_var.root = class_var._put(class_var.root, label, None)
        ### --- BLOCK END 5
    
    
    
    def _put(node, label, parent):
        ### --- BLOCK BEGIN 6
        if node is None:
            node = Node(label, parent)
        else:
            if label < node.label:
                node.left = class_var._put(node.left, label, node)
            elif label > node.label:
                node.right = class_var._put(node.right, label, node)
            else:
                msg = f"Node with label {label} already exists"
                raise Exception(msg)
        return node
        ### --- BLOCK END 6
    
    
    
    def search(label):
        """
            Searches a node in the tree
    
            >>> t = BinarySearchTree()
            >>> t.put(8)
            >>> t.put(10)
            >>> node = t.search(8)
            >>> assert node.label == 8
    
            >>> node = t.search(3)
            Traceback (most recent call last):
                ...
            Exception: Node with label 3 does not exist
            """
        ### --- BLOCK BEGIN 7
        return class_var._search(class_var.root, label)
        ### --- BLOCK END 7
    
    
    
    def _search(node, label):
        ### --- BLOCK BEGIN 8
        if node is None:
            msg = f"Node with label {label} does not exist"
            raise Exception(msg)
        else:
            if label < node.label:
                node = class_var._search(node.left, label)
            elif label > node.label:
                node = class_var._search(node.right, label)
        return node
        ### --- BLOCK END 8
    
    
    
    def remove(label):
        """
            Removes a node in the tree
    
            >>> t = BinarySearchTree()
            >>> t.put(8)
            >>> t.put(10)
            >>> t.remove(8)
            >>> assert t.root.label == 10
    
            >>> t.remove(3)
            Traceback (most recent call last):
                ...
            Exception: Node with label 3 does not exist
            """
        ### --- BLOCK BEGIN 9
        node = class_var.search(label)
        if node.right and node.left:
            lowest_node = class_var._get_lowest_node(node.right)
            lowest_node.left = node.left
            lowest_node.right = node.right
            node.left.parent = lowest_node
            if node.right:
                node.right.parent = lowest_node
            class_var._reassign_nodes(node, lowest_node)
        elif not node.right and node.left:
            class_var._reassign_nodes(node, node.left)
        elif node.right and not node.left:
            class_var._reassign_nodes(node, node.right)
        else:
            class_var._reassign_nodes(node, None)
        ### --- BLOCK END 9
    
    
    
    def _reassign_nodes(node, new_children):
        ### --- BLOCK BEGIN 10
        if new_children:
            new_children.parent = node.parent
        if node.parent:
            if node.parent.right == node:
                node.parent.right = new_children
            else:
                node.parent.left = new_children
        else:
            class_var.root = new_children
        ### --- BLOCK END 10
    
    
    
    def _get_lowest_node(node):
        ### --- BLOCK BEGIN 11
        if node.left:
            lowest_node = class_var._get_lowest_node(node.left)
        else:
            lowest_node = node
            class_var._reassign_nodes(node, node.right)
        return lowest_node
        ### --- BLOCK END 11
    
    
    
    def exists(label):
        """
            Checks if a node exists in the tree
    
            >>> t = BinarySearchTree()
            >>> t.put(8)
            >>> t.put(10)
            >>> t.exists(8)
            True
    
            >>> t.exists(3)
            False
            """
        ### --- BLOCK BEGIN 12
        try:
            class_var.search(label)
            return True
        except Exception:
            return False
        ### --- BLOCK END 12
    
    
    
    def get_max_label():
        """
            Gets the max label inserted in the tree
    
            >>> t = BinarySearchTree()
            >>> t.get_max_label()
            Traceback (most recent call last):
                ...
            Exception: Binary search tree is empty
    
            >>> t.put(8)
            >>> t.put(10)
            >>> t.get_max_label()
            10
            """
        ### --- BLOCK BEGIN 13
        if class_var.root is None:
            raise Exception("Binary search tree is empty")
        node = class_var.root
        while node.right is not None:
            node = node.right
        return node.label
        ### --- BLOCK END 13
    
    
    
    def get_min_label():
        """
            Gets the min label inserted in the tree
    
            >>> t = BinarySearchTree()
            >>> t.get_min_label()
            Traceback (most recent call last):
                ...
            Exception: Binary search tree is empty
    
            >>> t.put(8)
            >>> t.put(10)
            >>> t.get_min_label()
            8
            """
        ### --- BLOCK BEGIN 14
        if class_var.root is None:
            raise Exception("Binary search tree is empty")
        node = class_var.root
        while node.left is not None:
            node = node.left
        return node.label
        ### --- BLOCK END 14
    
    
    
    def inorder_traversal():
        """
            Return the inorder traversal of the tree
    
            >>> t = BinarySearchTree()
            >>> [i.label for i in t.inorder_traversal()]
            []
    
            >>> t.put(8)
            >>> t.put(10)
            >>> t.put(9)
            >>> [i.label for i in t.inorder_traversal()]
            [8, 9, 10]
            """
        ### --- BLOCK BEGIN 15
        return class_var._inorder_traversal(class_var.root)
        ### --- BLOCK END 15
    
    
    
    def _inorder_traversal(node):
        ### --- BLOCK BEGIN 16
        if node is not None:
            yield from class_var._inorder_traversal(node.left)
            yield node
            yield from class_var._inorder_traversal(node.right)
        ### --- BLOCK END 16
    
    
    
    def preorder_traversal():
        """
            Return the preorder traversal of the tree
    
            >>> t = BinarySearchTree()
            >>> [i.label for i in t.preorder_traversal()]
            []
    
            >>> t.put(8)
            >>> t.put(10)
            >>> t.put(9)
            >>> [i.label for i in t.preorder_traversal()]
            [8, 10, 9]
            """
        ### --- BLOCK BEGIN 17
        return class_var._preorder_traversal(class_var.root)
        ### --- BLOCK END 17
    
    
    
    def _preorder_traversal(node):
        ### --- BLOCK BEGIN 18
        if node is not None:
            yield node
            yield from class_var._preorder_traversal(node.left)
            yield from class_var._preorder_traversal(node.right)
        ### --- BLOCK END 18
    
    
    
    class_var = SkelClass('BinarySearchTree')
    class_var.__init__ = __init__
    class_var.empty = empty
    class_var.is_empty = is_empty
    class_var.put = put
    class_var._put = _put
    class_var.search = search
    class_var._search = _search
    class_var.remove = remove
    class_var._reassign_nodes = _reassign_nodes
    class_var._get_lowest_node = _get_lowest_node
    class_var.exists = exists
    class_var.get_max_label = get_max_label
    class_var.get_min_label = get_min_label
    class_var.inorder_traversal = inorder_traversal
    class_var._inorder_traversal = _inorder_traversal
    class_var.preorder_traversal = preorder_traversal
    class_var._preorder_traversal = _preorder_traversal
    __init__()
    return class_var


def _get_binary_search_tree():
    r"""
            8
            / \
        3   10
        / \    \
        1   6    14
            / \   /
        4   7 13
            \
            5
    """
    ### --- BLOCK BEGIN 19
    t = BinarySearchTree()
    t.put(8)
    t.put(3)
    t.put(6)
    t.put(1)
    t.put(10)
    t.put(14)
    t.put(13)
    t.put(4)
    t.put(7)
    t.put(5)
    return t
    ### --- BLOCK END 19



def test_put():
    ### --- BLOCK BEGIN 20
    t = BinarySearchTree()
    assert t.is_empty()
    t.put(8)
    r"""
                8
        """
    assert t.root is not None
    assert t.root.parent is None
    assert t.root.label == 8
    t.put(10)
    r"""
                8
                \
                10
        """
    assert t.root.right is not None
    assert t.root.right.parent == t.root
    assert t.root.right.label == 10
    t.put(3)
    r"""
                8
                / \
            3   10
        """
    assert t.root.left is not None
    assert t.root.left.parent == t.root
    assert t.root.left.label == 3
    t.put(6)
    r"""
                8
                / \
            3   10
                \
                6
        """
    assert t.root.left.right is not None
    assert t.root.left.right.parent == t.root.left
    assert t.root.left.right.label == 6
    t.put(1)
    r"""
                8
                / \
            3   10
            / \
            1   6
        """
    assert t.root.left.left is not None
    assert t.root.left.left.parent == t.root.left
    assert t.root.left.left.label == 1
    try:
        t.put(1)
    except Exception:
        pass
    ### --- BLOCK END 20



def test_search():
    ### --- BLOCK BEGIN 21
    t = _get_binary_search_tree()
    node = t.search(6)
    assert node.label == 6
    node = t.search(13)
    assert node.label == 13
    try:
        t.search(2)
    except Exception:
        pass
    ### --- BLOCK END 21



def test_remove():
    ### --- BLOCK BEGIN 22
    t = _get_binary_search_tree()
    t.remove(13)
    r"""
                8
                / \
            3   10
            / \    \
            1   6    14
                / \
            4   7
                \
                5
        """
    assert t.root is not None
    assert t.root.right is not None
    assert t.root.right.right is not None
    assert t.root.right.right.right is None
    assert t.root.right.right.left is None
    t.remove(7)
    r"""
                8
                / \
            3   10
            / \    \
            1   6    14
                /
            4
                \
                5
        """
    assert t.root.left is not None
    assert t.root.left.right is not None
    assert t.root.left.right.left is not None
    assert t.root.left.right.right is None
    assert t.root.left.right.left.label == 4
    t.remove(6)
    r"""
                8
                / \
            3   10
            / \    \
            1   4    14
                \
                5
        """
    assert t.root.left.left is not None
    assert t.root.left.right.right is not None
    assert t.root.left.left.label == 1
    assert t.root.left.right.label == 4
    assert t.root.left.right.right.label == 5
    assert t.root.left.right.left is None
    assert t.root.left.left.parent == t.root.left
    assert t.root.left.right.parent == t.root.left
    t.remove(3)
    r"""
                8
                / \
            4   10
            / \    \
            1   5    14
        """
    assert t.root is not None
    assert t.root.left.label == 4
    assert t.root.left.right.label == 5
    assert t.root.left.left.label == 1
    assert t.root.left.parent == t.root
    assert t.root.left.left.parent == t.root.left
    assert t.root.left.right.parent == t.root.left
    t.remove(4)
    r"""
                8
                / \
            5   10
            /      \
            1        14
        """
    assert t.root.left is not None
    assert t.root.left.left is not None
    assert t.root.left.label == 5
    assert t.root.left.right is None
    assert t.root.left.left.label == 1
    assert t.root.left.parent == t.root
    assert t.root.left.left.parent == t.root.left
    ### --- BLOCK END 22



def test_remove_2():
    ### --- BLOCK BEGIN 23
    t = _get_binary_search_tree()
    t.remove(3)
    r"""
                8
                / \
            4   10
            / \    \
            1   6    14
                / \   /
            5   7 13
        """
    assert t.root is not None
    assert t.root.left is not None
    assert t.root.left.left is not None
    assert t.root.left.right is not None
    assert t.root.left.right.left is not None
    assert t.root.left.right.right is not None
    assert t.root.left.label == 4
    assert t.root.left.right.label == 6
    assert t.root.left.left.label == 1
    assert t.root.left.right.right.label == 7
    assert t.root.left.right.left.label == 5
    assert t.root.left.parent == t.root
    assert t.root.left.right.parent == t.root.left
    assert t.root.left.left.parent == t.root.left
    assert t.root.left.right.left.parent == t.root.left.right
    ### --- BLOCK END 23



def test_empty():
    ### --- BLOCK BEGIN 24
    t = _get_binary_search_tree()
    t.empty()
    assert t.root is None
    ### --- BLOCK END 24



def test_is_empty():
    ### --- BLOCK BEGIN 25
    t = _get_binary_search_tree()
    assert not t.is_empty()
    t.empty()
    assert t.is_empty()
    ### --- BLOCK END 25



def test_exists():
    ### --- BLOCK BEGIN 26
    t = _get_binary_search_tree()
    assert t.exists(6)
    assert not t.exists(-1)
    ### --- BLOCK END 26



def test_get_max_label():
    ### --- BLOCK BEGIN 27
    t = _get_binary_search_tree()
    assert t.get_max_label() == 14
    t.empty()
    try:
        t.get_max_label()
    except Exception:
        pass
    ### --- BLOCK END 27



def test_get_min_label():
    ### --- BLOCK BEGIN 28
    t = _get_binary_search_tree()
    assert t.get_min_label() == 1
    t.empty()
    try:
        t.get_min_label()
    except Exception:
        pass
    ### --- BLOCK END 28



def test_inorder_traversal():
    ### --- BLOCK BEGIN 29
    t = _get_binary_search_tree()
    inorder_traversal_nodes = [i.label for i in t.inorder_traversal()]
    assert inorder_traversal_nodes == [1, 3, 4, 5, 6, 7, 8, 10, 13, 14]
    ### --- BLOCK END 29



def test_preorder_traversal():
    ### --- BLOCK BEGIN 30
    t = _get_binary_search_tree()
    preorder_traversal_nodes = [i.label for i in t.preorder_traversal()]
    assert preorder_traversal_nodes == [8, 3, 1, 6, 4, 5, 7, 10, 14, 13]
    ### --- BLOCK END 30



def binary_search_tree_example():
    r"""
    Example
                  8
                 / \
                3   10
               / \    \
              1   6    14
                 / \   /
                4   7 13
                \
                5

    Example After Deletion
                  4
                 / \
                1   7
                     \
                      5

    """
    ### --- BLOCK BEGIN 31
    t = BinarySearchTree()
    t.put(8)
    t.put(3)
    t.put(6)
    t.put(1)
    t.put(10)
    t.put(14)
    t.put(13)
    t.put(4)
    t.put(7)
    t.put(5)
    print(
    """
                8
               / \\
              3   10
             / \\    \\
            1   6    14
               / \\   /
              4   7 13
               \\
                5
            """
    )
    print("Label 6 exists:", t.exists(6))
    print("Label 13 exists:", t.exists(13))
    print("Label -1 exists:", t.exists(-1))
    print("Label 12 exists:", t.exists(12))
    # Prints all the elements of the list in inorder traversal
    inorder_traversal_nodes = [i.label for i in t.inorder_traversal()]
    print("Inorder traversal:", inorder_traversal_nodes)
    # Prints all the elements of the list in preorder traversal
    preorder_traversal_nodes = [i.label for i in t.preorder_traversal()]
    print("Preorder traversal:", preorder_traversal_nodes)
    print("Max. label:", t.get_max_label())
    print("Min. label:", t.get_min_label())
    # Delete elements
    print("\nDeleting elements 13, 10, 8, 3, 6, 14")
    print(
    """
              4
             / \\
            1   7
                 \\
                  5
            """
    )
    t.remove(13)
    t.remove(10)
    t.remove(8)
    t.remove(3)
    t.remove(6)
    t.remove(14)
    # Prints all the elements of the list in inorder traversal after delete
    inorder_traversal_nodes = [i.label for i in t.inorder_traversal()]
    print("Inorder traversal after delete:", inorder_traversal_nodes)
    # Prints all the elements of the list in preorder traversal after delete
    preorder_traversal_nodes = [i.label for i in t.preorder_traversal()]
    print("Preorder traversal after delete:", preorder_traversal_nodes)
    print("Max. label:", t.get_max_label())
    print("Min. label:", t.get_min_label())
    ### --- BLOCK END 31



def test():
    ### --- BLOCK BEGIN 32
    binary_search_tree_example()
    test_put()
    test_search()
    test_remove()
    test_remove_2()
    test_is_empty()
    test_empty()
    test_exists()
    test_get_max_label()
    test_get_min_label()
    test_inorder_traversal()
    test_preorder_traversal()
    ### --- BLOCK END 32



### Global Begin

### --- BLOCK BEGIN 0
test()

### --- BLOCK END 0
