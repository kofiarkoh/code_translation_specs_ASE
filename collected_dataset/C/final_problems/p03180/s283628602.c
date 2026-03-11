#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdint.h> // uint64_t

#define max(a,b) ((a) > (b) ? (a) : (b))
#define min(a,b) ((a) > (b) ? (b) : (a))

int get_int(void) {
  int num;
  scanf("%d", &num);
  return num;
}

// <arr[0]> <arr[1]> .. <arr[size-1]>
int fget_array(int *arr, int size) {
    int i;
    for(i = 0; i < size; i++) {
        scanf("%d", &arr[i]);
    }
    return 0;
}

#define NUM_MAX 16
#define PAT_MAX (1<<16)

struct affinity{
    int pat;
    int64_t point;
};

int desc_by_point(const void *a1, const void *a2) {
    struct affinity *af1 = (struct affinity*)a1;
    struct affinity *af2 = (struct affinity*)a2;
    if(af1->point < af2->point) {
        return 1;
    } else if (af1->point == af2->point) return 0;
    return -1;
}

int main(void) {
    int num = get_int();
    static int rels[NUM_MAX][NUM_MAX];
    int i, j;
    for(i = 0; i < num; i++) {
        fget_array(rels[i], num);
    }
    static struct affinity groups[PAT_MAX];
    int pat, bit;
    for(pat = 1; pat < (1<<num); pat++) {
        int labels[NUM_MAX] = {0};
        int idx = 0;
        for(bit = 0; bit < num; bit++) {
            if(!(pat & (1<<bit))) continue;
            labels[idx++] = bit;
        }
        int64_t res = 0;
        for(i = 0; i < idx; i++) {
            for(j = i+1; j < idx; j++) {
                res += rels[labels[i]][labels[j]];
            }
        }
        groups[pat].point = res;
        groups[pat].pat = pat;
    }
    qsort(groups, (1<<num), sizeof(struct affinity), desc_by_point);

    static int64_t ans[PAT_MAX];
    static int pats[PAT_MAX];
    int idx = 0;
    pats[idx++] = 0;
    for(i = 0; i < (1<<num); i++) {
        struct affinity g = groups[i];
        int cur_idx = idx;
        if(g.point < 0) break;
        for(j = 0; j < cur_idx; j++) {
            if(pats[j] & g.pat) continue;
            int newpat = pats[j] | g.pat;
            if(!ans[newpat]) {
                pats[idx++] = newpat;
            }
            ans[newpat] = max(ans[newpat], ans[pats[j]] + g.point);
        }
    }
    printf("%lld\n", ans[(1<<num)-1]);
    return 0;
}