#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>
typedef struct{
    int point;
    int len;
}pair;
int compare(const void *a,const void *b){
    pair x=*(pair*)a;
    pair y=*(pair*)b;
    int p=x.point;
    int q=y.point;
    if(p>q){
            return 1;
        }else{
            return -1;
        }
}
int max(int a,int b){
    return a>b?a:b;
}
int main()
{   int n;
    scanf("%d",&n);
    pair a[n];
    for(int i=0;i<n;i++){
        scanf("%d%d",&a[i].point,&a[i].len);
    }
    qsort(a,n,sizeof(pair),compare);
    /*for(int i=0;i<n;i++){
        printf("%d %d\n",a[i].point,a[i].len);
    }*/
    int ans=0,reach=1;
    for(int i=0;i<n-1;i+=reach){
        if(i+reach>=n){
            ans++;
            break;
        }
        if((a[i].point+a[i].len-1)<(a[i+reach].point-a[i+reach].len+1)){
            ans++;
            reach=1;
            //printf("ok%d",i);
            if(i==n-2){ans++;}
        }else{
            if(a[i].point+a[i].len<=a[i+reach].point+a[i+reach].len){
           // printf("pass");
            i-=reach;
            reach++;}else{
                i+=reach-1;
                reach=1;
            }
        }
    }if(ans==0)ans++;
    printf("%d",ans);
    
    return 0;
}
