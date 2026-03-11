#include<stdio.h>
#include<math.h>
#define inf 1e10
#define min(x,y) x<y?x:y
double cal_time(int r, int h, int x){
    return h + r - sqrt(r*r-x*x);
}
int main(void){
    int r, n, left, right, h;
    while(1){
        int x[40]={0};
        scanf("%d%d",&r,&n);
        if(r==0 && n==0) break;
        for(int i=0; i<n; i++){
            scanf("%d%d%d",&left,&right,&h);
            for(int j=left; j<right; j++){
                if(x[20+j] < h) x[20+j]=h;
            }
        }
        double min=inf,tmp;
        for(int i=-r+1; i<r; i++){
            if((tmp=cal_time(r,min(x[i+19],x[i+20]),i)) < min){
                min=tmp;
            }
        }
        printf("%.4f\n",min);
    }
    return 0;
}

