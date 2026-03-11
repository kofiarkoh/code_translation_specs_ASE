#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <ctype.h>
#include <stdint.h>
#include <string.h>
#include <wchar.h>
#include <math.h>

#define N_MAX (100)
#define P_MAX (100)
#define DP_ARRAY_SIZE (N_MAX * P_MAX / 32 + 1)

#define MIN(a, b) ((a) < (b) ? (a) : (b))
#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define ABS(a) ((a) < 0 ? -(a) : (a))
#define ABSS(a, b) ((a) > (b) ? (a) - (b) : (b) - (a))
#define SWAP(type, a, b) { const type tmp = a; a = b; b = tmp; }

int compare_sz_asc(const void* a, const void* b) {
    return *((size_t*)a) < *((size_t*)b) ? -1 : 1;
}

int compare_sz_desc(const void* a, const void* b) {
    return *((size_t*)a) > * ((size_t*)b) ? -1 : 1;
}

int compare_i64_asc(const void* a, const void* b) {
    return *((int64_t*)a) < *((int64_t*)b) ? -1 : 1;
}

int compare_i64_desc(const void* a, const void* b) {
    return *((int64_t*)a) > * ((int64_t*)b) ? -1 : 1;
}

int compare_u64_asc(const void* a, const void* b) {
    return *((uint64_t*)a) < *((uint64_t*)b) ? -1 : 1;
}

int compare_u64_desc(const void* a, const void* b) {
    return *((uint64_t*)a) > * ((uint64_t*)b) ? -1 : 1;
}

int compare_c_asc(const void* a, const void* b) {
    return *((char*)a) < *((char*)b) ? -1 : 1;
}

int compare_c_desc(const void* a, const void* b) {
    return *((char*)a) > * ((char*)b) ? -1 : 1;
}

int compare_dp_asc(const void* a, const void* b) {
    return *((double*)a) < *((double*)b) ? -1 : 1;
}

static uint64_t modinvU64(const uint64_t a, const uint64_t m) {
    int64_t aa = (int64_t)a;
    const int64_t mm = (int64_t)m;
    int64_t b = mm;
    int64_t u = 1;
    int64_t v = 0;

    while (b != 0) {
        const int64_t t = aa / b;

        aa -= t * b;
        SWAP(uint64_t, aa, b);
        u -= t * v;
        SWAP(uint64_t, u, v);
    }

    u %= mm;
    if (u < 0) {
        u += m;
    }

    return (uint64_t)u;
}

static size_t powSz(const size_t base, const size_t exp) {
    if (exp == 0) {
        return 1;
    }

    if (exp == 1) {
        return base;
    }

    if (exp % 2 == 0) {
        return powSz(base * base, exp / 2);
    }
    else {
        return base * powSz(base, exp - 1);
    }
}

static size_t comb(const size_t n, const size_t r) {
    size_t result = 1;

    for (size_t i = 0; i < r; i++) {
        result *= n - i;
        result /= i + 1;
    }

    return result;
}

static uint64_t combU64(const uint64_t n, const uint64_t r) {
    uint64_t result = 1;

    for (uint64_t i = 0; i < r; i++) {
        result *= n - i;
        result /= i + 1;
    }

    return result;
}

static uint64_t combU64WMod(const uint64_t n, const uint64_t r, const uint64_t m) {
    uint64_t result = 1;

    for (uint64_t i = 0; i < r; i++) {
        result = result * (n - i) % m;
        result = result * modinvU64(i + 1, m) % m;
    }

    return result;
}

static size_t gcdZu(size_t m, size_t n) {
    size_t temp;

    while (m % n != 0) {
        temp = n;
        n = m % n;
        m = temp;
    }

    return n;
}

static uint64_t gcdU64(uint64_t m, uint64_t n)
{
    uint64_t temp;

    while (m % n != 0) {
        temp = n;
        n = m % n;
        m = temp;
    }

    return n;
}

typedef struct {
    uint64_t val;
}BHeap_Val;

#define BHEAP_VAL_TYPE BHeap_Val

#define BHEAP_INVALID_NODE ((size_t)100000000)

typedef struct {
    BHEAP_VAL_TYPE val;
} BHeap_Node;


struct _BHeap_Ctx {
    size_t size;
    int (*compare)(const BHEAP_VAL_TYPE* parent, const BHEAP_VAL_TYPE* node);
    void(*notifyNodeChange)(const struct _BHeap_Ctx* const ctx, const size_t nodeListSize, const size_t* const nodeList);
    BHeap_Node* node;
};

typedef struct _BHeap_Ctx BHeap_Ctx;

static void bHeap_Init(BHeap_Ctx *const ctx, int(*compare)(const BHEAP_VAL_TYPE* parent, const BHEAP_VAL_TYPE* node), void(*notifyNodeChange)(const BHeap_Ctx * const ctx, const size_t nodeListSize, const size_t* const nodeList), BHeap_Node*const nodes) {
    ctx->size = 0;
    ctx->compare = compare;
    ctx->notifyNodeChange = notifyNodeChange;
    ctx->node = nodes;
}

static void bHeap_Swap(BHeap_Ctx* const ctx, const size_t node0, const size_t node1) {
    const BHEAP_VAL_TYPE tmp = ctx->node[node0].val;
    ctx->node[node0].val = ctx->node[node1].val;
    ctx->node[node1].val = tmp;

    if (ctx->notifyNodeChange != NULL) {
        const size_t nodeList[2] = { node0,node1 };
        ctx->notifyNodeChange(ctx, 2, nodeList);
    }
}

static void bHeap_UpHeap(BHeap_Ctx* const ctx, const size_t nodeIdx) {
    if (nodeIdx == 0) {
        return;
    }

    const size_t parentIdx = (nodeIdx + 1) / 2 - 1;

    const int compareResult = ctx->compare(&(ctx->node[parentIdx].val), &(ctx->node[nodeIdx].val));

    if (0 < compareResult) {
        bHeap_Swap(ctx, nodeIdx, parentIdx);

        if (parentIdx != 0) {
            bHeap_UpHeap(ctx, parentIdx);
        }
    }
}

static void bHeap_DownHeap(BHeap_Ctx* const ctx, const size_t nodeIdx) {
    if (ctx->size - 1 <= nodeIdx) {
        return;
    }

    const size_t leftIdx = 2 * (nodeIdx + 1) - 1;

    if (leftIdx < ctx->size) {
        const int leftResult = ctx->compare(&(ctx->node[nodeIdx].val), &(ctx->node[leftIdx].val));

        size_t swapTarget = leftResult < 0 ? BHEAP_INVALID_NODE : leftIdx;

        const size_t rightIdx = leftIdx + 1;

        if (rightIdx < ctx->size) {
            const int rightResult = ctx->compare(&(ctx->node[nodeIdx].val), &(ctx->node[rightIdx].val));

            if (swapTarget == BHEAP_INVALID_NODE) {
                if (0 < rightResult) {
                    swapTarget = rightIdx;
                }
            }
            else {
                if (0 < rightResult) {
                    const int rightLeftResult = ctx->compare(&(ctx->node[leftIdx].val), &(ctx->node[rightIdx].val));

                    if (0 < rightLeftResult) {
                        swapTarget = rightIdx;
                    }
                }
            }
        }

        if (swapTarget != BHEAP_INVALID_NODE) {
            bHeap_Swap(ctx, nodeIdx, swapTarget);

            bHeap_DownHeap(ctx, swapTarget);
        }
    }
}

static void bHeap_Add(BHeap_Ctx * const ctx, const BHEAP_VAL_TYPE val) {
    ctx->node[ctx->size].val = val;

    ctx->size++;

    if (ctx->notifyNodeChange != NULL) {
        const size_t node = ctx->size - 1;
        ctx->notifyNodeChange(ctx, 1, &node);
    }

    bHeap_UpHeap(ctx, ctx->size - 1);
}

static void bHeap_RemoveTop(BHeap_Ctx * const ctx) {
    if (ctx->size == 0) {
        return;
    }

    if (ctx->size == 1) {
        ctx->size--;
        return;
    }

    ctx->node[0].val = ctx->node[ctx->size - 1].val;

    if (ctx->notifyNodeChange != NULL) {
        const size_t node = 0;
        ctx->notifyNodeChange(ctx, 1, &node);
    }

    ctx->size--;

    bHeap_DownHeap(ctx, 0);
}

static void bHeap_RemoveNode(BHeap_Ctx *const ctx, const size_t node) {
    if (node == ctx->size - 1) {
        ctx->size--;
    }
    else {
        ctx->node[node].val = ctx->node[ctx->size - 1].val;

        ctx->size--;

        if (ctx->notifyNodeChange != NULL) {
            ctx->notifyNodeChange(ctx, 1, &node);
        }

        bHeap_DownHeap(ctx, node);
        bHeap_UpHeap(ctx, node);
    }
}

static BHEAP_VAL_TYPE* bHeap_GetTopVal(const BHeap_Ctx * const ctx) {
    return ctx->size == 0 ? NULL : &(ctx->node[0].val);
}

typedef struct {
    uint64_t cnt;
    size_t* child;
    size_t numChild;
    size_t maxNumChild;
} Tree_Node;

#define TREE_INIT_ALLOC (128)

static Tree_Node tree_Node[200000];

static void tree_Init(const uint64_t N) {
    for (uint64_t i = 0; i < N; i++) {
        tree_Node[i].cnt = 0;
        tree_Node[i].numChild = 0;
        tree_Node[i].child = malloc(sizeof(size_t) * TREE_INIT_ALLOC);
        tree_Node[i].maxNumChild = TREE_INIT_ALLOC;
    }
}

static void tree_AddChild(const size_t parent, const size_t child) {
    if (tree_Node[parent].maxNumChild <= tree_Node[parent].numChild) {
        tree_Node[parent].child = realloc(tree_Node[parent].child, sizeof(size_t) * tree_Node[parent].maxNumChild * 2);
        tree_Node[parent].maxNumChild *= 2;
    }

    tree_Node[parent].child[tree_Node[parent].numChild] = child;
    tree_Node[parent].numChild++;
}

static uint64_t P[200000];

static BHeap_Node BN_Asc[200000];
static BHeap_Node BN_Desc[200000];

static int _bHeap_Compare_Asc(const BHEAP_VAL_TYPE* parent, const BHEAP_VAL_TYPE* node) {
    return parent->val <= node->val ? -1 : 1;
}

static int _bHeap_Compare_Desc(const BHEAP_VAL_TYPE* parent, const BHEAP_VAL_TYPE* node) {
    return parent->val >= node->val ? -1 : 1;
}

static size_t PBHeap_Asc_Idx[200000];
static size_t PBHeap_Desc_Idx[200000];

static void _bHeap_NotifyNodeChange_Asc(const BHeap_Ctx* const ctx, const size_t nodeListSize, const size_t* const nodeList) {
    for (size_t i = 0; i < nodeListSize; i++) {
        PBHeap_Asc_Idx[ctx->node[nodeList[i]].val.val] = nodeList[i];
    }
}

static void _bHeap_NotifyNodeChange_Desc(const BHeap_Ctx* const ctx, const size_t nodeListSize, const size_t* const nodeList) {
    for (size_t i = 0; i < nodeListSize; i++) {
        PBHeap_Desc_Idx[ctx->node[nodeList[i]].val.val] = nodeList[i];
    }
}

int main(void) {
    uint64_t N, K;

    scanf("%"PRIu64"%"PRIu64, &N, &K);
    for (uint64_t i = 0; i < N; i++) {
        scanf("%"PRIu64, &P[i]);
    }

    size_t contLength = 1;
    uint64_t wideIncSecCnt = 0;
    for (uint64_t i = 1; i < N; i++) {
        if (P[i - 1] < P[i]) {
            contLength++;
        }
        else {
            if (K <= contLength) {
                wideIncSecCnt++;
            }

            contLength = 1;
        }

        if (i == N - 1 && K <= contLength) {
            wideIncSecCnt++;
        }
    }

    BHeap_Ctx ascCtx;
    BHeap_Ctx descCtx;

    bHeap_Init(&ascCtx, _bHeap_Compare_Asc, _bHeap_NotifyNodeChange_Asc, BN_Asc);
    bHeap_Init(&descCtx, _bHeap_Compare_Desc, _bHeap_NotifyNodeChange_Desc, BN_Desc);

    for (uint64_t i = 0; i < K; i++) {
        BHeap_Val val = { .val = P[i] };
        bHeap_Add(&ascCtx, val);
        bHeap_Add(&descCtx, val);
    }

    uint64_t cnt = 1;

    for (uint64_t i = 1; i < N - K + 1; i++) {
        bool cntAdded = false;

        if (bHeap_GetTopVal(&ascCtx)->val != P[i - 1]) {
            cnt++;

            cntAdded = true;
        }

        bHeap_RemoveNode(&ascCtx, PBHeap_Asc_Idx[P[i - 1]]);
        bHeap_RemoveNode(&descCtx, PBHeap_Desc_Idx[P[i - 1]]);

        BHeap_Val val = { .val = P[i + K - 1] };

        bHeap_Add(&ascCtx, val);
        bHeap_Add(&descCtx, val);

        if (bHeap_GetTopVal(&descCtx)->val != P[i + K -1] && !cntAdded) {
            cnt++;
        }
    }

    if (0 < wideIncSecCnt) {
        wideIncSecCnt--;
    }

    printf("%"PRIu64"\n", cnt - wideIncSecCnt);

    return 0;
}
