#ifndef RECT_PACK_H
#define RECT_PACK_H

#include <stdbool.h>
#include <stddef.h>

typedef struct rect_out_info rect_out_info;
struct rect_out_info {
    int x, y;
    bool packed;
    int page;
};

typedef struct rect_r rect_r;
struct rect_r {
    int id;
    int w, h;
    rect_out_info info;
};

// rect packing based on bin tree method
bool rect_pack(int max_w, int max_h, bool paging, rect_r* rects, int rects_size);

#endif

#ifdef RECT_PACK_H_IMPL

#include <math.h>
#include <stdlib.h>

typedef struct pack_ctx pack_ctx;
struct pack_ctx {
    rect_r* r;
    int n;
    int max_w, max_h;
    int page;
    int next;
    int last;
};

typedef struct pack_res pack_res;
struct pack_res {
    bool all_fit;
    bool none_fit;
};

typedef struct bnode bnode;
struct bnode {
    int x, y, w, h;
    bool used;
    bnode* right;
    bnode* down;
};

bnode* create_bnode_empty() {
    bnode* n = (bnode*)malloc(sizeof(bnode));
    n->right = n->down = NULL;
    return n;
}

bnode* create_bnode(int x, int y, int w, int h) {
    bnode* n = create_bnode_empty();

    if(!n) {
        return n;
    }

    n->x = x;
    n->y = y;
    n->w = w;
    n->h = h;

    n->used = false;
    n->right = NULL;
    n->down = NULL;

    return n;
}

void destroy_bnode(bnode* node) {
    if(!node) {
        return;
    }

    destroy_bnode(node->right);
    destroy_bnode(node->down);

    free(node);
}

rect_out_info empty_rect_r() {
    rect_out_info info = {.x = 0, .y = 0, .packed = false, .page = 0};
    return info;
}

int compare_rect_r_max_side(const void* r1, const void* r2) {
#define __MAX(a, b) ((a) > (b) ? (a) : (b))
#define __MIN(a, b) ((a) < (b) ? (a) : (b))

    int diff =
        __MAX(((rect_r*)r2)->w, ((rect_r*)r2)->h) - __MAX(((rect_r*)r1)->w, ((rect_r*)r1)->h);

    if(diff == 0) {
        diff =
            __MIN(((rect_r*)r2)->w, ((rect_r*)r2)->h) - __MIN(((rect_r*)r1)->w, ((rect_r*)r1)->h);
    }

    return diff;

#undef __MAX
#undef __MIN
}

bnode* find_bin_tree(bnode* node, int w, int h) {
    if(node->used) {
        bnode* right = find_bin_tree(node->right, w, h);
        return right ? right : find_bin_tree(node->down, w, h);
    } else if((w <= node->w) && (h <= node->h)) {
        return node;
    } else {
        return NULL;
    }
}

bnode* split_bin_tree(bnode* node, int w, int h) {
    node->used = true;
    node->down = create_bnode(node->x, node->y + h, node->w, node->h - h);
    node->right = create_bnode(node->x + w, node->y, node->w - w, h);
    return node;
}

bnode* grow_right(bnode* root, rect_r* r) {
    bnode* old = create_bnode_empty();
    *old = *root;

    root->used = true;
    root->x = 0;
    root->y = 0;
    root->w = old->w + r->w;
    root->h = old->h;
    root->down = old;
    root->right = create_bnode(old->w, 0, r->w, old->h);

    bnode* node = find_bin_tree(root, r->w, r->h);
    if(node) {
        return split_bin_tree(node, r->w, r->h);
    } else {
        return NULL;
    }
}

bnode* grow_down(bnode* root, rect_r* r) {
    bnode* old = create_bnode_empty();
    *old = *root;

    root->used = true;
    root->x = 0;
    root->y = 0;
    root->w = old->w;
    root->h = old->h + r->h;
    root->down = create_bnode(0, old->h, old->w, r->h);
    root->right = old;

    bnode* node = find_bin_tree(root, r->w, r->h);
    if (node) {
        return split_bin_tree(node, r->w, r->h);
    } else {
        return NULL;
    }
}

bnode* grow_bin_tree(bnode* root, rect_r* r, int max_w, int max_h) {
    bool can_grow_down = (r->w <= root->w) && ((r->h + root->h) <= max_h);
    bool can_grow_right = (r->h <= root->h) && ((r->w + root->w) <= max_w);

    bool should_grow_right = can_grow_right && (root->h >= (root->w + r->w));
    bool should_grow_down = can_grow_down && (root->w >= (root->h + r->h));

    if(should_grow_right) {
        return grow_right(root, r);
    } else if(should_grow_down) {
        return grow_down(root, r);
    }

    if(can_grow_right) {
        return grow_right(root, r);
    } else if (can_grow_down) {
        return grow_down(root, r);
    }

    return NULL;
}


pack_res pack_bin_tree(pack_ctx* ctx) {
    pack_res res;
    res.all_fit = true;
    res.none_fit = true;

    rect_r* r = ctx->r;

    int root_w = r[ctx->next].w;
    int root_h = r[ctx->next].h;

    bnode* root = create_bnode(0, 0, 
                                root_w <= ctx->max_w ? root_w : ctx->max_w,
                                root_h <= ctx->max_h ? root_h : ctx->max_h);

    bool contiguous = true;
    int last = ctx->last;

    for(int i = ctx->next; i <= ctx->last; i++) {
        
        if(!r[i].info.packed) {
            bnode* node = find_bin_tree(root, r[i].w, r[i].h);

            if(node) {
                r[i].info.x = node->x;
                r[i].info.y = node->y;
                r[i].info.packed = true;
                r[i].info.page = ctx->page;

                split_bin_tree(node, r[i].w, r[i].h);
                res.none_fit = false;

            } else {
                bnode* expanded = grow_bin_tree(root, &r[i], ctx->max_w, ctx->max_h);
                if(expanded) {
                    r[i].info.x = expanded->x;
                    r[i].info.y = expanded->y;
                    r[i].info.packed = true;
                    r[i].info.page = ctx->page;
                    res.none_fit = false;
                } else {
                    r[i].info.packed = false;
                    res.all_fit = false;
                    contiguous = false;
                    last = i;
                }
            }
        }

        if(contiguous) {
            ctx->next = i + 1;
        }
    }

    ctx->last = last;

    destroy_bnode(root);
    return res;
}

bool rect_pack(int max_w, int max_h, bool paging, rect_r* rects, int rects_size) {
    if(rects_size == 0) {
        return true;
    }

    // Sort nodes by the max side
    qsort(rects, rects_size, sizeof(rect_r), compare_rect_r_max_side);

    // building context variable
    pack_ctx ctx;
    ctx.max_w = max_w;
    ctx.max_h = max_h;
    ctx.r = rects;
    ctx.n = rects_size;
    ctx.page = 0;
    ctx.next = 0;
    ctx.last = rects_size - 1;

    bool ok = false;
    bool all_packed = false;

    for(int i = 0; i < ctx.n; i++) {
        rects[i].info = empty_rect_r();
    }

    while(!ok) {
        pack_res res = pack_bin_tree(&ctx);
        ok = res.all_fit;
        all_packed = all_packed || ok;

        if(!paging || res.none_fit) {
            break;
        }

        ctx.page++;
    }

    return all_packed;
}

#endif