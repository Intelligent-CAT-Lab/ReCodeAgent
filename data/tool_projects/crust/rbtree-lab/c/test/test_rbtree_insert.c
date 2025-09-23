#include "rbtree.h"
#include <stdio.h>
#include <stdlib.h>

void _rbtree_print(rbtree * tree, node_t * curr, int tab_count) {
    if(curr == tree->nil){
        return;
    }
    if(tab_count == 1){
        printf("root: %d(%s)\n", curr->key, curr->color == RBTREE_BLACK ? "black" : "red");
    }
    if(curr->left != tree->nil){
        for(int i = 0; i < tab_count * 4; i++){
            printf(" ");
        }
        printf("left: %d(%s)\n", curr->left->key, curr->left->color == RBTREE_BLACK ? "black" : "red");
        _rbtree_print(tree, curr->left, tab_count+1);
    }
    if(curr->right != tree->nil){
        for(int i = 0; i < tab_count * 4; i++){
            printf(" ");
        }
        printf("right: %d(%s)\n", curr->right->key, curr->right->color == RBTREE_BLACK ? "black" : "red");
        _rbtree_print(tree, curr->right, tab_count+1);
    }
}


void rbtree_print(rbtree * tree) {
    if(tree->root == tree->nil){
        printf("empty tree.\n");
        return;
    }
    _rbtree_print(tree, tree->root, 1);
    printf("\n");
}


void rbtree_insert_test(){
    printf("rbtree insertion test!\n");
    rbtree* rbt = new_rbtree();
    printf("test1\n");
    rbtree_insert(rbt, 11);
    rbtree_insert(rbt, 2);
    rbtree_insert(rbt, 14);
    rbtree_insert(rbt, 1);
    rbtree_insert(rbt, 7);
    rbtree_insert(rbt, 15);
    rbtree_insert(rbt, 5);
    rbtree_insert(rbt, 8);
    rbtree_insert(rbt, 4);
    rbtree_print(rbt);
    delete_rbtree(rbt);

    rbtree* rbt2 = new_rbtree();
    printf("test2\n");
    rbtree_insert(rbt2, 1);
    rbtree_insert(rbt2, 2);
    rbtree_insert(rbt2, 3);
    rbtree_insert(rbt2, 4);
    rbtree_insert(rbt2, 5);
    rbtree_insert(rbt2, 6);  
    rbtree_insert(rbt2, 7);
    rbtree_insert(rbt2, 8);
    rbtree_print(rbt2);
    delete_rbtree(rbt2);

    rbtree* rbt3 = new_rbtree();
    printf("test3\n");
    rbtree_insert(rbt3, 8);
    rbtree_insert(rbt3, 7);
    rbtree_insert(rbt3, 6);
    rbtree_insert(rbt3, 5);
    rbtree_insert(rbt3, 4);
    rbtree_insert(rbt3, 3);  
    rbtree_insert(rbt3, 2);
    rbtree_insert(rbt3, 1);
    rbtree_print(rbt3);
    delete_rbtree(rbt3);
}


int main(int argc, char *argv[]) {
    rbtree_insert_test();
    /*
    rbtree insertion test!
    test1
    root: 7(black)
        left: 2(red)
            left: 1(black)
            right: 5(black)
                left: 4(red)
        right: 11(red)
            left: 8(black)
            right: 14(black)
                right: 15(red)

    test2
    root: 4(black)
        left: 2(red)
            left: 1(black)
            right: 3(black)
        right: 6(red)
            left: 5(black)
            right: 7(black)
                right: 8(red)

    test3
    root: 5(black)
        left: 3(red)
            left: 2(black)
                left: 1(red)
            right: 4(black)
        right: 7(red)
            left: 6(black)
            right: 8(black)
    */
}