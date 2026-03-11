#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <limits.h>

typedef struct {
    int x;
    int y;
} squr_t;

int sort(const void *a, const void *b){
    if ( ((squr_t*)a)->x - ((squr_t*)b)->x >0 ) {
        return 1;
    }
    if ( ((squr_t*)a)->x - ((squr_t*)b)->x <0 ) {
        return -1;
    }
    if ( ((squr_t*)a)->x - ((squr_t*)b)->x ==0 ) {
        if ( ((squr_t*)a)->y - ((squr_t*)b)->y >=0 ) {
            return 1;
        }
        if ( ((squr_t*)a)->y - ((squr_t*)b)->y <0 ) {
            return -1;
        }
    }
    return 0;
}

int main(void){
    int h,w,n,a,b,count=0;
    scanf("%d%d%d",&h,&w,&n);
    squr_t s[n*9];
    for (int i=0; i<n; i++){
        scanf("%d%d",&a,&b);
        a--;
        b--;
        for (int j=-1; j<2; j++){
            if(a+j>=1 && a+j<h-1){
                for (int k=-1; k<2; k++){
                    if (b+k>=1 && b+k<w-1){
                        s[count].x=a+j;
                        s[count].y=b+k;
                        count++;
                    }
                }
            }
        }
    }
    qsort (s,count,sizeof(squr_t),sort);
    long long ans[10],am=0;
    for (int i=0; i<10; i++){
        ans[i]=0;
    }
    for (int i=0; i<count; i++){
        if (i==0){
            am++;
        } else if (s[i].x==s[i-1].x && s[i].y==s[i-1].y){
            am++;
        } else {
            ans[am]++;
            am=1;
        }
        if (i==count-1){
            ans[am]++;
        }
    }
    ans[0]=(h-2)*(w-2);
    for (int i=1; i<10; i++){
        ans[0]-=ans[i];
    }
    for (int i=0; i<10; i++){
        printf("%lld\n",ans[i]);
    }

    
	return 0 ;
}