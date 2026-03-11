#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int a;
    int no;
    int order;
} DATA;

DATA a[300000];

int comp1(const void *a, const void *b)
{
    return ((*(DATA *)a).a - (*(DATA *)b).a);
}

int comp2(const void *a, const void *b)
{
    return ((*(DATA *)a).no - (*(DATA *)b).no);
}

int main(void)
{
    int N, Q, x, y;
    int diff;
    int i;
    int out;
    DATA tmp;
    int sort_flag;
    
    
    scanf("%d", &N);
    for (i = 0; i < N; i++){
        scanf("%d", &a[i].a);
        a[i].no = i;
    }
    
    qsort(a, N, sizeof(DATA), comp1);
    for (i = 0; i < N; i++){
        a[i].order = i;
    }
    
    qsort(a, N, sizeof(DATA), comp2);
    
//for (i = 0; i < N; i++){
//    printf("%d:%d:%d ", a[i].no, a[i].a, a[i].order);
//}
//printf("\n");
    
    diff = 0;
    for (i = 0; i < N; i++){
        if (a[i].no != a[i].order){
            diff++;
        }
    }

    sort_flag = 0;
    for (i = 0; i < N - 1; i++){
        if (a[i].a > a[i + 1].a){
            sort_flag = 1;
            break;
        }
    }
    
    if (sort_flag == 0){
        out = 0;
    }
    else {
        out = -1;
    }

    scanf("%d", &Q);
    for (i = 0; i < Q; i++){
        scanf("%d%d", &x, &y);
        x--;
        y--;

        if (out != -1){
            continue;
        }
        
        if (x == a[y].order){
            diff--;
        }
        if (y == a[x].order){
            diff--;
        }
        if (x == a[x].order){
            diff++;
        }
        if (y == a[y].order){
            diff++;
        }
        
        tmp = a[x];
        a[x] = a[y];
        a[y] = tmp;
        
        if (diff == 0){
            out = i + 1;
        }
    }
    
    printf("%d\n", out);
    
    return (0);
}
