#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <time.h>

#define N 40
#define MAX 10000
#define MIN(a,b) (a<b?a:b)

typedef struct{
    int p;
    int cmp;
    int point[100];
    int sum;
    int ave;
    int mask;   // 1:True, 0:False
} problem;

// 降順ソート
int cmp(const void *a, const void *b) {
    return ((problem*)b)->ave - ((problem*)a)->ave;
}

int minimum(int goal, int d, int score, int num, problem a[])
{
    int i, j;
    int subsum[10] = {0};
    int count[10] = {0};
    int min_count=100;
    for(i=0; i<d; i++) {
        if(a[i].mask==1) {
            for(j=0; j<a[i].p; j++) {
                subsum[i] += a[i].point[j];
                count[i]++;
                if(subsum[i] >= goal-score) {
                    min_count = MIN(min_count, count[i]);
                }
            }
        }
    }
    return min_count;
}

int minimize(int goal, int d, int score, int num, problem a[])
{
    int i;

    for(i=0; i<d; i++) {
        if(a[i].sum >= goal-score && a[i].mask==1) {
            return minimum(goal, d, score, num, a);
        }
    }
    // if(any sum < goal-score)
    for(i=0; a[i].mask==0; i++);
    a[i].mask = 0;
    score += a[i].sum;
    num += a[i].p;
    return a[i].p + minimize(goal, d, score, num, a);
}

int main()
{
    int d, g;
    int i, j;
    problem a[10];

    scanf("%d %d",&d, &g);
    for(i=0; i<d; i++) {
        scanf("%d %d",&a[i].p,&a[i].cmp);
        a[i].sum = 0;
        a[i].mask = 1;
        for(j=0; j<a[i].p; j++) {
            a[i].point[j] = 100*(i+1);
            a[i].sum += 100*(i+1);
        }
        a[i].point[a[i].p-1] += a[i].cmp;
        a[i].sum += a[i].cmp;
        a[i].ave = a[i].sum / a[i].p;
    }
    qsort(a,d,sizeof(problem),cmp);
    /*
    for(i=0; i<d; i++) {
        printf("a[%d].sum = %d, a[%d].ave = %d\n",i,a[i].sum,i,a[i].ave);
    }
    */
    int ans = minimize(g, d, 0, 0, a);

    printf("%d",ans);



    return 0;
}
