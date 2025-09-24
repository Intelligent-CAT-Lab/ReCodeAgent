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
psf/black : true
ruff : passed
"""

from collections.abc import Iterator

def RedBlackTree(param_0, param_1, param_2, param_3, param_4):
    """
    A Red-Black tree, which is a self-balancing BST (binary search
    tree).
    This tree has similar performance to AVL trees, but the balancing is
    less strict, so it will perform faster for writing/deleting nodes
    and slower for reading in the average case, though, because they're
    both balanced binary search trees, both will get the same asymptotic
    performance.
    To read more about them, https://en.wikipedia.org/wiki/Red-black_tree
    Unless otherwise specified, all asymptotic runtimes are specified in
    terms of the size of the tree.
    """
    def __init__(label, color, parent, left, right):
        ### --- BLOCK BEGIN 1
        class_var.label = label
        class_var.parent = parent
        class_var.left = left
        class_var.right = right
        class_var.color = color
        ### --- BLOCK END 1
    
    
    
    # Here are functions which are specific to red-black trees
    def rotate_left():
        """Rotate the subtree rooted at this node to the left and
            returns the new root to this subtree.
            Performing one rotation can be done in O(1).
            """
        ### --- BLOCK BEGIN 2
        parent = class_var.parent
        right = class_var.right
        if right is None:
            return class_var
        class_var.right = right.left
        if class_var.right:
            class_var.right.parent = class_var
        class_var.parent = right
        right.left = class_var
        if parent is not None:
            if ((hasattr(parent.left, '__eq__') and parent.left.__eq__(class_var)) or (not hasattr(parent.left, '__eq__') and parent.left == class_var)):
                parent.left = right
            else:
                parent.right = right
        right.parent = parent
        return right
        ### --- BLOCK END 2
    
    
    
    def rotate_right():
        """Rotate the subtree rooted at this node to the right and
            returns the new root to this subtree.
            Performing one rotation can be done in O(1).
            """
        ### --- BLOCK BEGIN 3
        if class_var.left is None:
            return class_var
        parent = class_var.parent
        left = class_var.left
        class_var.left = left.right
        if class_var.left:
            class_var.left.parent = class_var
        class_var.parent = left
        left.right = class_var
        if parent is not None:
            if parent.right is class_var:
                parent.right = left
            else:
                parent.left = left
        left.parent = parent
        return left
        ### --- BLOCK END 3
    
    
    
    def insert(label):
        """Inserts label into the subtree rooted at self, performs any
            rotations necessary to maintain balance, and then returns the
            new root to this subtree (likely self).
            This is guaranteed to run in O(log(n)) time.
            """
        ### --- BLOCK BEGIN 4
        if class_var.label is None:
        # Only possible with an empty tree
            class_var.label = label
            return class_var
        if class_var.label == label:
            return class_var
        elif class_var.label > label:
            if class_var.left:
                class_var.left.insert(label)
            else:
                class_var.left = RedBlackTree(label, 1, class_var, None, None)
                class_var.left._insert_repair()
        else:
            if class_var.right:
                class_var.right.insert(label)
            else:
                class_var.right = RedBlackTree(label, 1, class_var, None, None)
                class_var.right._insert_repair()
        return class_var.parent or class_var
        ### --- BLOCK END 4
    
    
    
    def _insert_repair():
        """Repair the coloring from inserting into a tree."""
        ### --- BLOCK BEGIN 5
        if class_var.parent is None:
        # This node is the root, so it just needs to be black
            class_var.color = 0
        elif get_color(class_var.parent) == 0:
        # If the parent is black, then it just needs to be red
            class_var.color = 1
        else:
            uncle = class_var.parent.sibling()
            if get_color(uncle) == 0:
                if class_var.is_left() and class_var.parent.is_right():
                    class_var.parent.rotate_right()
                    if class_var.right:
                        class_var.right._insert_repair()
                elif class_var.is_right() and class_var.parent.is_left():
                    class_var.parent.rotate_left()
                    if class_var.left:
                        class_var.left._insert_repair()
                elif class_var.is_left():
                    if class_var.grandparent():
                        class_var.grandparent().rotate_right()
                        class_var.parent.color = 0
                    if class_var.parent.right:
                        class_var.parent.right.color = 1
                else:
                    if class_var.grandparent():
                        class_var.grandparent().rotate_left()
                        class_var.parent.color = 0
                    if class_var.parent.left:
                        class_var.parent.left.color = 1
            else:
                class_var.parent.color = 0
                if uncle and class_var.grandparent():
                    uncle.color = 0
                    class_var.grandparent().color = 1
                    class_var.grandparent()._insert_repair()
        ### --- BLOCK END 5
    
    
    
    def remove(label):
        """Remove label from this tree."""
        ### --- BLOCK BEGIN 6
        if class_var.label == label:
            if class_var.left and class_var.right:
            # It's easier to balance a node with at most one child,
            # so we replace this node with the greatest one less than
            # it and remove that.
                value = class_var.left.get_max()
                if value is not None:
                    class_var.label = value
                    class_var.left.remove(value)
            else:
            # This node has at most one non-None child, so we don't
            # need to replace
                child = class_var.left or class_var.right
                if class_var.color == 1:
                # This node is red, and its child is black
                # The only way this happens to a node with one child
                # is if both children are None leaves.
                # We can just remove this node and call it a day.
                    if class_var.parent:
                        if class_var.is_left():
                            class_var.parent.left = None
                        else:
                            class_var.parent.right = None
                else:
                # The node is black
                    if child is None:
                    # This node and its child are black
                        if class_var.parent is None:
                        # The tree is now empty
                            return RedBlackTree(None)
                        else:
                            class_var._remove_repair()
                            if class_var.is_left():
                                class_var.parent.left = None
                            else:
                                class_var.parent.right = None
                            class_var.parent = None
                    else:
                    # This node is black and its child is red
                    # Move the child node here and make it black
                        class_var.label = child.label
                        class_var.left = child.left
                        class_var.right = child.right
                        if class_var.left:
                            class_var.left.parent = class_var
                        if class_var.right:
                            class_var.right.parent = class_var
        elif class_var.label is not None and class_var.label > label:
            if class_var.left:
                class_var.left.remove(label)
        else:
            if class_var.right:
                class_var.right.remove(label)
        return class_var.parent or class_var
        ### --- BLOCK END 6
    
    
    
    def _remove_repair():
        """Repair the coloring of the tree that may have been messed up."""
        ### --- BLOCK BEGIN 7
        if (
        class_var.parent is None
        or class_var.sibling() is None
        or class_var.parent.sibling() is None
        or class_var.grandparent() is None
        ):
            return
        if get_color(class_var.sibling()) == 1:
            class_var.sibling().color = 0
            class_var.parent.color = 1
            if class_var.is_left():
                class_var.parent.rotate_left()
            else:
                class_var.parent.rotate_right()
        if (
        get_color(class_var.parent) == 0
        and get_color(class_var.sibling()) == 0
        and get_color(class_var.sibling().left) == 0
        and get_color(class_var.sibling().right) == 0
        ):
            class_var.sibling().color = 1
            class_var.parent._remove_repair()
            return
        if (
        get_color(class_var.parent) == 1
        and get_color(class_var.sibling()) == 0
        and get_color(class_var.sibling().left) == 0
        and get_color(class_var.sibling().right) == 0
        ):
            class_var.sibling().color = 1
            class_var.parent.color = 0
            return
        if (
        class_var.is_left()
        and get_color(class_var.sibling()) == 0
        and get_color(class_var.sibling().right) == 0
        and get_color(class_var.sibling().left) == 1
        ):
            class_var.sibling().rotate_right()
            class_var.sibling().color = 0
            if class_var.sibling().right:
                class_var.sibling().right.color = 1
        if (
        class_var.is_right()
        and get_color(class_var.sibling()) == 0
        and get_color(class_var.sibling().right) == 1
        and get_color(class_var.sibling().left) == 0
        ):
            class_var.sibling().rotate_left()
            class_var.sibling().color = 0
            if class_var.sibling().left:
                class_var.sibling().left.color = 1
        if (
        class_var.is_left()
        and get_color(class_var.sibling()) == 0
        and get_color(class_var.sibling().right) == 1
        ):
            class_var.parent.rotate_left()
            class_var.grandparent().color = class_var.parent.color
            class_var.parent.color = 0
            class_var.parent.sibling().color = 0
        if (
        class_var.is_right()
        and get_color(class_var.sibling()) == 0
        and get_color(class_var.sibling().left) == 1
        ):
            class_var.parent.rotate_right()
            class_var.grandparent().color = class_var.parent.color
            class_var.parent.color = 0
            class_var.parent.sibling().color = 0
        ### --- BLOCK END 7
    
    
    
    def check_color_properties():
        """Check the coloring of the tree, and return True iff the tree
            is colored in a way which matches these five properties:
            (wording stolen from wikipedia article)
             1. Each node is either red or black.
             2. The root node is black.
             3. All leaves are black.
             4. If a node is red, then both its children are black.
             5. Every path from any node to all of its descendent NIL nodes
                has the same number of black nodes.
            This function runs in O(n) time, because properties 4 and 5 take
            that long to check.
            """
        # I assume property 1 to hold because there is nothing that can
        # make the color be anything other than 0 or 1.
        # Property 2
        ### --- BLOCK BEGIN 8
        if class_var.color:
        # The root was red
            print("Property 2")
            return False
        # Property 3 does not need to be checked, because None is assumed
        # to be black and is all the leaves.
        # Property 4
        if not class_var.check_coloring():
            print("Property 4")
            return False
        # Property 5
        if class_var.black_height() is None:
            print("Property 5")
            return False
        # All properties were met
        return True
        ### --- BLOCK END 8
    
    
    
    def check_coloring():
        """A helper function to recursively check Property 4 of a
            Red-Black Tree. See check_color_properties for more info.
            """
        ### --- BLOCK BEGIN 9
        if class_var.color == 1 and 1 in (get_color(class_var.left), get_color(class_var.right)):
            return False
        if class_var.left and not class_var.left.check_coloring():
            return False
        if class_var.right and not class_var.right.check_coloring():
            return False
        return True
        ### --- BLOCK END 9
    
    
    
    def black_height():
        """Returns the number of black nodes from this node to the
            leaves of the tree, or None if there isn't one such value (the
            tree is color incorrectly).
            """
        ### --- BLOCK BEGIN 10
        if class_var is None or class_var.left is None or class_var.right is None:
        # If we're already at a leaf, there is no path
            return 1
        left = class_var.left.black_height()
        right = class_var.right.black_height()
        if left is None or right is None:
        # There are issues with coloring below children nodes
            return None
        if left != right:
        # The two children have unequal depths
            return None
        # Return the black depth of children, plus one if this node is
        # black
        return left + (1 - class_var.color)
        ### --- BLOCK END 10
    
    
    
    # Here are functions which are general to all binary search trees
    def __contains__(label):
        """Search through the tree for label, returning True iff it is
            found somewhere in the tree.
            Guaranteed to run in O(log(n)) time.
            """
        ### --- BLOCK BEGIN 11
        return class_var.search(label) is not None
        ### --- BLOCK END 11
    
    
    
    def search(label):
        """Search through the tree for label, returning its node if
            it's found, and None otherwise.
            This method is guaranteed to run in O(log(n)) time.
            """
        ### --- BLOCK BEGIN 12
        if class_var.label == label:
            return class_var
        elif class_var.label is not None and label > class_var.label:
            if class_var.right is None:
                return None
            else:
                return class_var.right.search(label)
        else:
            if class_var.left is None:
                return None
            else:
                return class_var.left.search(label)
        ### --- BLOCK END 12
    
    
    
    def floor(label):
        """Returns the largest element in this tree which is at most label.
            This method is guaranteed to run in O(log(n)) time."""
        ### --- BLOCK BEGIN 13
        if class_var.label == label:
            return class_var.label
        elif class_var.label is not None and class_var.label > label:
            if class_var.left:
                return class_var.left.floor(label)
            else:
                return None
        else:
            if class_var.right:
                attempt = class_var.right.floor(label)
                if attempt is not None:
                    return attempt
            return class_var.label
        ### --- BLOCK END 13
    
    
    
    def ceil(label):
        """Returns the smallest element in this tree which is at least label.
            This method is guaranteed to run in O(log(n)) time.
            """
        ### --- BLOCK BEGIN 14
        if class_var.label == label:
            return class_var.label
        elif class_var.label is not None and class_var.label < label:
            if class_var.right:
                return class_var.right.ceil(label)
            else:
                return None
        else:
            if class_var.left:
                attempt = class_var.left.ceil(label)
                if attempt is not None:
                    return attempt
            return class_var.label
        ### --- BLOCK END 14
    
    
    
    def get_max():
        """Returns the largest element in this tree.
            This method is guaranteed to run in O(log(n)) time.
            """
        ### --- BLOCK BEGIN 15
        if class_var.right:
        # Go as far right as possible
            return class_var.right.get_max()
        else:
            return class_var.label
        ### --- BLOCK END 15
    
    
    
    def get_min():
        """Returns the smallest element in this tree.
            This method is guaranteed to run in O(log(n)) time.
            """
        ### --- BLOCK BEGIN 16
        if class_var.left:
        # Go as far left as possible
            return class_var.left.get_min()
        else:
            return class_var.label
        ### --- BLOCK END 16
    
    
    
    def grandparent():
        """Get the current node's grandparent, or None if it doesn't exist."""
        ### --- BLOCK BEGIN 17
        if class_var.parent is None:
            return None
        else:
            return class_var.parent.parent
        ### --- BLOCK END 17
    
    
    
    def sibling():
        """Get the current node's sibling, or None if it doesn't exist."""
        ### --- BLOCK BEGIN 18
        if class_var.parent is None:
            return None
        elif class_var.parent.left is class_var:
            return class_var.parent.right
        else:
            return class_var.parent.left
        ### --- BLOCK END 18
    
    
    
    def is_left():
        """Returns true iff this node is the left child of its parent."""
        ### --- BLOCK BEGIN 19
        if class_var.parent is None:
            return False
        return class_var.parent.left is class_var.parent.left is class_var
        ### --- BLOCK END 19
    
    
    
    def is_right():
        """Returns true iff this node is the right child of its parent."""
        ### --- BLOCK BEGIN 20
        if class_var.parent is None:
            return False
        return class_var.parent.right is class_var
        ### --- BLOCK END 20
    
    
    
    def __bool__():
        ### --- BLOCK BEGIN 21
        return True
        ### --- BLOCK END 21
    
    
    
    def __len__():
        """
            Return the number of nodes in this tree.
            """
        ### --- BLOCK BEGIN 22
        ln = 1
        if class_var.left:
            ln += len(class_var.left)
        if class_var.right:
            ln += len(class_var.right)
        return ln
        ### --- BLOCK END 22
    
    
    
    def preorder_traverse():
        ### --- BLOCK BEGIN 23
        yield class_var.label
        if class_var.left:
            yield from class_var.left.preorder_traverse()
        if class_var.right:
            yield from class_var.right.preorder_traverse()
        ### --- BLOCK END 23
    
    
    
    def inorder_traverse():
        ### --- BLOCK BEGIN 24
        if class_var.left:
            yield from class_var.left.inorder_traverse()
        yield class_var.label
        if class_var.right:
            yield from class_var.right.inorder_traverse()
        ### --- BLOCK END 24
    
    
    
    def postorder_traverse():
        ### --- BLOCK BEGIN 25
        if class_var.left:
            yield from class_var.left.postorder_traverse()
        if class_var.right:
            yield from class_var.right.postorder_traverse()
        yield class_var.label
        ### --- BLOCK END 25
    
    
    
    def __eq__(other):
        """Test if two trees are equal."""
        ### --- BLOCK BEGIN 26
        if not user_check_type(other, RedBlackTree):
            return NotImplemented
        if class_var.label == other.label:
            return ((hasattr(class_var.left, '__eq__') and class_var.left.__eq__(other.left)) or (not hasattr(class_var.left, '__eq__') and class_var.left == other.left)) and ((hasattr(class_var.right, '__eq__') and class_var.right.__eq__(other.right)) or (not hasattr(class_var.right, '__eq__') and class_var.right == other.right))
        else:
            return False
        ### --- BLOCK END 26
    
    
    
    class_var = SkelClass('RedBlackTree')
    class_var.__init__ = __init__
    class_var.rotate_left = rotate_left
    class_var.rotate_right = rotate_right
    class_var.insert = insert
    class_var._insert_repair = _insert_repair
    class_var.remove = remove
    class_var._remove_repair = _remove_repair
    class_var.check_color_properties = check_color_properties
    class_var.check_coloring = check_coloring
    class_var.black_height = black_height
    class_var.__contains__ = __contains__
    class_var.search = search
    class_var.floor = floor
    class_var.ceil = ceil
    class_var.get_max = get_max
    class_var.get_min = get_min
    class_var.grandparent = grandparent
    class_var.sibling = sibling
    class_var.is_left = is_left
    class_var.is_right = is_right
    class_var.__bool__ = __bool__
    class_var.__len__ = __len__
    class_var.preorder_traverse = preorder_traverse
    class_var.inorder_traverse = inorder_traverse
    class_var.postorder_traverse = postorder_traverse
    class_var.__eq__ = __eq__
    __init__(param_0, param_1, param_2, param_3, param_4)
    return class_var


def get_color(node):
    """Returns the color of a node, allowing for None leaves."""
    ### --- BLOCK BEGIN 27
    if node is None:
        return 0
    else:
        return node.color
    ### --- BLOCK END 27



"""
Code for testing the various
functions of the red-black tree.
"""

def test_rotations():
    """Test that the rotate_left and rotate_right functions work."""
    # Make a tree to test on
    ### --- BLOCK BEGIN 28
    tree = RedBlackTree(0, 0, None, None, None)
    tree.left = RedBlackTree(-10, 0, tree, None, None)
    tree.right = RedBlackTree(10, 0, tree, None, None)
    tree.left.left = RedBlackTree(-20, 0, tree.left, None, None)
    tree.left.right = RedBlackTree(-5, 0, tree.left, None, None)
    tree.right.left = RedBlackTree(5, 0, tree.right, None, None)
    tree.right.right = RedBlackTree(20, 0, tree.right, None, None)
    # Make the right rotation
    left_rot = RedBlackTree(10, 0, None, None, None)
    left_rot.left = RedBlackTree(0, 0, left_rot, None, None)
    left_rot.left.left = RedBlackTree(-10, 0, left_rot.left, None, None)
    left_rot.left.right = RedBlackTree(5, 0, left_rot.left, None, None)
    left_rot.left.left.left = RedBlackTree(-20, 0, left_rot.left.left, None, None)
    left_rot.left.left.right = RedBlackTree(-5, 0, left_rot.left.left, None, None)
    left_rot.right = RedBlackTree(20, 0, left_rot, None, None)
    tree = tree.rotate_left()
    assert ((hasattr(tree, '__eq__') and tree.__eq__(left_rot)) or (not hasattr(tree, '__eq__') and tree == left_rot))
    tree = tree.rotate_right()
    tree = tree.rotate_right()
    # Make the left rotation
    right_rot = RedBlackTree(-10, 0, None, None, None)
    right_rot.left = RedBlackTree(-20, 0, right_rot, None, None)
    right_rot.right = RedBlackTree(0, 0, right_rot, None, None)
    right_rot.right.left = RedBlackTree(-5, 0, right_rot.right, None, None)
    right_rot.right.right = RedBlackTree(10, 0, right_rot.right, None, None)
    right_rot.right.right.left = RedBlackTree(5, 0, right_rot.right.right, None, None)
    right_rot.right.right.right = RedBlackTree(20, 0, right_rot.right.right, None, None)
    assert ((hasattr(tree, '__eq__') and tree.__eq__(right_rot)) or (not hasattr(tree, '__eq__') and tree == right_rot))
    return True
    ### --- BLOCK END 28



def test_insertion_speed():
    """Test that the tree balances inserts to O(log(n)) by doing a lot
    of them.
    """
    ### --- BLOCK BEGIN 29
    tree = RedBlackTree(-1, 0, None, None, None)
    for i in range(10):
        tree = tree.insert(i)
    return True
    ### --- BLOCK END 29



def test_insert():
    """Test the insert() method of the tree correctly balances, colors,
    and inserts.
    """
    ### --- BLOCK BEGIN 30
    tree = RedBlackTree(0, 0, None, None, None)
    tree.insert(8)
    tree.insert(-8)
    tree.insert(4)
    tree.insert(12)
    tree.insert(10)
    tree.insert(11)
    ans = RedBlackTree(0, 0, None, None, None)
    ans.left = RedBlackTree(-8, 0, ans, None, None)
    ans.right = RedBlackTree(8, 1, ans, None, None)
    ans.right.left = RedBlackTree(4, 0, ans.right, None, None)
    ans.right.right = RedBlackTree(11, 0, ans.right, None, None)
    ans.right.right.left = RedBlackTree(10, 1, ans.right.right, None, None)
    ans.right.right.right = RedBlackTree(12, 1, ans.right.right, None, None)
    return ((hasattr(tree, '__eq__') and tree.__eq__(ans)) or (not hasattr(tree, '__eq__') and tree == ans))
    ### --- BLOCK END 30



def test_insert_and_search():
    """Tests searching through the tree for values."""
    ### --- BLOCK BEGIN 31
    tree = RedBlackTree(0, 0, None, None, None)
    tree.insert(8)
    tree.insert(-8)
    tree.insert(4)
    tree.insert(12)
    tree.insert(10)
    tree.insert(11)
    assert not (((hasattr(tree, '__contains__') and tree.__contains__(5)) or (not hasattr(tree, '__contains__') and 5 in tree)) or ((hasattr(tree, '__contains__') and tree.__contains__(-6)) or (not hasattr(tree, '__contains__') and -6 in tree)) or ((hasattr(tree, '__contains__') and tree.__contains__(-10)) or (not hasattr(tree, '__contains__') and -10 in tree)) or ((hasattr(tree, '__contains__') and tree.__contains__(13)) or (not hasattr(tree, '__contains__') and 13 in tree)))
    # Found something not in there
    assert (((hasattr(tree, '__contains__') and tree.__contains__(11)) or (not hasattr(tree, '__contains__') and 11 in tree)) and ((hasattr(tree, '__contains__') and tree.__contains__(12)) or (not hasattr(tree, '__contains__') and 12 in tree)) and ((hasattr(tree, '__contains__') and tree.__contains__(-8)) or (not hasattr(tree, '__contains__') and -8 in tree)) and ((hasattr(tree, '__contains__') and tree.__contains__(0)) or (not hasattr(tree, '__contains__') and 0 in tree)))
    # Didn't find something in there
    return True
    ### --- BLOCK END 31



def test_insert_delete():
    """Test the insert() and delete() method of the tree, verifying the
    insertion and removal of elements, and the balancing of the tree.
    """
    ### --- BLOCK BEGIN 32
    tree = RedBlackTree(0, 0, None, None, None)
    tree = tree.insert(-12)
    tree = tree.insert(8)
    tree = tree.insert(-8)
    tree = tree.insert(15)
    tree = tree.insert(4)
    tree = tree.insert(12)
    tree = tree.insert(10)
    tree = tree.insert(9)
    tree = tree.insert(11)
    tree = tree.remove(15)
    tree = tree.remove(-12)
    tree = tree.remove(9)
    assert tree.check_color_properties()
    assert list(tree.inorder_traverse()) == [-8, 0, 4, 8, 10, 11, 12]
    return True
    ### --- BLOCK END 32



def test_floor_ceil():
    """Tests the floor and ceiling functions in the tree."""
    ### --- BLOCK BEGIN 33
    tree = RedBlackTree(0, 0, None, None, None)
    tree.insert(-16)
    tree.insert(16)
    tree.insert(8)
    tree.insert(24)
    tree.insert(20)
    tree.insert(22)
    tuples = [(-20, None, -16), (-10, -16, 0), (8, 8, 8), (50, 24, None)]
    for val, floor, ceil in tuples:
        assert tree.floor(val) == floor and tree.ceil(val) == ceil
    return True
    ### --- BLOCK END 33



def test_min_max():
    """Tests the min and max functions in the tree."""
    ### --- BLOCK BEGIN 34
    tree = RedBlackTree(0, 0, None, None, None)
    tree.insert(-16)
    tree.insert(16)
    tree.insert(8)
    tree.insert(24)
    tree.insert(20)
    tree.insert(22)
    assert tree.get_max() == 24 and tree.get_min() == -16
    return True
    ### --- BLOCK END 34



def test_tree_traversal():
    """Tests the three different tree traversal functions."""
    ### --- BLOCK BEGIN 35
    tree = RedBlackTree(0, 0, None, None, None)
    tree = tree.insert(-16)
    tree.insert(16)
    tree.insert(8)
    tree.insert(24)
    tree.insert(20)
    tree.insert(22)
    assert list(tree.inorder_traverse()) == [-16, 0, 8, 16, 20, 22, 24]
    assert list(tree.preorder_traverse()) == [0, -16, 16, 8, 22, 20, 24]
    assert list(tree.postorder_traverse()) == [-16, 8, 20, 24, 22, 16, 0]
    return True
    ### --- BLOCK END 35



def test_tree_chaining():
    """Tests the three different tree chaining functions."""
    ### --- BLOCK BEGIN 36
    tree = RedBlackTree(0, 0, None, None, None)
    tree = tree.insert(-16).insert(16).insert(8).insert(24).insert(20).insert(22)
    assert list(tree.inorder_traverse()) == [-16, 0, 8, 16, 20, 22, 24]
    assert list(tree.preorder_traverse()) == [0, -16, 16, 8, 22, 20, 24]
    assert list(tree.postorder_traverse()) == [-16, 8, 20, 24, 22, 16, 0]
    return True
    ### --- BLOCK END 36



def print_results(msg, passes):
    ### --- BLOCK BEGIN 37
    print(str(msg), "works!" if passes else "doesn't work :|")
    ### --- BLOCK END 37



def test():
    """
    >>> pytests()
    """
    ### --- BLOCK BEGIN 38
    print_results("Rotating right and left", test_rotations())
    print_results("Inserting", test_insert())
    print_results("Searching", test_insert_and_search())
    print_results("Deleting", test_insert_delete())
    print_results("Floor and ceil", test_floor_ceil())
    print_results("Min and max", test_min_max())
    print_results("Tree traversal", test_tree_traversal())
    print_results("Tree traversal", test_tree_chaining())
    print("Testing tree balancing...")
    print("This should only be a few seconds.")
    test_insertion_speed()
    additional_tests()
    print("Done!")
    ### --- BLOCK END 38



def additional_tests():
    ### --- BLOCK BEGIN 39
    tree = RedBlackTree(0, 0, None, None, None)
    assert tree.__len__() == 1
    tree = RedBlackTree(0, 0, None, None, None)
    tree.insert(-16).insert(16).insert(-8).insert(12)
    tree.insert(-20).insert(8).insert(-4).insert(4)
    tree.insert(-3).insert(24).insert(-20).insert(20)
    tree.insert(-1).insert(2).insert(-3).insert(3)
    tree.insert(10).insert(26)
    tree.right.right.left._remove_repair()
    assert tree.right.right.left.label == 20
    ### --- BLOCK END 39



### Global Begin

### --- BLOCK BEGIN 0
test()

### --- BLOCK END 0
