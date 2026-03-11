#include <stdio.h>
#include <stdlib.h>

typedef struct data { int solved, penalty, id, wa[11]; } data;

int comp(const void* a, const void* b)
{
    data x=*((data*)a), y=*((data*)b);
    if (x.solved>y.solved) return -1;
    if (x.solved<y.solved) return 1;
    if (x.penalty<y.penalty) return -1;
    if (x.penalty>y.penalty) return 1;
    if (x.id>y.id) return -1;
    if (x.id<y.id) return 1;
    return 0;
}

int comp2(data* a, data* b)
{
    data x=*a, y=*b;
    if (x.solved>y.solved) return -1;
    if (x.solved<y.solved) return 1;
    if (x.penalty<y.penalty) return -1;
    if (x.penalty>y.penalty) return 1;
    return 0;
}


int main(void)
{
    data a[52];
    int M, T, P, R;
    while (scanf("%d%d%d%d", &M, &T, &P, &R), M) {
        int i, j;
        for(i=0;i<T;++i) {
            a[i].solved=a[i].penalty=0;
            a[i].id=i+1;
            for(j=0;j<P;++j) a[i].wa[j]=0;
        }
        while (R--) {
            int m, t, p;
            scanf("%d%d%d%d", &m, &t, &p, &j);
            --t, --p;
            if (j==0) {
                a[t].solved++;
                a[t].penalty+=m+a[t].wa[p]*20;
            }
            else {
                a[t].wa[p]++;
            }
        }
        qsort(a, T, sizeof(data), comp);
        printf("%d", a[0].id);
        for(i=1;i<T;++i)
            printf("%c%d", comp2(a+i-1, a+i)==0?'=':',', a[i].id);
        puts("");
    }
    return 0;
}